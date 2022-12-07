#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod error;

use alloc::ffi::{CString, NulError};
use core::{ffi::CStr, mem::MaybeUninit};

use btrfsutil_sys as ffi;

pub use error::BtrfsError;

#[derive(Debug)]
pub enum Error {
    BtrfsError(BtrfsError),
    NulError(NulError),
    UnknownBtrfsError,
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Error::NulError(err)
    }
}

impl From<BtrfsError> for Error {
    fn from(err: BtrfsError) -> Self {
        Error::BtrfsError(err)
    }
}

impl From<Option<BtrfsError>> for Error {
    fn from(err: Option<BtrfsError>) -> Self {
        match err {
            Some(err) => Error::BtrfsError(err),
            None => Error::UnknownBtrfsError,
        }
    }
}

pub fn strerror(error: BtrfsError) -> Option<CString> {
    let ret = unsafe { ffi::btrfs_util_strerror(error.as_raw()) };
    if ret.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(ret) }.to_owned())
}

pub fn subvolume_info(path: &str, id: u64) -> Result<ffi::btrfs_util_subvolume_info, Error> {
    let path = CString::new(path.as_bytes())?;
    let mut info = MaybeUninit::zeroed();
    unsafe {
        match ffi::btrfs_util_subvolume_info(path.as_ptr().cast(), id, info.as_mut_ptr()) {
            ffi::BTRFS_UTIL_OK => Ok(info.assume_init()),
            ret => Err(BtrfsError::from_raw_unchecked(ret).into()),
        }
    }
}

pub struct SubvolumeIterator(*mut ffi::btrfs_util_subvolume_iterator);

impl SubvolumeIterator {
    pub fn new(path: &str, top: u64, flags: i32) -> Result<Self, Error> {
        let path = CString::new(path.as_bytes())?;
        let mut iter = MaybeUninit::zeroed();

        unsafe {
            match ffi::btrfs_util_create_subvolume_iterator(
                path.as_ptr().cast(),
                top,
                flags,
                iter.as_mut_ptr(),
            ) {
                ffi::BTRFS_UTIL_OK => Ok(Self(iter.assume_init())),
                ret => Err(BtrfsError::from_raw_unchecked(ret).into()),
            }
        }
    }
}

impl Iterator for SubvolumeIterator {
    type Item = Result<(ffi::btrfs_util_subvolume_info, Option<CString>), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut info = MaybeUninit::zeroed();
        let mut path = MaybeUninit::zeroed();

        unsafe {
            match ffi::btrfs_util_subvolume_iterator_next_info(
                self.0,
                path.as_mut_ptr(),
                info.as_mut_ptr(),
            ) {
                ffi::BTRFS_UTIL_OK => {
                    let path = path.assume_init();
                    let path = if path.is_null() {
                        None
                    } else {
                        let owned = CStr::from_ptr(path).to_owned();
                        libc::free(path.cast());
                        Some(owned)
                    };
                    Some(Ok((info.assume_init(), path)))
                }
                ffi::BTRFS_UTIL_ERROR_STOP_ITERATION => None,
                ret => Some(Err(BtrfsError::from_raw_unchecked(ret).into())),
            }
        }
    }
}

impl Drop for SubvolumeIterator {
    fn drop(&mut self) {
        unsafe {
            ffi::btrfs_util_destroy_subvolume_iterator(self.0);
        }
    }
}
