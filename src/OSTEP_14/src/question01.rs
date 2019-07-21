use libc::c_int as int;
use libc::free;
use libc::malloc;
use std::mem::size_of as sizeof;

pub fn free_null_pointer() {
    unsafe {
        // let ptr: *mut libc::c_int = libc::malloc(sizeof::<int>() * 8) as *mut i32;
        let ptr: *mut int = std::ptr::null::<int>() as *mut i32;
        println!("ptr is null: {}", ptr.is_null());
        libc::free(ptr as *mut core::ffi::c_void);
    }
}
