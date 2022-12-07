use alloc::fmt;
use core::mem;

use btrfsutil_sys as ffi;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum BtrfsError {
    StopIteration = ffi::BTRFS_UTIL_ERROR_STOP_ITERATION,
    NoMemory = ffi::BTRFS_UTIL_ERROR_NO_MEMORY,
    InvalidArgument = ffi::BTRFS_UTIL_ERROR_INVALID_ARGUMENT,
    NotBtrfs = ffi::BTRFS_UTIL_ERROR_NOT_BTRFS,
    NotSubvolume = ffi::BTRFS_UTIL_ERROR_NOT_SUBVOLUME,
    SubvolumeNotFound = ffi::BTRFS_UTIL_ERROR_SUBVOLUME_NOT_FOUND,
    OpenFailed = ffi::BTRFS_UTIL_ERROR_OPEN_FAILED,
    RmdirFailed = ffi::BTRFS_UTIL_ERROR_RMDIR_FAILED,
    UnlinkFailed = ffi::BTRFS_UTIL_ERROR_UNLINK_FAILED,
    StatFailed = ffi::BTRFS_UTIL_ERROR_STAT_FAILED,
    StatfsFailed = ffi::BTRFS_UTIL_ERROR_STATFS_FAILED,
    SearchFailed = ffi::BTRFS_UTIL_ERROR_SEARCH_FAILED,
    InoLookupFailed = ffi::BTRFS_UTIL_ERROR_INO_LOOKUP_FAILED,
    SubvolGetflagsFailed = ffi::BTRFS_UTIL_ERROR_SUBVOL_GETFLAGS_FAILED,
    SubvolSetflagsFailed = ffi::BTRFS_UTIL_ERROR_SUBVOL_SETFLAGS_FAILED,
    SubvolCreateFailed = ffi::BTRFS_UTIL_ERROR_SUBVOL_CREATE_FAILED,
    SnapCreateFailed = ffi::BTRFS_UTIL_ERROR_SNAP_CREATE_FAILED,
    SnapDestroyFailed = ffi::BTRFS_UTIL_ERROR_SNAP_DESTROY_FAILED,
    DefaultSubvolFailed = ffi::BTRFS_UTIL_ERROR_DEFAULT_SUBVOL_FAILED,
    SyncFailed = ffi::BTRFS_UTIL_ERROR_SYNC_FAILED,
    StartSyncFailed = ffi::BTRFS_UTIL_ERROR_START_SYNC_FAILED,
    WaitSyncFailed = ffi::BTRFS_UTIL_ERROR_WAIT_SYNC_FAILED,
    GetSubvolInfoFailed = ffi::BTRFS_UTIL_ERROR_GET_SUBVOL_INFO_FAILED,
    GetSubvolRootrefFailed = ffi::BTRFS_UTIL_ERROR_GET_SUBVOL_ROOTREF_FAILED,
    InoLookupUserFailed = ffi::BTRFS_UTIL_ERROR_INO_LOOKUP_USER_FAILED,
    FsInfoFailed = ffi::BTRFS_UTIL_ERROR_FS_INFO_FAILED,
}

impl BtrfsError {
    pub fn from_raw(raw: ffi::btrfs_util_error) -> Option<Self> {
        use BtrfsError::*;
        Some(match raw {
            ffi::BTRFS_UTIL_ERROR_STOP_ITERATION => StopIteration,
            ffi::BTRFS_UTIL_ERROR_NO_MEMORY => NoMemory,
            ffi::BTRFS_UTIL_ERROR_INVALID_ARGUMENT => InvalidArgument,
            ffi::BTRFS_UTIL_ERROR_NOT_BTRFS => NotBtrfs,
            ffi::BTRFS_UTIL_ERROR_NOT_SUBVOLUME => NotSubvolume,
            ffi::BTRFS_UTIL_ERROR_SUBVOLUME_NOT_FOUND => SubvolumeNotFound,
            ffi::BTRFS_UTIL_ERROR_OPEN_FAILED => OpenFailed,
            ffi::BTRFS_UTIL_ERROR_RMDIR_FAILED => RmdirFailed,
            ffi::BTRFS_UTIL_ERROR_UNLINK_FAILED => UnlinkFailed,
            ffi::BTRFS_UTIL_ERROR_STAT_FAILED => StatFailed,
            ffi::BTRFS_UTIL_ERROR_STATFS_FAILED => StatfsFailed,
            ffi::BTRFS_UTIL_ERROR_SEARCH_FAILED => SearchFailed,
            ffi::BTRFS_UTIL_ERROR_INO_LOOKUP_FAILED => InoLookupFailed,
            ffi::BTRFS_UTIL_ERROR_SUBVOL_GETFLAGS_FAILED => SubvolGetflagsFailed,
            ffi::BTRFS_UTIL_ERROR_SUBVOL_SETFLAGS_FAILED => SubvolSetflagsFailed,
            ffi::BTRFS_UTIL_ERROR_SUBVOL_CREATE_FAILED => SubvolCreateFailed,
            ffi::BTRFS_UTIL_ERROR_SNAP_CREATE_FAILED => SnapCreateFailed,
            ffi::BTRFS_UTIL_ERROR_SNAP_DESTROY_FAILED => SnapDestroyFailed,
            ffi::BTRFS_UTIL_ERROR_DEFAULT_SUBVOL_FAILED => DefaultSubvolFailed,
            ffi::BTRFS_UTIL_ERROR_SYNC_FAILED => SyncFailed,
            ffi::BTRFS_UTIL_ERROR_START_SYNC_FAILED => StartSyncFailed,
            ffi::BTRFS_UTIL_ERROR_WAIT_SYNC_FAILED => WaitSyncFailed,
            ffi::BTRFS_UTIL_ERROR_GET_SUBVOL_INFO_FAILED => GetSubvolInfoFailed,
            ffi::BTRFS_UTIL_ERROR_GET_SUBVOL_ROOTREF_FAILED => GetSubvolRootrefFailed,
            ffi::BTRFS_UTIL_ERROR_INO_LOOKUP_USER_FAILED => InoLookupUserFailed,
            ffi::BTRFS_UTIL_ERROR_FS_INFO_FAILED => FsInfoFailed,
            _ => return None,
        })
    }

    pub unsafe fn from_raw_unchecked(raw: ffi::btrfs_util_error) -> Self {
        mem::transmute(raw)
    }

    pub fn as_raw(self) -> ffi::btrfs_util_error {
        unsafe { mem::transmute(self) }
    }
}

impl fmt::Display for BtrfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(error) = super::strerror(*self) else {
            return Err(fmt::Error);
        };
        write!(f, "{}", error.to_string_lossy())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BtrfsError {}
