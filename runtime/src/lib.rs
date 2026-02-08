#![no_std]

use core::panic::PanicInfo;

use talc::{ErrOnOom, Talc, Talck};

#[global_allocator]
#[cfg(not(target_os = "asha"))]
static TESTING_ALLOCATOR: common::alloc::Allocator = common::alloc::Allocator::new();

#[global_allocator]
#[cfg(target_os = "asha")]
pub static ALLOCATOR: Talck<spin::Mutex<()>, ErrOnOom> = Talc::new(ErrOnOom).lock();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    api::abort(info)
}
