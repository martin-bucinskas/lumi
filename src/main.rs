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

  #[cfg(test)]
  test_main();

  // let parsed: Vec<Output> = "This is \u{1b}[3Asome text!"
  //   .ansi_parse()
  //   .take(2)
  //   .collect();

  panic!("Panic - something went wrong...");

  loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
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
