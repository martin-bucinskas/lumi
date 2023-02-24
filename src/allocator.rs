pub mod bump;
pub mod linked_list;
pub mod fixed_size_block;

use linked_list_allocator::LockedHeap;
use x86_64::{
  structures::paging::{
    mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB
  },
  VirtAddr
};
use crate::allocator::bump::BumpAllocator;
use crate::allocator::fixed_size_block::FixedSizeBlockAllocator;
use crate::allocator::linked_list::LinkedListAllocator;

// define virtual memory region for heap
pub const HEAP_START: usize = 0x_4444_4444_0000;
// 100Kib heap size
pub const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
// static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap(
  mapper: &mut impl Mapper<Size4KiB>,
  frame_allocator: &mut impl FrameAllocator<Size4KiB>
) -> Result<(), MapToError<Size4KiB>> {
  let page_range = {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let heap_end = heap_start + HEAP_SIZE - 1u64;
    let heap_start_page = Page::containing_address(heap_start);
    let heap_end_page = Page::containing_address(heap_end);
    Page::range_inclusive(heap_start_page, heap_end_page)
  };

  for page in page_range {
    let frame = frame_allocator
      .allocate_frame()
      .ok_or(MapToError::FrameAllocationFailed)?;
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {
      mapper.map_to(page, frame, flags, frame_allocator)?.flush()
    };
  }

  unsafe {
    ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
  }

  Ok(())
}

/// wrapper around spin::Mutex to permit trait implementations
pub struct Locked<A> {
  inner: spin::Mutex<A>
}

impl<A> Locked<A> {
  pub const fn new (inner: A) -> Self {
    Locked {
      inner: spin::Mutex::new(inner)
    }
  }

  pub fn lock(&self) -> spin::MutexGuard<A> {
    self.inner.lock()
  }
}

/// align the given address `addr upwards to aligment `align`.
///
/// requires that `align` is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
  (addr + align - 1) & !(align - 1)
}
