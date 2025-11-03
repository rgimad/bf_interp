// BF++ interpreter
// Grammar:
// + inc current cell
// - dec current cell
// > move to next cell
// < move to previous cell
// , read current cell from stdin
// . print current cell as ASCII char
// [ code ] while(mem[pc]) { code } 

// TODO make struct of context and impl for interpreter

use std::env;
use std::fs;
use std::process;
use std::io::Write;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    //let program_name : String = args[0].split(&['\\', '/'][..]).collect().last().unwrap_or_default();
    // println!("{:?}", args);
    if args.len() < 2 {
        println!("No arguments. Usage: bf <file>");
        process::exit(1);
    }
    let filename = &args[1];
    let file_content = match fs::read_to_string(filename) {
        Ok(s) => {s},
        Err(e) => {
            println!("Error reading file:\n{}", e);
            process::exit(2);
        }
    };
    let code = file_content.as_bytes();

    let mut mem = [0 as i8; 30000];
    let mut pc  = 0usize; // program counter
    let mut dc  = 0usize; // data counter
    let mut stack= Vec::<usize>::new(); // stack of positions of loop beginnings
    while pc < code.len() {
        match code[pc] {
            b'+' => {
                //mem[dc] += 1;
                mem[dc] = mem[dc].wrapping_add(1); // add with overflow. 128 -> -128, 129 -> -127 etc.
            },
            b'-' => {
                //mem[dc] -= 1;
                mem[dc] = mem[dc].wrapping_sub(1); // subract with overflow
            },
            b'>' => { dc += 1; },
            b'<' => { dc -= 1; },
            b',' => {
                let inp = std::io::stdin().bytes().next().unwrap().unwrap() as i8;
                mem[dc] = if inp == b'\r' as i8 {b'\n' as i8} else {inp}; // convert CRLF to LF
            },
            b'.' => {
                print!("{}", mem[dc] as u8 as char);
                std::io::stdout().flush().unwrap(); // flush makes chars immediatly appear on the screen
            },
            b'[' => {
                let mut cnt = 0;
                let mut close_found = false;
                for j in pc..code.len() {
                    match code[j] {
                        b'[' => { cnt += 1; },
                        b']' => { if cnt > 0 {cnt -= 1;} else { panic!("unpaired brackets"); } },
                        _ => {}
                    }
                    if cnt == 0 { // we've found close brace
                        if mem[dc] != 0 { // if current cell is not zero then start loop
                            stack.push(pc + 1); // save loop's first command address
                            // println!("loop added {}", pc + 1)
                        } else { // skip loop
                            pc = j;
                        }
                        close_found = true;
                        break;
                    }
                }
                if !close_found { panic!("unpaired brackets"); }
            },
            b']' => {
                if !stack.is_empty() {
                    if mem[dc] != 0 {
                        pc = stack[stack.len() - 1]; // jump to loop beginning
                        continue; // for not to increment if after match
                    } else {
                        stack.pop().unwrap();
                        // println!("loop deleted");
                    }
                } else { panic!("Unexpected ] pc = {}", pc); }
             },
            _ => {}
        }
        pc += 1;
    }
}
