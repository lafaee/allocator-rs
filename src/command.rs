use std::io::{self, Write};
use libc::c_void;

use crate::allocator::MmapAllocator;
use crate::memory_ops::{read_memory, write_memory};

pub enum Command {
    Allocate(usize),
    Free(usize),
    List,
    Write(usize, String),
    Read(usize, usize),
    Help,
    Quit,
    Unknown,
}

pub fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    if parts.is_empty() {
        return Command::Unknown;
    }
    
    match parts[0] {
        "alloc" | "a" => {
            if parts.len() < 2 {
                println!("Usage: alloc <size>");
                return Command::Unknown;
            }
            
            match parts[1].parse::<usize>() {
                Ok(size) => Command::Allocate(size),
                Err(_) => {
                    println!("Invalid size");
                    Command::Unknown
                }
            }
        },
        "free" | "f" => {
            if parts.len() < 2 {
                println!("Usage: free <allocation_index>");
                return Command::Unknown;
            }
            
            match parts[1].parse::<usize>() {
                Ok(index) => Command::Free(index),
                Err(_) => {
                    println!("Invalid index");
                    Command::Unknown
                }
            }
        },
        "list" | "l" => Command::List,
        "write" | "w" => {
            if parts.len() < 3 {
                println!("Usage: write <allocation_index> <data>");
                return Command::Unknown;
            }
            
            match parts[1].parse::<usize>() {
                Ok(index) => {
                    let data = parts[2..].join(" ");
                    Command::Write(index, data)
                },
                Err(_) => {
                    println!("Invalid index");
                    Command::Unknown
                }
            }
        },
        "read" | "r" => {
            if parts.len() < 3 {
                println!("Usage: read <allocation_index> <length>");
                return Command::Unknown;
            }
            
            match parts[1].parse::<usize>() {
                Ok(index) => {
                    match parts[2].parse::<usize>() {
                        Ok(length) => Command::Read(index, length),
                        Err(_) => {
                            println!("Invalid length");
                            Command::Unknown
                        }
                    }
                },
                Err(_) => {
                    println!("Invalid index");
                    Command::Unknown
                }
            }
        },
        "quit" | "q" | "exit" => Command::Quit,
        "help" | "h" => Command::Help,
        _ => {
            println!("Unknown command. Type 'help' for available commands.");
            Command::Unknown
        }
    }
}

pub fn process_command(cmd: Command, allocator: &mut MmapAllocator) -> bool {
    match cmd {
        Command::Allocate(size) => {
            match allocator.allocate(size) {
                Ok(ptr) => println!("Allocated {} bytes at address: {:p}", size, ptr),
                Err(e) => println!("Error: {}", e),
            }
            true
        },
        Command::Free(index) => {
            if index >= allocator.memory_chunks.len() {
                println!("Invalid allocation index");
                return true;
            }
            
            let ptr = allocator.memory_chunks[index].0;
            match allocator.deallocate(ptr) {
                Ok(_) => println!("Freed allocation at index {}", index),
                Err(e) => println!("Error: {}", e),
            }
            true
        },
        Command::List => {
            allocator.list_allocations();
            true
        },
        Command::Write(index, data) => {
            if index >= allocator.memory_chunks.len() {
                println!("Invalid allocation index");
                return true;
            }
            
            let (ptr, size) = allocator.memory_chunks[index];
            match write_memory(ptr, size, &data) {
                Ok(_) => println!("Data written to allocation {}", index),
                Err(e) => println!("Error: {}", e),
            }
            true
        },
        Command::Read(index, length) => {
            if index >= allocator.memory_chunks.len() {
                println!("Invalid allocation index");
                return true;
            }
            
            let (ptr, size) = allocator.memory_chunks[index];
            match read_memory(ptr, size, length) {
                Ok(data) => println!("Read from allocation {}: {}", index, data),
                Err(e) => println!("Error: {}", e),
            }
            true
        },
        Command::Help => {
            print_help();
            true
        },
        Command::Quit => false,
        Command::Unknown => true,
    }
}

pub fn read_command() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input
}

fn print_help() {
    println!("Commands:");
    println!("  alloc/a <size>                - Allocate memory of given size");
    println!("  free/f <allocation_index>     - Free memory at given index");
    println!("  list/l                        - List current allocations");
    println!("  write/w <allocation_index> <data> - Write data to allocation");
    println!("  read/r <allocation_index> <length> - Read data from allocation");
    println!("  quit/q/exit                   - Exit program");
    println!("  help/h                        - Show this help");
}