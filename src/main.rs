use std::os::raw::{c_int, c_ulong, c_void};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let mut buf = Vec::new();
    let mut closure = |size: c_ulong, arr: *mut c_int| {
        for i in 0..size {
            buf.push(unsafe { *arr.offset(i as isize) });
        }
    };

    let (state, callback) = unsafe { ffi_helpers::split_closure(&mut closure) };

    unsafe {
        send_array(state, Some(callback));
    }

    println!("{:?}", buf);
}
