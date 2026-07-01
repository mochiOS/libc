//! Common types used by most newlib platforms

#[allow(unused_imports)]
use crate::prelude::*;

s! {
    #[cfg(all(not(target_os = "vita"), not(target_os = "horizon")))]
    pub struct sigset_t {
        __val: u32,
    }

    #[cfg(all(not(target_os = "vita"), not(target_os = "horizon")))]
    pub struct stat {
        pub st_dev: super::dev_t,
        pub st_ino: super::ino_t,
        pub st_mode: super::mode_t,
        pub st_nlink: super::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: super::dev_t,
        pub st_size: super::off_t,
        pub st_atime: super::time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: super::time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: super::time_t,
        pub st_ctime_nsec: c_long,
        pub st_blksize: super::blksize_t,
        pub st_blocks: super::blkcnt_t,
        pub st_spare4: [c_long; 2usize],
    }

    #[cfg(not(target_os = "vita"))]
    pub struct dirent {
        pub d_ino: super::ino_t,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256usize],
    }
}
