use std::any::Any;
use std::cell::{Cell, RefCell};
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;

use slotmap::{DefaultKey, Key, KeyData, SlotMap};

thread_local! {
    static RUNTIME: Runtime = Runtime::new();
    static ID: fn() = || {};
}

/// The reactive runtime system data.
#[derive(Clone)]
pub struct Runtime {
    inner: Rc<Inner>,
}

struct Inner {
    values: RefCell<SlotMap<DefaultKey, Box<dyn Any>>>,
}

/// Drops the values in a [`Runtime`] when dropped.
pub struct RuntimeGuard {
    _marker: PhantomData<*const ()>,
}

impl Drop for RuntimeGuard {
    fn drop(&mut self) {
        Runtime::dispose()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Signal<T> {
    id: u64,

    _ty: PhantomData<T>,
    /// As [`Runtime`]s are thread-local, signals cannot be sent across threads.
    _marker: PhantomData<*const ()>,
}

pub struct Effect {}

impl Runtime {
    fn new() -> Self {
        Self { inner: Rc::new(Inner { values: RefCell::default() }) }
    }

    pub fn init() {
        RUNTIME.with(|rt| _ = &*rt);
    }

    pub fn dispose() {
        todo!()
    }

    fn signal<T: 'static>(&self, val: T) -> Signal<T> {
        let id = self.inner.values.borrow_mut().insert(Box::new(val)).data().as_ffi();

        Signal { id, _ty: PhantomData, _marker: PhantomData }
    }

    fn get<T: Clone + 'static>(&self, id: u64) -> T {
        self.inner
            .values
            .borrow()
            .get(key(id))
            .and_then(|any| any.downcast_ref::<T>())
            .unwrap()
            .clone()
    }

    fn set<T: 'static>(&self, id: u64, val: T) {
        *self
            .inner
            .values
            .borrow_mut()
            .get_mut(key(id))
            .and_then(|any| any.downcast_mut())
            .unwrap() = val;
    }
}

fn key(id: u64) -> DefaultKey {
    DefaultKey::from(KeyData::from_ffi(id))
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
    /// # use crossd::signal::Rt;
    /// #
    /// let count = rt.signal(0);
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

    /// Get the value of this signal.
    ///
    /// ```
    /// # use crossd::signal::Rt;
    /// #
    /// let count = rt.signal(0);
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
        RUNTIME.with(|rt| rt.set(self.id, val))
    }
}
