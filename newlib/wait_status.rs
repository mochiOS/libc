//! Status encoding shared by the mochiOS newlib runtime and sys/wait.h.
#![allow(non_snake_case)]
use super::c_int;
pub const WNOHANG: c_int = 1;
pub const WUNTRACED: c_int = 2;
pub const WCONTINUED: c_int = 8;
pub const WEXITED: c_int = 4;
pub const WSTOPPED: c_int = WUNTRACED;
pub const WNOWAIT: c_int = 16;
pub const fn WIFSTOPPED(status: c_int) -> bool {
    status & 0xff == 0x7f
}
pub const fn WSTOPSIG(status: c_int) -> c_int {
    WEXITSTATUS(status)
}
pub const fn WIFSIGNALED(status: c_int) -> bool {
    status & 0x7f > 0 && status & 0x7f < 0x7f
}
pub const fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}
pub const fn WIFEXITED(status: c_int) -> bool {
    status & 0xff == 0
}
pub const fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}
pub const fn WIFCONTINUED(status: c_int) -> bool {
    status == 0xffff
}
/// Decode the bit only; this does not claim the runtime writes core files.
pub const fn WCOREDUMP(status: c_int) -> bool {
    WIFSIGNALED(status) && status & 0x80 != 0
}
