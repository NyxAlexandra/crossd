use std::any::Any;
use std::cell::RefCell;
use std::hash::Hash;
use std::marker::PhantomData;
use std::{mem, ptr};

use slotmap::{new_key_type, SlotMap};

thread_local! {
    static RUNTIME: Runtime = Runtime::new();
}

/// The reactive runtime system data.
pub struct Runtime {
    values: RefCell<SlotMap<ValueId, Node>>,
}

struct Node {
    val: Box<RefCell<dyn Any>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signal<T> {
    id: ValueId,
    _ty: PhantomData<*mut T>,
}

new_key_type! {
    struct ValueId;
}

impl Runtime {
    fn new() -> Self {
        Self { values: RefCell::default() }
    }

    /// Ensure initialization of the reactive runtime.
    pub fn init() {
        RUNTIME.with(|rt| _ = &*rt);
    }

    /// Drop all current signal values, re-initializing the reactive runtime.
    /// It is safe to create new signals after this point.
    ///
    /// ## Safety
    ///
    /// All existing signals on the thread must be dropped or never used after
    /// this is called.
    pub unsafe fn dispose() {
        RUNTIME.with(|rt| {
            let mut new = Self::new();

            ptr::swap(&mut new, rt as *const _ as *mut Self);

            // old is dropped
        });
    }

    fn signal<T: 'static>(&self, val: T) -> Signal<T> {
        let id = self.values.borrow_mut().insert(Node::new(val));

        Signal { id, _ty: PhantomData }
    }

    fn get<T: Clone + 'static>(&self, id: ValueId) -> T {
        self.values.borrow().get(id).map(|node| node.get()).unwrap()
    }

    fn with<T: 'static, R>(&self, id: ValueId, f: impl FnOnce(&T) -> R) -> R {
        self.values.borrow().get(id).map(|node| node.with(f)).unwrap()
    }

    fn set<T: 'static>(&self, id: ValueId, val: T) {
        self.values.borrow().get(id).map(|node| node.set(val));
    }

    fn update<T: 'static>(&self, id: ValueId, f: impl FnOnce(&mut T)) {
        self.values.borrow().get(id).map(|node| node.update(f));
    }
}

impl Node {
    pub fn new<T: 'static>(val: T) -> Self {
        Self { val: Box::new(RefCell::new(val)) }
    }

    pub fn get<T: Clone + 'static>(&self) -> T {
        self.val.borrow().downcast_ref().cloned().unwrap()
    }

    pub fn with<T: 'static, R>(&self, f: impl FnOnce(&T) -> R) -> R {
        self.val.borrow().downcast_ref().map(f).unwrap()
    }

    pub fn set<T: 'static>(&self, val: T) {
        *self.val.borrow_mut().downcast_mut().unwrap() = val;
    }

    pub fn update<T: 'static, R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut self.val.borrow_mut().downcast_mut().unwrap())
    }
}

impl<T: 'static> Signal<T> {
    /// Create a new signal.
    ///
    /// ```
    /// let count = Signal::new(0);
    /// ```
    pub fn new(val: T) -> Self {
        RUNTIME.with(|rt| rt.signal(val))
    }

    /// Get the value of this signal.
    ///
    /// ```
    /// # use crossd::signal::Signal;
    /// #
    /// let count = Signal::new(0);
    ///
    /// assert_eq!(count.get(), 0);
    /// ```
    ///
    /// ## Panics
    ///
    /// Panics if the runtime of this signal is dropped.
    pub fn get(self) -> T
    where
        T: Clone,
    {
        RUNTIME.with(|rt| rt.get(self.id))
    }

    /// Run the callback on a reference to a signal.
    ///
    ///  ```
    /// # use crossd::signal::Signal;
    /// #
    /// let count = Signal::new(Point2::new(2, 4));
    ///
    ///
    ///  ```
    pub fn with<R>(self, f: impl FnOnce(&T) -> R) -> R {
        RUNTIME.with(|rt| rt.with(self.id, f))
    }

    /// Get the value of this signal.
    ///
    /// ```
    /// # use crossd::signal::Signal;
    /// #
    /// let count = Signal::new(0);
    ///
    /// count.set(3);
    ///
    /// assert_eq!(count.get(), 3);
    /// ```
    ///
    /// ## Panics
    ///
    /// Panics if the runtime of this signal is dropped.
    pub fn set(self, val: T) {
        RUNTIME.with(|rt| rt.set(self.id, val));
    }

    /// Mutate the value of the signal.
    pub fn update(self, f: impl FnOnce(&mut T)) {
        RUNTIME.with(|rt| rt.update(self.id, f));
    }
}
