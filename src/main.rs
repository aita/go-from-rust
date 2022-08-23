use std::os::raw::{c_ulong, c_void};
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

unsafe extern "C" fn new_array(len: c_ulong, data: *mut c_void) -> *mut c_void {
    let mut buf = Vec::new();
    for i in 0..len {
        buf.push(*(data as *mut GoInt).offset(i as isize));
    }
    Box::into_raw(Box::new(buf)) as *mut c_void
}

fn main() {
    let array = unsafe { send_array(Some(new_array)) as *mut Vec<i32> };
    let array = unsafe { Box::from_raw(array) };

    println!("{:?}", array);
}
