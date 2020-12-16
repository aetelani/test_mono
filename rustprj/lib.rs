use std::ffi::CString;
use std::os::raw::{c_int, c_char};

fn get_hello_world() -> String {
    return String::from("Hello world!");
}

#[no_mangle]
pub extern "C" fn c_meaning_of_life() -> c_int {
    return 42;
}

#[no_mangle]
pub extern "C" fn c_hello_world_free(ptr: *mut c_char) {
    unsafe {
        if ptr.is_null() {
            // No data there, already freed probably.
            return;
        }

        // Here we reclaim ownership of the data the pointer points to, to free the memory properly.
        let str = CString::from_raw(ptr);
        drop(str)
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_hello_world, c_hello_world};
    use std::os::raw::c_char;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        assert_eq!(get_hello_world(), "Hello world!");
    }

    #[test]
    fn test_library_cabi_function_works() {
        let ptr: *mut c_char = c_hello_world();
        let cstring;

        unsafe {
            cstring = CString::from_raw(ptr);
        }

        assert_eq!(CString::new("Hello world!").unwrap(), cstring);
    }
}

#[no_mangle]
pub extern "C" fn c_hello_world() -> *const c_char {
    let rust_string: String = get_hello_world();

    // Convert the String into a CString
    let c_string: CString = CString::new(rust_string).expect("Could not convert to CString");

    // Instead of returning the CString, we return a pointer for it
    return c_string.into_raw();
}
