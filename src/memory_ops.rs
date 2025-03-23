use std::ptr;
use libc::c_void;

pub fn write_memory(ptr: *mut c_void, size: usize, data: &str) -> Result<(), String> {
    let ptr = ptr as *mut u8;
    let data_bytes = data.as_bytes();
    

    let write_size = data_bytes.len().min(size);
    
    unsafe {
        ptr::copy(data_bytes.as_ptr(), ptr, write_size);
    }
    
    Ok(())
}

pub fn read_memory(ptr: *const c_void, size: usize, length: usize) -> Result<String, String> {
    let ptr = ptr as *const u8;
    let safe_length = length.min(size);
    
    let mut buffer = vec![0u8; safe_length];
    unsafe {
        ptr::copy(ptr, buffer.as_mut_ptr(), safe_length);
    }
    
    match String::from_utf8(buffer.clone()) {
        Ok(s) => Ok(s),
        Err(_) => {
            Ok(format!("{:?}", buffer))
        }
    }
}