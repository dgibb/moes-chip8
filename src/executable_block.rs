extern crate libc;

use std::mem;
use std::ops::{Index, IndexMut};
use register_map::register_map;

extern{
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

pub struct executable_block {
    pub block: *mut u8,
    pub map: register_map,
    pub fn_ptr: Option< fn(&mut [u8;16]) -> u64 >,
    pub chip8_entry: u16,
    pub chip8_exit: u16,
}

unsafe impl Send for executable_block {}
unsafe impl Sync for executable_block {}

const PAGE_SIZE: usize = 4096;

impl Index<usize> for executable_block {
    type Output = u8;
    fn index(&self, _index: usize) -> &u8 {
        unsafe {&*self.block.offset(_index as isize) }
    }
}

impl IndexMut<usize> for executable_block {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe {&mut *self.block.offset(_index as isize) }
    }
}

impl executable_block {
    pub fn new(chip8_entry: u16) -> executable_block {
        let block : *mut u8;
        unsafe {
            let cache_size = PAGE_SIZE;
            let mut _block : *mut libc::c_void = mem::uninitialized();
            libc::posix_memalign(&mut _block, PAGE_SIZE, cache_size);
            libc::mprotect(_block, cache_size, libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
            memset(_block, 0xC3, cache_size);
            block = mem::transmute(_block);
        }
        executable_block {
             block: block,
             map: register_map::new(),
             fn_ptr: None,
             chip8_entry: chip8_entry,
             chip8_exit: 0x000,
          }
    }

    pub fn set_exit(&mut self, exit_point: u16){
        self.chip8_exit = exit_point;
    }

    pub fn create_fn_ptr(&mut self) -> (fn(&mut [u8;16]) -> u64) {
         unsafe { mem::transmute(self.block) }
    }

    pub fn assign_fn_ptr(&mut self) -> Option< fn(&mut [u8;16]) -> u64 > {
         self.fn_ptr = Some(self.create_fn_ptr());
         return self.fn_ptr;
    }
}
