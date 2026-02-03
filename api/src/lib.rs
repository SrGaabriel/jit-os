#![no_std]

extern crate raph_common;

use core::panic::PanicInfo;

pub fn abort(info: &PanicInfo) -> ! {
    todo!("Implement abort logic: {}", info);
}
