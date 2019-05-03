use std::mem;
use std::ops::{Deref, DerefMut};

use mach::kern_return;

use super::{ffi, IoObject};

#[derive(Debug)]
pub struct IoIterator(ffi::io_iterator_t);

impl Deref for IoIterator {
    type Target = ffi::io_iterator_t;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IoIterator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Iterator for IoIterator {
    type Item = IoObject;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { ffi::IOIteratorNext(self.0) } {
            0 => None,  // TODO: Should not there be some `NULL` instead of `0`?
            io_object => Some(IoObject::from(io_object))
        }
    }
}

impl Drop for IoIterator {
    fn drop(&mut self) {
        let result = unsafe {
            ffi::IOObjectRelease(self.0)
        };
        assert_eq!(result, kern_return::KERN_SUCCESS);
    }
}

impl Default for IoIterator {
    // It is unsafe to just create an `IoIterator` instance via `default`
    // and do not initialize it later.
    // In addition, it needs to be initialized properly before the `Drop` call.
    fn default() -> IoIterator {
        let inner = unsafe {
            mem::zeroed()
        };
        IoIterator(inner)
    }
}
