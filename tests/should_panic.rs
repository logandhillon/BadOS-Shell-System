// Copyright (c) 2024 Logan Dhillon. This software is subject to the Bad Technologies Open Software License.

#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bad_os_shell_system::{serial_println, qemu::{exit, ExitCode}, serial_print};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit(ExitCode::Success);
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[failed, test did not panic]");
        exit(ExitCode::Failed);
    }
    exit(ExitCode::Success);
}

#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
