extern crate jwconv;


use std::ffi::{CStr, CString};
use std::os::raw::c_char;


macro_rules! cchar_to_str {
    ($x:ident) => ({
        let ptr = unsafe {
            assert!(!$x.is_null());
            CStr::from_ptr($x)
        };
        ptr.to_str().unwrap()
    })
}

macro_rules! str_to_cchar {
    ($x:expr) => ({
        CString::new($x).unwrap().into_raw()
    })
}


#[no_mangle]
pub extern fn romaji_to_hiragana(data: *const c_char) -> *const c_char {
    let data = cchar_to_str!(data);
    let result = jwconv::romaji_to_hiragana(data);
    str_to_cchar!(result)
}

#[no_mangle]
pub extern fn romaji_to_katakana(data: *const c_char) -> *const c_char {
    let data = cchar_to_str!(data);
    let result = jwconv::romaji_to_katakana(data);
    str_to_cchar!(result)
}

#[no_mangle]
pub extern fn string_free(ptr: *mut c_char) {
    unsafe {
        if ptr.is_null() { return }
        CString::from_raw(ptr)
    };
}
