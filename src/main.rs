#![no_std]
#![no_main]

extern crate arm_core;

#[no_mangle]
pub fn main() -> ! {
    let _x = 42;

    loop {}
}