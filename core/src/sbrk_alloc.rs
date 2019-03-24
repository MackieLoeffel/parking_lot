use allocator_api::{Alloc, Layout, AllocErr};
use libc;
use std::ptr::NonNull;

pub struct SbrkAlloc;
unsafe impl Alloc for SbrkAlloc {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        // TODO: loop here like in je malloc to no always alloc
        // too much memory
        let new_brk = libc::sbrk((layout.size() + layout.align()) as isize) as usize;
        if new_brk == -1isize as usize {
            return Err(AllocErr);
        }
        let aligned_addr = new_brk + layout.align() - (new_brk % layout.align());
        assert!(aligned_addr % layout.align() == 0);
        Ok(NonNull::new_unchecked(aligned_addr as *mut u8))
    }

    unsafe fn dealloc(&mut self, _ptr: NonNull<u8>, _layout: Layout) {
        // nop, we cannot deallocate
    }
}
