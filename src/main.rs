use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
struct Message {
    id: i32,
    text: *const c_char,
}

extern "C" {
    fn greeting(message: *const Message);
}

fn main() {
    let c_string: CString = CString::new("Hello Go").unwrap();

    let message = Message {
        id: 1,
        text: c_string.as_ptr(),
    };

    unsafe {
        greeting(&message as *const Message);
    }
}
