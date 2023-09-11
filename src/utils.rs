use anyhow::Result;
use std::ffi::{CString, c_char};


pub fn nul_terminated_str(s: &str) -> Result<*const c_char> {
    Ok(Box::leak(CString::new(s)?.into_boxed_c_str()).as_ptr())
}
