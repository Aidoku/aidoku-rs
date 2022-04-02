#![no_std]
#![no_main]
#![feature(core_intrinsics, alloc_error_handler)]

#[cfg_attr(not(test), panic_handler)]
pub unsafe fn panic_handle(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort()
}

#[cfg_attr(not(test), alloc_error_handler)]
pub unsafe fn alloc_error_handle(_: core::alloc::Layout) -> ! {
    core::intrinsics::abort()
}

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use aidoku_imports::{
    error::Result,
    net::{HttpMethod, Request},
};

pub fn foobar() -> Result<i32> {
    let req = Request::new("https://example.com", HttpMethod::Get);
    let json = req.json();
    let obj = json.as_object()?;
    let num = obj.get("value").as_int()?;
    Ok(num)
}

#[no_mangle]
extern "C" fn foo() -> i32 {
    match foobar() {
        Ok(v) => v,
        Err(_) => -1,
    }
}
