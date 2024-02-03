use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

thread_local! {
    static RUNTIME: Runtime = Runtime::new();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signal<T> {
    id: SignalId,
    _ty: PhantomData<*mut T>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Signal<T> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Effect {
    id: EffectId,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Memo<T> {
    signal: Signal<Option<T>>,
    effect: Effect,
}

impl<T> Clone for Memo<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Memo<T> {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalId {
    val: u64,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct EffectId {
    val: u64,
}

struct Runtime {
    scope: RefCell<Scope>,

    signals: RefCell<HashMap<SignalId, SignalNode>>,
    effects: RefCell<HashMap<EffectId, EffectNode>>,

    next_id: Cell<u64>,
}

struct Scope {
    current: Option<EffectId>,
    writeable: bool,
}

struct SignalNode {
    val: Box<RefCell<dyn Any>>,
    subs: RefCell<HashSet<EffectId>>,
}

struct EffectNode {
    callback: Box<RefCell<dyn FnMut()>>,
}

impl<T: 'static> Signal<T> {
    /// Create a new signal with given value.
    ///
    /// ```
    /// # use crossd::signal::Signal;
    /// #
    /// let count = Signal::new(0);
    /// ```
    pub fn new(val: T) -> Self {
        RUNTIME.with(|rt| {
            let id = SignalId { val: rt.next_id() };

            rt.signals.borrow_mut().insert(
                id,
                SignalNode { val: Box::new(RefCell::new(val)), subs: RefCell::default() },
            );

            Signal { id, _ty: PhantomData }
        })
    }

    /// Clone the value of the signal.
    ///
    /// ```
    /// # use crossd::signal::Signal;
    /// #
    /// let count = Signal::new(0);
    ///
    /// assert_eq!(count(), 0);
    ///
    /// count.set(count() + 1);
    ///
    /// assert_eq!(count(), 1);
    /// ```
    ///
    /// ## Panics
    ///
    /// (todo)
    pub fn get(self) -> T
    where
        T: Clone,
    {
        self.try_get().unwrap()
    }

    /// Attempts to retreive signal value.
    ///
    /// Returns `None` if (todo).
    pub fn try_get(self) -> Option<T>
    where
        T: Clone,
    {
        RUNTIME.with(|rt| {
            let val = rt
                .signals
                .borrow()
                .get(&self.id)
                .and_then(|node| {
                    if let Some(id) = rt.scope.borrow().current {
                        node.subs.try_borrow_mut().ok()?.insert(id);
                    }

                    node.val.try_borrow().ok()
                })
                .and_then(|any| any.downcast_ref().cloned());

            rt.run_effects(self.id);

            val
        })
    }

    /// Set the value of the signal.
    ///
    /// ## Panics
    ///
    /// (todo)
    pub fn set(self, val: T) {
        self.try_set(val).unwrap()
    }

    /// Attempts to set signal value.
    ///
    /// Returns `None` if (todo).
    pub fn try_set(self, val: T) -> Option<()> {
        RUNTIME.with(|rt| {
            if rt.scope.borrow().writeable {
                rt.signals.try_borrow().ok()?.get(&self.id).and_then(|node| {
                    Some(*node.val.try_borrow_mut().ok()?.downcast_mut()? = val)
                })
            } else {
                None
            }
        })
    }

    /// Set the value of the signal without checking for writeability.
    pub fn set_untracked(self, val: T) {
        self.try_set_untracked(val).unwrap()
    }

    /// Try to set the value of the signal without checking for writeability.
    pub fn try_set_untracked(self, val: T) -> Option<()> {
        RUNTIME.with(|rt| {
            rt.signals.try_borrow().ok()?.get(&self.id).and_then(|node| {
                Some(*node.val.try_borrow_mut().ok()?.downcast_mut()? = val)
            })
        })
    }

    pub fn with<O>(self, f: impl FnOnce(&T) -> O) -> O {
        self.try_with(f).unwrap()
    }

    pub fn try_with<O>(self, f: impl FnOnce(&T) -> O) -> Option<O> {
        RUNTIME.with(|rt| {
            rt.signals
                .try_borrow()
                .ok()?
                .get(&self.id)
                .and_then(|node| node.val.try_borrow().ok()?.downcast_ref().map(f))
        })
    }

    pub fn update<O>(self, f: impl FnOnce(&mut T) -> O) -> O {
        self.try_update(f).unwrap()
    }

    pub fn try_update<O>(self, f: impl FnOnce(&mut T) -> O) -> Option<O> {
        RUNTIME.with(|rt| {
            rt.signals
                .try_borrow()
                .ok()?
                .get(&self.id)
                .and_then(|node| node.val.try_borrow_mut().ok()?.downcast_mut().map(f))
        })
    }
}

impl<T: Clone + 'static> FnOnce<()> for Signal<T> {
    type Output = T;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl Effect {
    /// Create a reactive effect.
    ///
    /// ```
    /// # use crossd::signal::{Effect, Signal};
    /// #
    /// let num = Signal::new(1);
    ///
    /// Effect::new(move || println!("num has changed to {}", num.get()));
    /// ```
    ///
    /// ## Panics
    ///
    /// Panics if the provided function mutates signals.
    pub fn new(f: impl FnMut() + 'static) -> Self {
        RUNTIME.with(|rt| {
            let id = EffectId { val: rt.next_id() };

            rt.effects
                .borrow_mut()
                .insert(id, EffectNode { callback: Box::new(RefCell::new(f)) });

            Self { id }
        })
    }
}

impl<T: 'static> Memo<T> {
    /// ```
    /// # use crossd::signal::{Memo, Signal};
    /// #
    /// // ax^2 + bx + c
    /// let a = Signal::new(2.0f32);
    /// let b = Signal::new(3.0f32);
    /// let c = Signal::new(1.0f32);
    ///
    /// let discriminant = Memo::new(move || b().powf(2.0) - 4.0 * a() * c());
    /// let real_zeros = Memo::new(move || {
    ///     if discriminant() < 0.0 {
    ///         0
    ///     } else if discriminant() == 0.0 {
    ///         1
    ///     } else {
    ///         2
    ///     }
    /// });
    /// ```
    ///
    /// ## Panics
    ///
    /// Panics if signals are written to in the callback.
    pub fn new(mut f: impl FnMut() -> T + 'static) -> Self {
        let signal = Signal::new(None);
        let effect = Effect::new(move || {
            signal.set_untracked(Some(f()));
        });

        Self { signal, effect }
    }

    pub fn get(self) -> T
    where
        T: Clone,
    {
        self.try_get().unwrap()
    }

    pub fn try_get(self) -> Option<T>
    where
        T: Clone,
    {
        self.signal.try_get().flatten()
    }
}

impl<T: Clone + 'static> FnOnce<()> for Memo<T> {
    type Output = T;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.get()
    }
}

impl Runtime {
    fn new() -> Self {
        let scope = RefCell::new(Scope { current: None, writeable: true });

        let signals = RefCell::default();
        let effects = RefCell::default();

        let next_id = Cell::default();

        Self { scope, signals, effects, next_id }
    }

    fn next_id(&self) -> u64 {
        self.next_id.set(self.next_id.get() + 1);

        self.next_id.get()
    }

    fn run_effects(&self, id: SignalId) {
        for effect in self.signals.borrow().get(&id).unwrap().subs.borrow().iter() {
            self.effects.borrow_mut().get(&effect).unwrap().callback.borrow_mut()();
        }
    }
}
