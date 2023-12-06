#![no_std]

use core::{panic::PanicInfo, ptr, mem};

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;

            f()
        }
    };
}

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: usize;
        static mut _ebss: usize;
        static mut _sdata: usize;
        static mut _edata: usize;
        static mut _sidata: usize;
    }

    let mut p_bss: *mut usize = &mut _sbss;
    while p_bss < &mut _ebss {
        ptr::write_volatile(p_bss, mem::zeroed());
        p_bss = p_bss.offset(1);
    }

    let mut p_data: *mut usize = &mut _sdata;
    let mut p_sidata: *mut usize = &mut _sidata;
    while p_data < &mut _edata {
        ptr::write(p_data, ptr::read(p_sidata));
        p_data = p_data.offset(1);
        p_sidata = p_sidata.offset(1);
    }

    extern "Rust" {
        fn main() -> !;
    }

    main()
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

extern "C" {
    fn nmi_handler();
    fn hard_fault_handler();
    fn memory_management_fault_handler();
    fn bus_fault_handler();
    fn usage_fault_handler();
    fn sv_call_handler();
    fn pending_sv_handler();
    fn systick_handler();
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: nmi_handler },
    Vector { handler: hard_fault_handler },
    Vector { handler: memory_management_fault_handler },
    Vector { handler: bus_fault_handler },
    Vector { handler: usage_fault_handler },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: sv_call_handler },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: pending_sv_handler },
    Vector { handler: systick_handler },
];

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
fn default_exception_handler() {
    loop {}
}