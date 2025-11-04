use std::io::{self, Read, Write};

#[derive(Debug, PartialEq)]
pub enum Op {
    Inc(usize),
    Dec(usize),  
    Next(usize),
    Prev(usize),
    Read,
    Print,
    Loop(Vec<Op>),
}

pub struct Interpreter {
    memory: [i8; 30000],
    data_ptr: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: [0; 30000],
            data_ptr: 0,
        }
    }

    pub fn run(&mut self, program: &[Op]) -> Result<(), String> {
        self.execute(program)?;
        Ok(())
    }

    fn execute(&mut self, program: &[Op]) -> Result<(), String> {
        for op in program {
            match op {
                Op::Inc(count) => {
                    for _ in 0..*count {
                        self.memory[self.data_ptr] = self.memory[self.data_ptr].wrapping_add(1);
                    }
                }
                Op::Dec(count) => {
                    for _ in 0..*count {
                        self.memory[self.data_ptr] = self.memory[self.data_ptr].wrapping_sub(1);
                    }
                }
                Op::Next(count) => {
                    if self.data_ptr + count >= self.memory.len() {
                        return Err("Data pointer overflow".to_string());
                    }
                    self.data_ptr += count;
                }
                Op::Prev(count) => {
                    if *count > self.data_ptr {
                        return Err("Data pointer underflow".to_string());
                    }
                    self.data_ptr -= count;
                }
                Op::Read => {
                    let mut input = [0u8; 1];
                    if let Ok(()) = io::stdin().read_exact(&mut input) {
                        self.memory[self.data_ptr] = if input[0] == b'\r' {
                            b'\n' as i8
                        } else {
                            input[0] as i8
                        };
                    }
                }
                Op::Print => {
                    print!("{}", self.memory[self.data_ptr] as u8 as char);
                    io::stdout().flush().map_err(|e| e.to_string())?;
                }
                Op::Loop(body) => {
                    while self.memory[self.data_ptr] != 0 {
                        self.execute(body)?;
                    }
                }
            }
        }
        Ok(())
    }

    // Helper method for tests to inspect memory
    pub fn get_memory(&self) -> &[i8] {
        &self.memory
    }

    // Helper method for tests to set memory
    pub fn set_memory(&mut self, addr: usize, value: i8) {
        self.memory[addr] = value;
    }

    // Helper to get data pointer for tests
    pub fn get_data_ptr(&self) -> usize {
        self.data_ptr
    }

    // Helper to set data pointer for tests
    pub fn set_data_ptr(&mut self, ptr: usize) {
        self.data_ptr = ptr;
    }
}

pub fn parse(code: &[u8]) -> Result<Vec<Op>, String> {
    let mut program = Vec::new();
    let mut i = 0;
    
    while i < code.len() {
        match code[i] {
            b'+' | b'-' | b'>' | b'<' => {
                let start = i;
                let op_char = code[i];
                
                while i < code.len() && code[i] == op_char {
                    i += 1;
                }
                
                let count = i - start;
                
                match op_char {
                    b'+' => program.push(Op::Inc(count)),
                    b'-' => program.push(Op::Dec(count)),
                    b'>' => program.push(Op::Next(count)),
                    b'<' => program.push(Op::Prev(count)),
                    _ => unreachable!(),
                }
            }
            b',' => {
                program.push(Op::Read);
                i += 1;
            }
            b'.' => {
                program.push(Op::Print);
                i += 1;
            }
            b'[' => {
                let mut balance = 1;
                let start = i + 1;
                let mut j = i + 1;
                
                while j < code.len() && balance > 0 {
                    match code[j] {
                        b'[' => balance += 1,
                        b']' => balance -= 1,
                        _ => {}
                    }
                    if balance == 0 {
                        break;
                    }
                    j += 1;
                }
                
                if balance != 0 {
                    return Err("Unmatched opening bracket".to_string());
                }
                
                let body = parse(&code[start..j])?;
                program.push(Op::Loop(body));
                i = j + 1;
            }
            b']' => {
                return Err("Unexpected closing bracket".to_string());
            }
            _ => {
                i += 1;
            }
        }
    }
    
    Ok(program)
}
