use std::ffi::CString;
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let c_string: CString = CString::new("Hello Go").unwrap();

    let mut message = Message {
        id: 1,
        text: c_string.as_ptr() as *mut c_char,
    };

    unsafe {
        greeting(&mut message as *mut Message);
    }
}
