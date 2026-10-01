#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use rskernel::test_runner;
use core::panic::PanicInfo;
use rskernel::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rskernel::test_panic_handler(info)
}

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println(){
    println!("test_println output");
}
