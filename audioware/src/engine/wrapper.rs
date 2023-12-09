use std::{sync::{OnceLock, Arc, Mutex, MutexGuard}, borrow::BorrowMut};

pub struct OnceWrapper<T>(OnceLock<Arc<Mutex<T>>>);
impl<T> OnceWrapper<T> {
    pub fn set(&self, inner: T) -> anyhow::Result<()> {
        if let Ok(()) = self.0.set(Arc::new(Mutex::new(inner))) {
            return Ok(());
        }
        anyhow::bail!("set was called more than once");
    }
    pub fn try_call<O>(&self, method: impl FnOnce(MutexGuard<'_, T>) -> anyhow::Result<O>) -> anyhow::Result<O> {
        if let Ok(guard) = unsafe{self.0.get().unwrap_unchecked()}.clone().borrow_mut().try_lock() {
            return method(guard);
        }
        anyhow::bail!("unable to reach inner value");
    }
}
impl<T> Default for OnceWrapper<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}