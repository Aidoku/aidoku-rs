#![no_std]
#![no_main]
#![feature(core_intrinsics)]

#[cfg_attr(not(test), panic_handler)]
pub unsafe fn panic_handle(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort()
}

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
