#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lumi::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{Page, PageTable, Translate};
use x86_64::VirtAddr;
use lumi::memory;
use lumi::memory::BootInfoFrameAllocator;
use lumi::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use lumi::memory::BootInfoFrameAllocator;

  println!("# Lumi v{} #", "0.1.0");

  println!("Initialising...");
  lumi::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocator = unsafe {
    BootInfoFrameAllocator::init(&boot_info.memory_map)
  };

  // let page = Page::containing_address(VirtAddr::new(0));
  // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
  //
  // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
  // unsafe {
  //   page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e);
  // };

  #[cfg(test)]
  test_main();

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
