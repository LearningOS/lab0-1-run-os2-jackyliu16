#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
extern crate log;

#[macro_use]
mod console;
mod batch;
mod lang_items;
mod logging;
mod sbi;
mod sync;
mod syscall;
mod trap;

core::arch::global_asm!(include_str!("entry.asm"));
// linked application's Binary image file infor kernel
core::arch::global_asm!(include_str!("link_app.S"));

// we should clear bss before we run the programme, which is because we save things in bss ( global static variable ) it should be empty before we use
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();                // clear bss semgerment
    logging::init();                    
    println!("[kernel] Hello, world!");
    trap::init();               // start trap
    batch::init();              // 
    batch::run_next_app();      // 
}
