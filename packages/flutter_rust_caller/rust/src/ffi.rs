use crate::call::call;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Go-style FFI export for call
/// Converts C strings to Rust strings and back
/// # Safety
/// The caller must ensure that the pointers are valid and properly allocated.
#[no_mangle]
pub extern "C" fn rust_call(
    method: *const c_char,
    param_json: *const c_char,
) -> *mut c_char {
    if method.is_null() || param_json.is_null() {
        let error = serde_json::json!({ "error": "Null pointer passed to rust_call" }).to_string();
        return CString::new(error)
            .map(|s| s.into_raw())
            .unwrap_or(std::ptr::null_mut());
    }

    let result = std::panic::catch_unwind(|| {
        // Convert C strings to Rust strings
        let method_str = unsafe { CStr::from_ptr(method).to_string_lossy() };
        let param_json_str = unsafe { CStr::from_ptr(param_json).to_string_lossy() };

        // Call the main function
        call(&method_str, &param_json_str)
    });

    match result {
        Ok(json_result) => {
            // Convert Rust string to C string
            CString::new(json_result)
                .map(|s| s.into_raw())
                .unwrap_or_else(|_| {
                    let error = serde_json::json!({ "error": "Failed to convert result to C string" }).to_string();
                    CString::new(error)
                        .map(|s| s.into_raw())
                        .unwrap_or(std::ptr::null_mut())
                })
        }
        Err(_) => {
            let error = serde_json::json!({ "error": "Panicked during execution" }).to_string();
            CString::new(error)
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

/// Free a C string allocated by Rust
/// # Safety
/// The caller must ensure that the pointer was allocated by rust_call or is null.
#[no_mangle]
pub extern "C" fn rust_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_call_with_valid_params() {
        let method = CString::new("Multiply").unwrap();
        let params = CString::new(r#"{"base": 10, "multiplier": 20}"#).unwrap();

        let result_ptr = rust_call(method.as_ptr(), params.as_ptr());
        assert!(!result_ptr.is_null());

        let result_str = unsafe { CStr::from_ptr(result_ptr).to_string_lossy() };
        assert!(result_str.contains("200"));

        // Clean up
        rust_free_string(result_ptr);
    }

    #[test]
    fn test_rust_call_with_null_pointer() {
        let method = CString::new("Multiply").unwrap();
        let result_ptr = rust_call(std::ptr::null(), method.as_ptr());
        assert!(!result_ptr.is_null());

        let result_str = unsafe { CStr::from_ptr(result_ptr).to_string_lossy() };
        assert!(result_str.contains("error"));

        // Clean up
        rust_free_string(result_ptr);
    }

    #[test]
    fn test_rust_free_string() {
        let s = CString::new("test").unwrap();
        let ptr = s.into_raw();
        // Should not panic
        rust_free_string(ptr);
    }

    #[test]
    fn test_rust_free_string_null() {
        // Should not panic with null pointer
        rust_free_string(std::ptr::null_mut());
    }
}
