use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_char};
use std::sync::Mutex;
use std::borrow::BorrowMut;

#[macro_use]
extern crate lazy_static;

fn get_hello_world() -> String {
    return String::from("ok");
}

lazy_static! {
    static ref CLEANUP_STORAGE: Mutex<Vec<CString>> = Mutex::new(vec![]);
}

fn push_cleanup_stack<'a>(cstr: CString) -> *const i8
{
    let lock = &mut CLEANUP_STORAGE.try_lock();
    if lock.is_ok() {
        println!("try_lock success");
        lock.as_mut().unwrap().push(cstr);
    } else {
        println!("try_lock failed");
    }
    let rust_string: String = String::from("ok");
    let c_string: CString = CString::new(rust_string).expect("Could not convert to CString");
    return c_string.into_raw();
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
    use crate::{get_hello_world, c_hello_world, push_cleanup_stack, CLEANUP_STORAGE};
    use std::ffi::{CString, CStr};
    use std::mem::forget;

    #[test]
    fn it_works() {
        assert_eq!(get_hello_world(), "ok");
    }

    #[test]
    fn test_library_cabi_function_works() {
        let ptr = c_hello_world();
        let cstring;

        unsafe {
            cstring = CString::from_raw(ptr);
        }

        assert_eq!(CString::new("ok").unwrap(), cstring);
    }

    #[test]
    fn test_cleanup_storage() {
        let rust_string: String = String::from("ok");

        // Convert the String into a CString
        let c_string: CString = CString::new(rust_string).expect("Could not convert to CString");
        let raw = push_cleanup_stack(c_string);
        println!("{}", "Trying lock in test");
        let storage = &CLEANUP_STORAGE.lock().unwrap();
        println!("{} with item count {}", "Lock Success", storage.len());
        assert!(storage.get(0).cloned().unwrap()
            .into_string().unwrap()
            .eq("ok"));

        unsafe {
            assert_eq!(CStr::from_ptr(raw).to_str().unwrap(), "ok");
        }

        //assert_eq!(storage.len(), 1);
        //storage.clear();
        //assert_eq!(storage.len(), 0);
    }
}

#[no_mangle]
pub extern "C" fn c_hello_world() -> *mut c_char {
    let rust_string: String = get_hello_world();

    // Convert the String into a CString
    let c_string: CString = CString::new(rust_string).expect("Could not convert to CString");
    let c_element = push_cleanup_stack(c_string);

    return c_element as *mut c_char;
}
