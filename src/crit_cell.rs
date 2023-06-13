//! Cell based on the critical section.
//!
//! This cell is used for the safe access to the data that have to be declared as static. Access to
//! the internal value can be only done by performing a lock on the cell which enables critical
//! section for the duration.

use core::cell::UnsafeCell;

use cortex_m::interrupt;

/// Cell based on the critical section.
#[repr(transparent)]
pub(crate) struct CritCell<T: ?Sized>(UnsafeCell<T>);

/// CritCell is safe to share between threads because critical section prevents any access to the
/// data from other threads or interrupts. Value cannot be borrowed outside of the
/// critical section.
unsafe impl<T: Send + ?Sized> Sync for CritCell<T> {}

impl<T> CritCell<T> {
    /// Creates new cell with given value
    ///
    /// * `value` - Value to initialize the cell with.
    ///
    /// Returns new cell.
    #[inline(always)]
    pub(crate) const fn new(value: T) -> Self {
        CritCell(UnsafeCell::new(value))
    }
}

impl<T: ?Sized> CritCell<T> {
    /// Gives access to the value in critical section.
    ///
    /// This is the only access to the value. Given lambda is passed a mutable reference to the
    /// value and executed in critical section. This ensures that the value won't be borrowed more
    /// than once at the given time.
    ///
    /// * `f` - Lambda to execute.
    ///
    /// Returns the result of the executed lambda.
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        unsafe { interrupt::free(|_| f(self.as_mut_ref())) }
    }

    #[inline(always)]
    unsafe fn as_mut_ref(&self) -> &mut T {
        &mut *self.0.get()
    }
}

impl<T: Default> Default for CritCell<T> {
    fn default() -> CritCell<T> {
        CritCell::new(Default::default())
    }
}

impl<T> From<T> for CritCell<T> {
    fn from(t: T) -> CritCell<T> {
        CritCell::new(t)
    }
}
