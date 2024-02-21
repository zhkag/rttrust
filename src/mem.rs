use crate::{align, align_down, system};
use macros::init_export;
use crate::include::ALIGN_SIZE;

struct SmallMemItem {
    pool_ptr: usize,
    next: usize,
    prev: usize,
}

pub struct SmallMem {
    address: usize,
    total: usize,
    mem_size_aligned:usize,
    heap_ptr: *mut u8,
    heap_end: *mut SmallMemItem,
    lfree: *mut SmallMemItem,
}

macro_rules! MIN_SIZE {
    () => {12};
}

macro_rules! SIZEOF_STRUCT_MEM {
    () => {align!(core::mem::size_of::<SmallMemItem>(), ALIGN_SIZE)};
}

macro_rules! MEM_MASK {
    () => {((!0) - 1)};
}

macro_rules! MEM_USED {
    ($small_mem:expr) => {
        (($small_mem as *mut SmallMem as *const usize as usize) & MEM_MASK!()) | 0x1
    };
}
macro_rules! MEM_FREED {
    ($small_mem:expr) => {
        (($small_mem as *mut SmallMem as *const usize as usize) & MEM_MASK!()) | 0x0
    };
}

macro_rules! MEM_ISUSED {
    ($mem:expr) => {
        $mem.pool_ptr & !MEM_MASK!() == 1
    };
}

macro_rules! MIN_SIZE_ALIGNED {
    () => {
        align!(MIN_SIZE!(), ALIGN_SIZE)
    };
}

macro_rules! MEM_POOL {
    ($mem:expr) => {
        unsafe {&mut *(($mem.pool_ptr & MEM_MASK!()) as *mut SmallMem)}
    };
}

impl SmallMem {
    fn new(heap_start: usize, size: usize) -> &'static mut Self {
        let heap_start_align = align!(heap_start, ALIGN_SIZE);
        let small_mem = unsafe {&mut *(heap_start_align as *const usize as *mut SmallMem)};
        let start_addr = heap_start_align + core::mem::size_of::<SmallMem>();
        let begin_align = align!(start_addr, ALIGN_SIZE);
        let end_align   = align_down!(heap_start + size, ALIGN_SIZE);
        let mem_size = end_align - begin_align - 2 * SIZEOF_STRUCT_MEM!();

        unsafe{core::ptr::write_bytes(small_mem,0,core::mem::size_of::<SmallMem>());}

        small_mem.address = begin_align;
        small_mem.total = mem_size;
        small_mem.mem_size_aligned = mem_size;
        small_mem.heap_ptr = begin_align as *mut u8;
        
        let mem = unsafe {&mut *(small_mem.heap_ptr as *const usize as *mut SmallMemItem)};
        
        mem.pool_ptr = MEM_FREED!(small_mem);
        mem.next  = small_mem.mem_size_aligned + SIZEOF_STRUCT_MEM!();
        mem.prev  = 0;
        
        small_mem.heap_end = small_mem.heap_ptr.wrapping_add(mem.next) as *mut SmallMemItem;
        let heap_end = unsafe {&mut *small_mem.heap_end};
        
        heap_end.pool_ptr = MEM_USED!(small_mem);
        heap_end.next  = mem.next;
        heap_end.prev  = mem.next;

        small_mem.lfree = small_mem.heap_ptr as *mut SmallMemItem;
        small_mem
    }

    fn malloc(&mut self, size: usize) -> Option<usize> {
        let size = align!(size, ALIGN_SIZE);
        let mut ptr = self.lfree as *mut usize as usize - self.heap_ptr as *mut usize as usize;
        while ptr <= self.mem_size_aligned - size {
            let mem = unsafe { &mut *(self.heap_ptr.wrapping_add(ptr) as *mut SmallMemItem)};
            if !MEM_ISUSED!(mem) &&  (mem.next - (ptr + SIZEOF_STRUCT_MEM!())) >= size{
                
                if mem.next - (ptr + SIZEOF_STRUCT_MEM!()) >= (size + SIZEOF_STRUCT_MEM!() + MIN_SIZE_ALIGNED!())
                {
                    let ptr2 = ptr + SIZEOF_STRUCT_MEM!() + size;
                    let mem2 = unsafe { &mut *(self.heap_ptr.wrapping_add(ptr2) as *mut SmallMemItem)};
                    mem2.pool_ptr = MEM_FREED!(self);
                    mem2.next = mem.next;
                    mem2.prev = ptr;

                    
                    mem.next = ptr2;

                    if mem2.next != self.mem_size_aligned + SIZEOF_STRUCT_MEM!()
                    {
                        unsafe { &mut *(self.heap_ptr.wrapping_add(mem2.next) as *mut SmallMemItem)}.prev = ptr2;
                    }
                }
                mem.pool_ptr = MEM_USED!(self);
                if mem as *mut SmallMemItem == self.lfree
                {
                    while MEM_ISUSED!(unsafe {&mut *self.lfree}) && self.lfree != self.heap_end{
                        self.lfree = unsafe { &mut *(self.heap_ptr.wrapping_add((&mut *self.lfree).next) as *mut SmallMemItem)};
                    }
                }
                return Some(mem as *mut SmallMemItem as usize + SIZEOF_STRUCT_MEM!());
            }
            ptr = mem.next;
        }


        None
    }

    fn free(&mut self, addr: usize) {
        let mem = unsafe { &mut *((addr - SIZEOF_STRUCT_MEM!()) as *mut SmallMemItem)};
        let small_mem = MEM_POOL!(mem);
        mem.pool_ptr = MEM_FREED!(small_mem);
        if (mem as *mut SmallMemItem) < small_mem.lfree
        {
            small_mem.lfree = mem;
        }
        self.plug_holes(mem);
    }

    fn malloc_align(&mut self, size:usize, align:usize) -> *mut u8 {
        let mut uintptr_size = core::mem::size_of::<usize>();
        uintptr_size -= 1;
        let align = (align + uintptr_size) & !uintptr_size;
        let align_size = ((size + uintptr_size) & !uintptr_size) + align;
        let ret_ptr:usize;
        if let Some(ptr) = self.malloc(align_size){
            let align_ptr:usize;
            if (ptr & (align - 1)) == 0
            {
                align_ptr = ptr + align;
            }
            else
            {
                align_ptr = (ptr + (align - 1)) & !(align - 1);
            }
            unsafe {
                *((align_ptr - core::mem::size_of::<usize>()) as *mut usize) = ptr;
            }
            ret_ptr = align_ptr;
        }
        else {
            unreachable!();
        }
        return ret_ptr as *mut u8;
    }

    fn free_align(&mut self,ptr: *mut u8)
    {
        self.free(unsafe {* ((ptr as usize - core::mem::size_of::<usize>())as *mut usize)});
    }

    fn plug_holes(&mut self, mem: &mut SmallMemItem) {
        
        let nmem = unsafe { &mut *(self.heap_ptr.wrapping_add(mem.next) as *mut SmallMemItem)};
        if mem as *mut SmallMemItem != nmem as *mut SmallMemItem && !MEM_ISUSED!(nmem) && nmem as *mut SmallMemItem != self.heap_end
        {
            if self.lfree == nmem
            {
                self.lfree = mem;
            }
            nmem.pool_ptr = 0;
            mem.next = nmem.next;
            unsafe { &mut *(self.heap_ptr.wrapping_add(nmem.next) as *mut SmallMemItem)}.prev = mem as *mut SmallMemItem as usize - self.heap_ptr as usize;
        }
        
        let pmem = unsafe { &mut *(self.heap_ptr.wrapping_add(mem.prev) as *mut SmallMemItem)};
        if pmem as *mut SmallMemItem != mem as *mut SmallMemItem && !MEM_ISUSED!(pmem)
        {
            if self.lfree == mem
            {
                self.lfree = pmem;
            }
            mem.pool_ptr = 0;
            pmem.next = mem.next;
            unsafe { &mut *(self.heap_ptr.wrapping_add(mem.next) as *mut SmallMemItem)}.prev = pmem as *mut SmallMemItem as usize - self.heap_ptr as usize;
        }
    }

}


const  STM32_SRAM_END:usize = 0x20000000 + 128 * 1024;

use core::alloc::GlobalAlloc;

#[global_allocator]
static ALLOCATOR: SystemHeapAllocator = SystemHeapAllocator;

pub struct SystemHeapAllocator;

unsafe impl GlobalAlloc for SystemHeapAllocator {
    #[inline]
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        system!(heap()).malloc_align(layout.size(), layout.align())
    }
    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        system!(heap()).free_align(ptr);
    }
}

#[init_export("0.0")]
fn men_init() {
    extern {static __bss_end:usize;}
    let heap_begin = unsafe {&__bss_end as *const usize as usize};
    let heap_end = STM32_SRAM_END;

    let begin_align = align!(heap_begin, ALIGN_SIZE);
    let end_align   = align_down!(heap_end, ALIGN_SIZE);

    system!(set_heap(SmallMem::new(begin_align, end_align - begin_align)));
}
