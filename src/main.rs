#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
use core::panic::PanicInfo;

//#[no_mangle]
//pub extern "C" fn _start() -> ! {
//    loop {}
//}

//static HELLO: &[u8] = b"Hello World!";
//
//#[no_mangle]
//pub extern "C" fn _start() -> ! {
//    let vga_buffer = 0xb8000 as *mut u8;
//
//    for (i, &byte) in HELLO.iter().enumerate() {
//        unsafe {
//            *vga_buffer.offset(i as isize * 2) = byte;
//            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//        }
//    }
//
//    loop {}
//}

//#[no_mangle]
//pub extern "C" fn _start() -> ! {
//    use core::fmt::Write;
//
//    vga_buffer::WRITER.lock().write_str("Hello agein").unwrap();
//    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
//
//    loop {}
//}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }
}
