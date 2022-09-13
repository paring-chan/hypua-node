extern crate alloc;
extern crate hypua;
extern crate libc;

use alloc::ffi::CString;

#[no_mangle]
pub extern "C" fn to_ipf_string(arg1: *const libc::c_char) -> *const libc::c_char {
    let s = unsafe { std::ffi::CStr::from_ptr(arg1) }.to_str().unwrap();

    let ipf = hypua::to_ipf_string(s);

    let v = ipf.as_bytes();

    CString::new(v).unwrap().into_raw()
}
