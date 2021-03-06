use parking_lot::lock_api::{RawRwLock, RawMutex};
use std::cell::UnsafeCell;
use std::ptr::NonNull;
use parking_lot::lock_api::{Mutex, MutexGuard};

///
/// SAFETY: The contents of `data` should only
/// be accessed if `all_state` allows us to read
/// and `my_state` allows the adequate access.
///
/// `all_state` points to the parent `RawRwLock`
/// for the collection. Dropping the parent
/// collection should be impossible if there
/// still exists a reference to it, so `all_state`
/// should always be valid for the lifetime of
/// this struct, and `data` is only valid to be
/// read while `all_state` allows us to read.
///
pub struct SliceTracker<M: RawMutex, T> {
    pub(crate) data: NonNull<UnsafeCell<T>>,
    pub(crate) length: usize,
    pub(crate) lock: Mutex<M, ()>,
}

// SAFETY: T need not be Sync, since we check for
// T: Sync before we hand out any &T.
unsafe impl<M: Sync + RawMutex, T> Sync for SliceTracker<M, T> {}
// SAFETY: T need not be Send, since we check for
// T: Send before we hand out any &mut T.
unsafe impl<M: Send + Sync + RawMutex, T> Send for SliceTracker<M, T> {}

impl<M: RawMutex, T> SliceTracker<M, T> {
    /// SAFETY: `data`, and `length` must be valid
    /// and not overlap with any other `SliceTracker`s
    /// in the same collection.
    pub unsafe fn new(data: *const UnsafeCell<T>, length: usize) -> Self {
        Self {
            data: NonNull::new_unchecked(data as *mut _),
            length,
            lock: Default::default(),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, M, ()> {
        self.lock.lock()
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, M, ()>> {
        self.lock.try_lock()
    }
}

pub struct LockGuard<'a, R: RawRwLock> {
    pub(crate) rwlock: &'a R,
    pub(crate) kind: LockGuardType,
}

impl<'a, R: RawRwLock> LockGuard<'a, R> {
    pub fn lock(rwlock: &'a R, lock_type: LockGuardType) -> Self {
        match lock_type {
            LockGuardType::Read => rwlock.lock_shared(),
            LockGuardType::Write => rwlock.lock_exclusive(),
        }

        Self {
            rwlock,
            kind: lock_type,
        }
    }

    pub fn try_lock(rwlock: &'a R, lock_type: LockGuardType) -> Option<Self> {
        let succeeded = match lock_type {
            LockGuardType::Read => rwlock.try_lock_shared(),
            LockGuardType::Write => rwlock.try_lock_exclusive(),
        };
        if succeeded {
            Some(Self {
                rwlock,
                kind: lock_type,
            })
        } else {
            None
        }
    }
}

impl<'a, R: RawRwLock> Drop for LockGuard<'a, R> {
    fn drop(&mut self) {
        unsafe {
            match self.kind {
                LockGuardType::Read => self.rwlock.unlock_shared(),
                LockGuardType::Write => self.rwlock.unlock_exclusive(),
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub enum LockGuardType {
    Read,
    Write,
}
