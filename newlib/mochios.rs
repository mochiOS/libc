use crate::prelude::*;

pub type clock_t = c_ulong;
// The x86_64 SDK/newlib runtime uses signed 64-bit seconds, not newlib's
// generic 32-bit time_t. Size alone cannot detect timespec's padding mismatch.
const _: [(); 8] = [(); core::mem::size_of::<crate::time_t>()];
const _: [(); 8] = [(); core::mem::align_of::<crate::time_t>()];
const _: [(); 8] = [(); core::mem::size_of::<crate::suseconds_t>()];
const _: [(); 16] = [(); core::mem::size_of::<crate::timespec>()];
const _: [(); 8] = [(); core::mem::offset_of!(crate::timespec, tv_nsec)];
const _: [(); 16] = [(); core::mem::size_of::<crate::timeval>()];
const _: [(); 8] = [(); core::mem::offset_of!(crate::timeval, tv_usec)];
pub const TIMER_ABSTIME: c_int = 4;
pub type wchar_t = u32;
pub type posix_spawn_file_actions_t = *mut c_void;

pub type pthread_rwlock_t = c_uint;
pub type pthread_spinlock_t = c_uint;
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = c_uint::MAX;
pub const PTHREAD_ONCE_INIT: pthread_once_t = pthread_once_t { initialized: 1, executed: 0 };
pub const PTHREAD_BARRIER_SERIAL_THREAD: c_int = -1;
pub const SEM_FAILED: *mut sem_t = core::ptr::null_mut();
pub const SEM_VALUE_MAX: c_int = 2147483647;

const _: [(); 4] = [(); core::mem::size_of::<pthread_rwlock_t>()];
const _: [(); 4] = [(); core::mem::align_of::<pthread_rwlock_t>()];
const _: [(); 8] = [(); core::mem::size_of::<pthread_rwlockattr_t>()];
const _: [(); 4] = [(); core::mem::align_of::<pthread_rwlockattr_t>()];
const _: [(); 8] = [(); core::mem::size_of::<pthread_once_t>()];
const _: [(); 4] = [(); core::mem::align_of::<pthread_once_t>()];
const _: [(); 16] = [(); core::mem::size_of::<sem_t>()];
const _: [(); 8] = [(); core::mem::align_of::<sem_t>()];
const _: [(); 32] = [(); core::mem::size_of::<pthread_barrier_t>()];
const _: [(); 8] = [(); core::mem::align_of::<pthread_barrier_t>()];
const _: [(); 8] = [(); core::mem::size_of::<pthread_barrierattr_t>()];
const _: [(); 4] = [(); core::mem::align_of::<pthread_barrierattr_t>()];

pub const PTHREAD_MUTEX_DEFAULT: c_int = 3;
pub const PTHREAD_PROCESS_PRIVATE: c_int = 0;
pub const PTHREAD_PROCESS_SHARED: c_int = 1;
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    state: 0, kind: 0, owner: 0, depth: 0, initialized: 1, waiters: 0, reserved: 0,
};
const _: [(); 32] = [(); core::mem::size_of::<pthread_mutex_t>()];
const _: [(); 8] = [(); core::mem::align_of::<pthread_mutex_t>()];
const _: [(); 12] = [(); core::mem::size_of::<pthread_mutexattr_t>()];
const _: [(); crate::__SIZEOF_PTHREAD_MUTEXATTR_T] = [(); core::mem::size_of::<pthread_mutexattr_t>()];
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    generation: 0, gate: 0, waiters: 0, initialized: 1, clock: 1,
};
const _: [(); 24] = [(); core::mem::size_of::<pthread_cond_t>()];
const _: [(); 8] = [(); core::mem::align_of::<pthread_cond_t>()];
const _: [(); crate::__SIZEOF_PTHREAD_COND_T] = [(); core::mem::size_of::<pthread_cond_t>()];
const _: [(); crate::__SIZEOF_PTHREAD_CONDATTR_T] = [(); core::mem::size_of::<pthread_condattr_t>()];

const _: [(); 4] = [(); core::mem::size_of::<crate::pthread_t>()];

s! {
    pub struct rusage {
        pub ru_utime: crate::timeval,
        pub ru_stime: crate::timeval,
    }
    pub struct pthread_rwlockattr_t { initialized: c_int, shared: c_int }
    pub struct pthread_once_t { initialized: c_int, executed: c_uint }
    pub struct sem_t { count: u64, initialized: c_uint, reserved: c_uint }
    pub struct pthread_barrier_t {
        count: c_uint,
        initialized: c_uint,
        entered: u64,
        completed: u64,
        departed: u64,
    }
    pub struct pthread_barrierattr_t { initialized: c_int, shared: c_int }
    pub struct pthread_cond_t {
        generation: u64,
        gate: c_uint,
        waiters: c_uint,
        initialized: c_uint,
        clock: c_int,
    }
    pub struct pthread_condattr_t {
        initialized: c_int,
        clock: c_int,
        shared: c_int,
    }
    pub struct pthread_mutex_t {
        state: c_uint,
        kind: c_int,
        owner: u64,
        depth: c_uint,
        initialized: c_uint,
        waiters: c_uint,
        reserved: c_uint,
    }
    pub struct pthread_mutexattr_t {
        initialized: c_int,
        kind: c_int,
        shared: c_int,
    }
    pub struct sockaddr {
        pub sa_family: super::sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct msghdr {
        pub msg_name: *mut c_void,
        pub msg_namelen: crate::socklen_t,
        pub msg_iov: *mut crate::iovec,
        pub msg_iovlen: size_t,
        pub msg_control: *mut c_void,
        pub msg_controllen: size_t,
        pub msg_flags: c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: size_t,
        pub cmsg_level: c_int,
        pub cmsg_type: c_int,
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

    #[repr(align(8))]
    pub struct sockaddr_storage {
        pub ss_family: super::sa_family_t,
        __ss_padding: Padding<[u8; 126]>,
    }
}

pub const AF_UNIX: c_int = 1;
pub const AF_LOCAL: c_int = AF_UNIX;
pub const PF_UNIX: c_int = AF_UNIX;
pub const PF_LOCAL: c_int = AF_LOCAL;
pub const PF_INET6: c_int = AF_INET6;
pub const RUSAGE_SELF: c_int = 0;
pub const RUSAGE_CHILDREN: c_int = -1;
const _: [(); 32] = [(); core::mem::size_of::<rusage>()];
const _: [(); 8] = [(); core::mem::align_of::<rusage>()];
const _: [(); 16] = [(); core::mem::offset_of!(rusage, ru_stime)];
const _: unsafe extern "C" fn(c_int, *mut rusage) -> c_int = crate::getrusage;
pub const AF_INET6: c_int = 10;
pub const FIONBIO: c_ulong = 0x5421;
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
pub const O_CLOEXEC: c_int = 0x40000;

pub const _SC_OPEN_MAX: c_int = 4;
pub const _SC_PAGESIZE: c_int = 8;
pub const _SC_PAGE_SIZE: c_int = _SC_PAGESIZE;
pub const _SC_NPROCESSORS_CONF: c_int = 9;
pub const _SC_NPROCESSORS_ONLN: c_int = 10;
pub const _SC_THREAD_STACK_MIN: c_int = 39;
pub const _SC_GETPW_R_SIZE_MAX: c_int = 51;
pub const _SC_HOST_NAME_MAX: c_int = 65;

pub const PROT_NONE: c_int = 0x0;
pub const PROT_READ: c_int = 0x1;
pub const PROT_WRITE: c_int = 0x2;
pub const PROT_EXEC: c_int = 0x4;

pub const MAP_SHARED: c_int = 0x1;
pub const MAP_PRIVATE: c_int = 0x2;
pub const MAP_FIXED: c_int = 0x10;
pub const MAP_ANON: c_int = 0x20;
pub const MAP_ANONYMOUS: c_int = MAP_ANON;
pub const MAP_STACK: c_int = 0x20000;
pub const MAP_NORESERVE: c_int = 0x4000;
pub const MAP_POPULATE: c_int = 0x8000;
pub const MAP_HUGETLB: c_int = 0x40000;
pub const MAP_HUGE_SHIFT: c_int = 26;
pub const MAP_HUGE_MASK: c_int = 0x3f << MAP_HUGE_SHIFT;
pub const MAP_FAILED: *mut c_void = !0isize as *mut c_void;

pub const MS_ASYNC: c_int = 0x1;
pub const MS_INVALIDATE: c_int = 0x2;
pub const MS_SYNC: c_int = 0x4;

pub const MADV_NORMAL: c_int = 0;
pub const MADV_RANDOM: c_int = 1;
pub const MADV_SEQUENTIAL: c_int = 2;
pub const MADV_WILLNEED: c_int = 3;
pub const MADV_DONTNEED: c_int = 4;

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

pub const FD_CLOEXEC: c_int = 1;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;

pub const SIG_DFL: size_t = 0;
pub const SIG_ERR: size_t = !0usize;

pub const POSIX_SPAWN_RESETIDS: c_int = 0x01;
pub const POSIX_SPAWN_SETPGROUP: c_int = 0x02;
pub const POSIX_SPAWN_SETSCHEDPARAM: c_int = 0x04;
pub const POSIX_SPAWN_SETSCHEDULER: c_int = 0x08;
pub const POSIX_SPAWN_SETSIGDEF: c_int = 0x10;
pub const POSIX_SPAWN_SETSIGMASK: c_int = 0x20;
pub const POSIX_SPAWN_SETSID: c_int = 0x80;

pub const POLLIN: c_short = 0x1;
pub const POLLPRI: c_short = 0x2;
pub const POLLOUT: c_short = 0x4;
pub const POLLERR: c_short = 0x8;
pub const POLLHUP: c_short = 0x10;
pub const POLLNVAL: c_short = 0x20;

pub const SOL_SOCKET: c_int = 1;
pub const SOCK_SEQPACKET: c_int = 5;
pub const SOCK_NONBLOCK: c_int = 0x800;
pub const SOCK_CLOEXEC: c_int = 0x80000;
pub const SO_DEBUG: c_int = 1;
pub const SO_REUSEADDR: c_int = 2;
pub const SO_TYPE: c_int = 3;
pub const SO_ERROR: c_int = 4;
pub const SO_DONTROUTE: c_int = 5;
pub const SO_BROADCAST: c_int = 6;
pub const SO_SNDBUF: c_int = 7;
pub const SO_RCVBUF: c_int = 8;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_OOBINLINE: c_int = 10;
pub const SO_LINGER: c_int = 13;
pub const SO_REUSEPORT: c_int = 15;
pub const SO_RCVLOWAT: c_int = 18;
pub const SO_SNDLOWAT: c_int = 19;
pub const SO_RCVTIMEO: c_int = 20;
pub const SO_SNDTIMEO: c_int = 21;
pub const SO_TIMESTAMP: c_int = 29;
pub const SO_ACCEPTCONN: c_int = 30;
pub const SCM_RIGHTS: c_int = 1;

pub const MSG_OOB: c_int = 1;
pub const MSG_PEEK: c_int = 2;
pub const MSG_DONTWAIT: c_int = 0x40;
pub const MSG_DONTROUTE: c_int = 0x4;
pub const MSG_WAITALL: c_int = 0x100;
pub const MSG_MORE: c_int = 0x8000;
pub const MSG_NOSIGNAL: c_int = 0x4000;
pub const MSG_CTRUNC: c_int = 0x8;
pub const MSG_TRUNC: c_int = 0x20;
pub const MSG_CMSG_CLOEXEC: c_int = 0x40000000;

const _: [(); 2] = [(); core::mem::size_of::<crate::sa_family_t>()];
const _: [(); 16] = [(); core::mem::size_of::<sockaddr>()];
const _: [(); 110] = [(); core::mem::size_of::<sockaddr_un>()];
const _: [(); 128] = [(); core::mem::size_of::<sockaddr_storage>()];
const _: [(); 8] = [(); core::mem::align_of::<sockaddr_storage>()];
const _: [(); 56] = [(); core::mem::size_of::<msghdr>()];
const _: [(); 8] = [(); core::mem::align_of::<msghdr>()];
const _: [(); 16] = [(); core::mem::size_of::<cmsghdr>()];
const _: [(); 8] = [(); core::mem::align_of::<cmsghdr>()];

const fn cmsg_align(length: size_t) -> size_t {
    length.wrapping_add(core::mem::size_of::<size_t>() - 1)
        & !(core::mem::size_of::<size_t>() - 1)
}

f! {
    pub fn CMSG_FIRSTHDR(message: *const msghdr) -> *mut cmsghdr {
        if !(*message).msg_control.is_null()
            && (*message).msg_controllen >= core::mem::size_of::<cmsghdr>()
        {
            (*message).msg_control.cast()
        } else {
            core::ptr::null_mut()
        }
    }

    pub fn CMSG_DATA(header: *const cmsghdr) -> *mut c_uchar {
        header.cast::<c_uchar>()
            .add(cmsg_align(core::mem::size_of::<cmsghdr>())) as *mut c_uchar
    }

    pub const fn CMSG_SPACE(length: c_uint) -> c_uint {
        cmsg_align(core::mem::size_of::<cmsghdr>())
            .wrapping_add(cmsg_align(length as size_t)) as c_uint
    }

    pub const fn CMSG_LEN(length: c_uint) -> c_uint {
        cmsg_align(core::mem::size_of::<cmsghdr>()).wrapping_add(length as size_t) as c_uint
    }

    pub fn CMSG_NXTHDR(message: *const msghdr, header: *const cmsghdr) -> *mut cmsghdr {
        let base = (*message).msg_control as usize;
        let current = header as usize;
        let length = (*message).msg_controllen;
        if base == 0 || current == 0 || current < base || current - base > length {
            return core::ptr::null_mut();
        }
        let remaining = length - (current - base);
        if remaining < core::mem::size_of::<cmsghdr>() {
            return core::ptr::null_mut();
        }
        let size = (*header).cmsg_len;
        if size < core::mem::size_of::<cmsghdr>() || size > remaining {
            return core::ptr::null_mut();
        }
        let step = cmsg_align(size);
        if step < size || step > remaining
            || remaining - step < core::mem::size_of::<cmsghdr>()
            || current.checked_add(step).is_none()
        {
            return core::ptr::null_mut();
        }
        current.wrapping_add(step) as *mut cmsghdr
    }
}

pub const EAI_AGAIN: c_int = 2;
pub const EAI_BADFLAGS: c_int = 3;
pub const EAI_FAIL: c_int = 4;
pub const EAI_SERVICE: c_int = 9;
pub const EAI_SYSTEM: c_int = 11;
pub const EAI_OVERFLOW: c_int = 14;

pub use crate::unix::newlib::generic::sigset_t;

s! {
    pub struct stat {
        pub st_dev: c_short,
        pub st_ino: c_ushort,
        pub st_mode: c_uint,
        pub st_nlink: c_ushort,
        pub st_uid: c_ushort,
        pub st_gid: c_ushort,
        pub st_rdev: c_short,
        pub st_size: c_long,
        pub st_atime: c_long,
        pub st_atime_nsec: c_long,
        pub st_mtime: c_long,
        pub st_mtime_nsec: c_long,
        pub st_ctime: c_long,
        pub st_ctime_nsec: c_long,
        pub st_blksize: c_long,
        pub st_blocks: c_long,
        pub st_spare4: [c_long; 2usize],
    }

    pub struct posix_spawnattr_t {
        __flags: c_short,
        __pgroup: crate::pid_t,
    }

    pub struct dirent {
        pub d_ino: u64,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256usize],
    }
}

const _: [(); 104] = [(); core::mem::size_of::<stat>()];
const _: [(); 8] = [(); core::mem::align_of::<stat>()];
const _: [(); 4] = [(); core::mem::offset_of!(stat, st_mode)];
const _: [(); 16] = [(); core::mem::offset_of!(stat, st_size)];
const _: [(); 24] = [(); core::mem::offset_of!(stat, st_atime)];
const _: [(); 72] = [(); core::mem::offset_of!(stat, st_blksize)];
const _: [(); 88] = [(); core::mem::offset_of!(stat, st_spare4)];

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
    pub fn sendmsg(socket: c_int, message: *const msghdr, flags: c_int) -> ssize_t;
    pub fn recvmsg(socket: c_int, message: *mut msghdr, flags: c_int) -> ssize_t;
    pub fn accept4(
        socket: c_int,
        address: *mut sockaddr,
        length: *mut crate::socklen_t,
        flags: c_int,
    ) -> c_int;
    pub fn pthread_spin_init(lock: *mut pthread_spinlock_t, shared: c_int) -> c_int;
    pub fn pthread_spin_destroy(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_lock(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_trylock(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_unlock(lock: *mut pthread_spinlock_t) -> c_int;
    pub fn pthread_once(once: *mut pthread_once_t, initialize: Option<unsafe extern "C" fn()>) -> c_int;
    pub fn pthread_rwlockattr_getpshared(attr: *const pthread_rwlockattr_t, shared: *mut c_int) -> c_int;
    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t, shared: c_int) -> c_int;
    pub fn pthread_rwlock_timedrdlock(lock: *mut pthread_rwlock_t, time: *const crate::timespec) -> c_int;
    pub fn pthread_rwlock_timedwrlock(lock: *mut pthread_rwlock_t, time: *const crate::timespec) -> c_int;
    pub fn pthread_rwlock_clockrdlock(lock: *mut pthread_rwlock_t, clock: crate::clockid_t, time: *const crate::timespec) -> c_int;
    pub fn pthread_rwlock_clockwrlock(lock: *mut pthread_rwlock_t, clock: crate::clockid_t, time: *const crate::timespec) -> c_int;
    pub fn sem_getvalue(semaphore: *mut sem_t, value: *mut c_int) -> c_int;
    pub fn sem_timedwait(semaphore: *mut sem_t, time: *const crate::timespec) -> c_int;
    pub fn sem_clockwait(semaphore: *mut sem_t, clock: crate::clockid_t, time: *const crate::timespec) -> c_int;
    pub fn pthread_barrierattr_init(attr: *mut pthread_barrierattr_t) -> c_int;
    pub fn pthread_barrierattr_destroy(attr: *mut pthread_barrierattr_t) -> c_int;
    pub fn pthread_barrierattr_getpshared(attr: *const pthread_barrierattr_t, shared: *mut c_int) -> c_int;
    pub fn pthread_barrierattr_setpshared(attr: *mut pthread_barrierattr_t, shared: c_int) -> c_int;
    pub fn pthread_barrier_init(barrier: *mut pthread_barrier_t, attr: *const pthread_barrierattr_t, count: c_uint) -> c_int;
    pub fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    pub fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_condattr_getclock(attr: *const pthread_condattr_t, clock: *mut crate::clockid_t) -> c_int;
    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t, clock: crate::clockid_t) -> c_int;
    pub fn pthread_condattr_getpshared(attr: *const pthread_condattr_t, shared: *mut c_int) -> c_int;
    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, shared: c_int) -> c_int;
    pub fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> c_int;
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_cond_timedwait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t, time: *const crate::timespec) -> c_int;
    pub fn pthread_cond_clockwait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t, clock: crate::clockid_t, time: *const crate::timespec) -> c_int;
    pub fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_timedlock(mutex: *mut pthread_mutex_t, time: *const crate::timespec) -> c_int;
    pub fn pthread_mutex_clocklock(mutex: *mut pthread_mutex_t, clock: crate::clockid_t, time: *const crate::timespec) -> c_int;
    pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_gettype(attr: *const pthread_mutexattr_t, kind: *mut c_int) -> c_int;
    pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, kind: c_int) -> c_int;
    pub fn pthread_mutexattr_getpshared(attr: *const pthread_mutexattr_t, shared: *mut c_int) -> c_int;
    pub fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t, shared: c_int) -> c_int;
    pub fn getdelim(
        lineptr: *mut *mut c_char,
        capacity: *mut size_t,
        delimiter: c_int,
        stream: *mut crate::FILE,
    ) -> crate::ssize_t;
    pub fn getline(
        lineptr: *mut *mut c_char,
        capacity: *mut size_t,
        stream: *mut crate::FILE,
    ) -> crate::ssize_t;
    pub fn clock_nanosleep(
        clock_id: crate::clockid_t,
        flags: c_int,
        request: *const crate::timespec,
        remaining: *mut crate::timespec,
    ) -> c_int;
    pub fn pthread_create(
        thread: *mut crate::pthread_t,
        attr: *const crate::pthread_attr_t,
        start: extern "C" fn(*mut c_void) -> *mut c_void,
        argument: *mut c_void,
    ) -> c_int;
    pub fn pthread_join(thread: crate::pthread_t, value: *mut *mut c_void) -> c_int;
    pub fn pthread_detach(thread: crate::pthread_t) -> c_int;
    pub fn pthread_self() -> crate::pthread_t;
    pub fn pthread_attr_init(attr: *mut crate::pthread_attr_t) -> c_int;
    pub fn pthread_attr_destroy(attr: *mut crate::pthread_attr_t) -> c_int;
    pub fn pthread_attr_setstacksize(
        attr: *mut crate::pthread_attr_t,
        stack_size: size_t,
    ) -> c_int;
    pub fn pthread_key_create(
        key: *mut crate::pthread_key_t,
        destructor: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> c_int;
    pub fn pthread_key_delete(key: crate::pthread_key_t) -> c_int;
    pub fn pthread_setspecific(key: crate::pthread_key_t, value: *const c_void) -> c_int;
    pub fn pthread_getspecific(key: crate::pthread_key_t) -> *mut c_void;
    pub fn sched_yield() -> c_int;
    pub fn nanosleep(request: *const crate::timespec, remaining: *mut crate::timespec) -> c_int;
    pub fn futimens(fd: c_int, times: *const crate::timespec) -> c_int;
    pub fn utimensat(
        fd: c_int,
        path: *const c_char,
        times: *const crate::timespec,
        flag: c_int,
    ) -> c_int;
    pub fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;
    pub fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;
    pub fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: crate::off_t,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, len: size_t) -> c_int;
    pub fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    pub fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int;
    pub fn msync(addr: *mut c_void, len: size_t, flags: c_int) -> c_int;
    pub fn sysconf(name: c_int) -> c_long;
    pub fn pipe(fds: *mut c_int) -> c_int;
    pub fn pipe2(fds: *mut c_int, flags: c_int) -> c_int;
    pub fn fork() -> crate::pid_t;
    pub fn execvp(file: *const c_char, argv: *const *const c_char) -> c_int;
    pub fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn signal(signum: c_int, handler: size_t) -> size_t;
    pub fn lstat(path: *const c_char, stat_buf: *mut crate::stat) -> c_int;
    pub fn dirfd(dirp: *mut crate::DIR) -> c_int;
    pub fn setgroups(ngroups: c_int, grouplist: *const crate::gid_t) -> c_int;
    pub fn setgid(gid: crate::gid_t) -> c_int;
    pub fn setuid(uid: crate::uid_t) -> c_int;
    pub fn setpgid(pid: crate::pid_t, pgid: crate::pid_t) -> c_int;
    pub fn setsid() -> crate::pid_t;
    pub fn tcgetpgrp(fd: c_int) -> crate::pid_t;
    pub fn tcsetpgrp(fd: c_int, pgrp: crate::pid_t) -> c_int;
    pub fn login_tty(fd: c_int) -> c_int;
    pub fn chroot(path: *const c_char) -> c_int;
    pub fn waitpid(pid: crate::pid_t, status: *mut c_int, options: c_int) -> crate::pid_t;
    pub fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> c_int;
    pub fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> c_int;
    pub fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: c_short) -> c_int;
    pub fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, pgroup: crate::pid_t) -> c_int;
    pub fn posix_spawnattr_setsigdefault(
        attr: *mut posix_spawnattr_t,
        sigdefault: *const sigset_t,
    ) -> c_int;
    pub fn posix_spawn_file_actions_init(actions: *mut posix_spawn_file_actions_t) -> c_int;
    pub fn posix_spawn_file_actions_destroy(actions: *mut posix_spawn_file_actions_t) -> c_int;
    pub fn posix_spawn_file_actions_addclose(
        actions: *mut posix_spawn_file_actions_t,
        fd: c_int,
    ) -> c_int;
    pub fn posix_spawn_file_actions_adddup2(
        actions: *mut posix_spawn_file_actions_t,
        fd: c_int,
        newfd: c_int,
    ) -> c_int;
    pub fn posix_spawn(
        pid: *mut crate::pid_t,
        path: *const c_char,
        file_actions: *const posix_spawn_file_actions_t,
        attrp: *const posix_spawnattr_t,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    pub fn posix_spawnp(
        pid: *mut crate::pid_t,
        file: *const c_char,
        file_actions: *const posix_spawn_file_actions_t,
        attrp: *const posix_spawnattr_t,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
}
