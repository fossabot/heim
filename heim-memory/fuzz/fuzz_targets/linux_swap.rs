#![no_main]
#![cfg(target_os = "linux")]

#[macro_use]
extern crate libfuzzer_sys;

use std::str::{self, FromStr};

use heim_memory::sys::Swap;
extern crate heim_memory;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let _ = Swap::parse_str(s);
    }
});
