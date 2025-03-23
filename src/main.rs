mod allocator;
mod memory_ops;
mod command;

use allocator::MmapAllocator;
use command::{read_command, parse_command, process_command};

fn main() {
    let mut allocator = MmapAllocator::new();
    
    println!("Rust mmap Memory Allocator");
    println!("Type 'help' for available commands");
    
    loop {
        let input = read_command();
        let cmd = parse_command(&input);
        
        if !process_command(cmd, &mut allocator) {
            println!("Exiting program");
            break;
        }
    }
}