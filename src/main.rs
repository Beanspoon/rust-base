#![no_std]
#![no_main]

use arm_core::entry;

extern crate arm_core;

entry!(main);

fn main() -> ! {
    let _x = 42;

    loop {}
}