#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lumi::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lumi::println;
// use ansi_parser::{AnsiParser, Output};

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("# Lumi v{} #", "0.1.0");

  println!("Initialising...");
  lumi::init();

  // x86_64::instructions::interrupts::int3(); // -> test breakpoint

  // trigger a page fault
  // unsafe {
  //   *(0xdeadbeef as *mut u64) = 42;
  // }

  fn stack_overflow() {
    // for each recursion the return address is pushed to stack
    stack_overflow();
  }

  // trigger a stack overflow
  // stack_overflow();

  #[cfg(test)]
  test_main();

  // panic!("Panic - something went wrong...");

  println!(".");

  lumi::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  lumi::hlt_loop();
}

// -------------------------------------------------------------

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  lumi::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
  assert_eq!(1, 1);
}
