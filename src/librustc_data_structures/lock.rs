// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use parking_lot;

pub use std::sync::Arc as Rcd;

pub use parking_lot::RwLockReadGuard as ReadGuard;

pub struct RwLock<T>(parking_lot::RwLock<T>);

const ERROR_CHECKING: bool = false;

impl<T> RwLock<T> {
    pub fn new(inner: T) -> Self {
        RwLock(parking_lot::RwLock::new(inner))
    }

    pub fn borrow(&self) -> parking_lot::RwLockReadGuard<T> {
        if ERROR_CHECKING {
            self.0.try_read().expect("lock was already held")
        } else {
            self.0.read()
        }
    }

    pub fn borrow_mut(&self) -> parking_lot::RwLockWriteGuard<T> {
        if ERROR_CHECKING {
            self.0.try_write().expect("lock was already held")
        } else {
            self.0.write()
        }
    }
}

pub struct Lock<T>(parking_lot::Mutex<T>);

pub use parking_lot::MutexGuard as LockGuard;

impl<T> Lock<T> {
    pub fn new(inner: T) -> Self {
        Lock(parking_lot::Mutex::new(inner))
    }

    pub fn lock(&self) -> parking_lot::MutexGuard<T> {
        if ERROR_CHECKING {
            self.0.try_lock().expect("lock was already held")
        } else {
            self.0.lock()
        }
    }

    pub fn borrow(&self) -> parking_lot::MutexGuard<T> {
        self.lock()
    }

    pub fn borrow_mut(&self) -> parking_lot::MutexGuard<T> {
        self.lock()
    }
}
