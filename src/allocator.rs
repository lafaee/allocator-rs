use std::io;
use std::ptr;
use libc::{mmap, munmap, PROT_READ, PROT_WRITE, MAP_ANON, MAP_PRIVATE, c_void, size_t};

pub struct MmapAllocator {
    pub memory_chunks: Vec<(*mut c_void, size_t)>,
}

impl MmapAllocator {
    pub fn new() -> Self {
        MmapAllocator {
            memory_chunks: Vec::new(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Result<*mut c_void, String> {
        let page_size = 4096;
        let aligned_size = (size + page_size - 1) & !(page_size - 1);
        
        unsafe {
            let ptr = mmap(
                ptr::null_mut(),
                aligned_size,
                PROT_READ | PROT_WRITE,
                MAP_ANON | MAP_PRIVATE,
                -1,
                0,
            );
            
            if ptr == libc::MAP_FAILED {
                return Err(format!("Failed to allocate memory: {}", io::Error::last_os_error()));
            }

            self.memory_chunks.push((ptr, aligned_size));
            
            Ok(ptr)
        }
    }
    
    pub fn deallocate(&mut self, ptr: *mut c_void) -> Result<(), String> {
        if let Some(index) = self.memory_chunks.iter().position(|&(p, _)| p == ptr) {
            let (ptr, size) = self.memory_chunks.remove(index);
            
            unsafe {
                if munmap(ptr, size) != 0 {
                    return Err(format!("Failed to deallocate memory: {}", io::Error::last_os_error()));
                }
            }
            
            Ok(())
        } else {
            Err("Pointer not found in allocator".to_string())
        }
    }
    
    pub fn list_allocations(&self) {
        println!("Current allocations:");
        for (i, &(ptr, size)) in self.memory_chunks.iter().enumerate() {
            println!("  {}: Address: {:p}, Size: {} bytes", i, ptr, size);
        }
    }
}

impl Drop for MmapAllocator {
    fn drop(&mut self) {
        for &(ptr, size) in &self.memory_chunks {
            unsafe {
                munmap(ptr, size);
            }
        }
    }
}