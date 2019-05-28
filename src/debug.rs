//! The native debug interface exposed to the ewasm contract. These functions are for testing
//! purposes only. On a live VM, any bytecode trying to import these symbols will be rejected.

use crate::types::*;

mod native {
    extern "C" {
        pub fn debug_print32(value: u32);
        pub fn debug_print64(value: u64);
        pub fn debug_printMem(offset: *const u32, len: u32);
        pub fn debug_printMemHex(offset: *const u32, len: u32);
        pub fn debug_printStorage(pathOffset: *const u32);
        pub fn debug_printStorageHex(pathOffset: *const u32);
    }
}

pub fn print32(value: u32) {
    unsafe { native::debug_print32(value) }
}

pub fn print64(value: u64) {
    unsafe { native::debug_print64(value) }
}

pub fn print_mem(slice: &[u8]) {
    unsafe { native::debug_printMem(slice.as_ptr() as *const u32, slice.len() as u32) }
}

pub fn print_mem_hex(slice: &[u8]) {
    unsafe { native::debug_printMemHex(slice.as_ptr() as *const u32, slice.len() as u32) }
}

pub fn print_storage(key: &StorageKey) {
    unsafe { native::debug_printStorage(key.bytes.as_ptr() as *const u32) }
}

pub fn print_storage_hex(key: &StorageKey) {
    unsafe { native::debug_printStorageHex(key.bytes.as_ptr() as *const u32) }
}
