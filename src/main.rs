use std::env;
use std::fs;
use std::process;

use libbfside::{parse, Interpreter};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    let file_content = match fs::read(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(2);
        }
    };
    
    let program = match parse(&file_content) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(3);
        }
    };
    
    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.run(&program) {
        eprintln!("Runtime error: {}", e);
        process::exit(4);
    }
}
