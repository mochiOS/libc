use crate::prelude::*;

pub type clock_t = c_long;
pub type wchar_t = u32;

s! {
    pub struct sockaddr {
        pub sa_family: super::sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: super::sa_family_t,
        pub sin6_port: crate::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_in {
        pub sin_family: super::sa_family_t,
        pub sin_port: crate::in_port_t,
        pub sin_addr: crate::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_un {
        pub sun_family: super::sa_family_t,
        pub sun_path: [c_char; 108],
    }

    pub struct sockaddr_storage {
        pub ss_family: super::sa_family_t,
        __ss_padding: Padding<[u8; 126]>,
    }
}

pub const AF_UNIX: c_int = 1;
pub const AF_INET6: c_int = 23;
pub const FIONBIO: c_ulong = 1;
pub const SOMAXCONN: c_int = 128;
pub const RTLD_DEFAULT: *mut c_void = -2isize as *mut c_void;

pub const UTIME_OMIT: c_long = -1;
pub const AT_FDCWD: c_int = -2;
pub const AT_EACCESS: c_int = 1;
pub const AT_SYMLINK_NOFOLLOW: c_int = 2;
pub const AT_SYMLINK_FOLLOW: c_int = 4;
pub const AT_REMOVEDIR: c_int = 8;

pub const O_DIRECTORY: c_int = 0x200000;
pub const O_NOFOLLOW: c_int = 0x100000;

pub const POLLIN: c_short = 0x1;
pub const POLLPRI: c_short = 0x2;
pub const POLLOUT: c_short = 0x4;
pub const POLLERR: c_short = 0x8;
pub const POLLHUP: c_short = 0x10;
pub const POLLNVAL: c_short = 0x20;

pub const SOL_SOCKET: c_int = 0xffff;

pub const MSG_OOB: c_int = 1;
pub const MSG_PEEK: c_int = 2;
pub const MSG_DONTWAIT: c_int = 4;
pub const MSG_DONTROUTE: c_int = 0;
pub const MSG_WAITALL: c_int = 0;
pub const MSG_MORE: c_int = 0;
pub const MSG_NOSIGNAL: c_int = 0;

pub const EAI_AGAIN: c_int = 2;
pub const EAI_BADFLAGS: c_int = 3;
pub const EAI_FAIL: c_int = 4;
pub const EAI_SERVICE: c_int = 9;
pub const EAI_SYSTEM: c_int = 11;
pub const EAI_OVERFLOW: c_int = 14;

pub use crate::unix::newlib::generic::{dirent, sigset_t, stat};

pub const SIG_BLOCK: c_int = 1;
pub const SIG_UNBLOCK: c_int = 2;
pub const SIG_SETMASK: c_int = 0;
pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGILL: c_int = 4;
pub const SIGTRAP: c_int = 5;
pub const SIGABRT: c_int = 6;
pub const SIGPIPE: c_int = 13;
pub const SIGTERM: c_int = 15;
pub const SIGKILL: c_int = 9;
pub const SIGCHLD: c_int = 17;
pub const SIGCONT: c_int = 18;
pub const SIGSTOP: c_int = 19;
pub const SIGTSTP: c_int = 20;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGURG: c_int = 23;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;
pub const SIGVTALRM: c_int = 26;
pub const SIGPROF: c_int = 27;
pub const SIGWINCH: c_int = 28;
pub const SIGIO: c_int = 29;
pub const SIGSYS: c_int = 31;
pub const SIGBUS: c_int = 7;
pub const SIGFPE: c_int = 8;
pub const SIGUSR1: c_int = 10;
pub const SIGSEGV: c_int = 11;
pub const SIGUSR2: c_int = 12;
pub const SIGALRM: c_int = 14;

pub const PTHREAD_STACK_MIN: size_t = 16384;
pub const WNOHANG: c_int = 1;
pub const WUNTRACED: c_int = 2;

safe_f! {
    pub const fn WIFSTOPPED(status: c_int) -> bool {
        (status & 0xff) == 0x7f
    }

    pub const fn WSTOPSIG(status: c_int) -> c_int {
        WEXITSTATUS(status)
    }

    pub const fn WIFSIGNALED(status: c_int) -> bool {
        ((status & 0x7f) > 0) && ((status & 0x7f) < 0x7f)
    }

    pub const fn WTERMSIG(status: c_int) -> c_int {
        status & 0x7f
    }

    pub const fn WIFEXITED(status: c_int) -> bool {
        (status & 0xff) == 0
    }

    pub const fn WEXITSTATUS(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub const fn WIFCONTINUED(_status: c_int) -> bool {
        true
    }

    pub const fn WCOREDUMP(_status: c_int) -> bool {
        false
    }
}

extern "C" {
    pub fn futimens(fd: c_int, times: *const crate::timespec) -> c_int;
    pub fn utimensat(
        fd: c_int,
        path: *const c_char,
        times: *const crate::timespec,
        flag: c_int,
    ) -> c_int;
    pub fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;
    pub fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;
    pub fn dirfd(dirp: *mut crate::DIR) -> c_int;
    pub fn setgroups(ngroups: c_int, grouplist: *const crate::gid_t) -> c_int;
}
