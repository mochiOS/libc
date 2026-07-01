use core::{concat, env, include};

include!(concat!(env!("OUT_DIR"), "/newlib_mod.rs"));
