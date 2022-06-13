#![feature(naked_functions)]
#![feature(asm)]
#![no_std]
#![no_main]

mod syscall;

// gets called when a panic occurs
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    syscall::exit(-1);
}

#[no_mangle]
extern fn _start() {
    //syscall::write_stdout("hello\n");

    syscall::exit(0);
}

