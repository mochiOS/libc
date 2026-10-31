//! The SDK termios layout, not generic newlib's c_line variant.
s! {
    pub struct termios {
        pub c_iflag: u32,
        pub c_oflag: u32,
        pub c_cflag: u32,
        pub c_lflag: u32,
        pub c_cc: [u8; crate::NCCS],
        pub c_ispeed: u32,
        pub c_ospeed: u32,
    }
}
