#![no_std]
#![no_main]

use raph_api::println;

extern crate raph_common;
extern crate raph_runtime;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> () {
    println!("Hello, World!")
}
