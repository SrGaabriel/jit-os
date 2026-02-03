#![no_std]
#![no_main]

extern crate raph_common;
extern crate raph_runtime;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> () {
    42;
}
