use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

/// Wrapper of a data in thread-safe manner.
#[derive(Debug)]
pub struct Spinlock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

/// A guard to which the protected data can be accessed
///
/// When the guard falls out of scope it will release the lock.
#[derive(Debug)]
pub struct SpinlockGuard<'a, T: 'a> {
    lock: &'a AtomicBool,
    data: &'a mut T,
}

unsafe impl<T> Sync for Spinlock<T> {}

impl<T> Spinlock<T> {
    /// Create new SpinnLock wrapping the supplied data.
    pub const fn new(d: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(d),
        }
    }

    /// Block until it's unlocked.
    fn obtain_lock(&self) {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.is_locked() {
                core::hint::spin_loop();
            }
        }
    }

    /// Return true if the lock is currently held.
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }

    /// Locks the spinlock and return a guard.
    ///
    /// The returned value may be dereferenced for data access
    /// and the lock will be dropped when the guard falls out of scope.
    pub fn lock(&self) -> SpinlockGuard<T> {
        self.obtain_lock();

        SpinlockGuard {
            lock: &self.lock,
            data: unsafe { &mut *self.data.get() },
        }
    }
}

impl<'a, T> Deref for SpinlockGuard<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b Self::Target {
        &*self.data
    }
}

impl<'a, T> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut *self.data
    }
}

impl<'a, T> Drop for SpinlockGuard<'a, T> {
    /// The dropping of the SpinlockGuard will release the lock it was created from.
    fn drop(&mut self) {
        self.lock.store(false, Ordering::SeqCst);
    }
}
