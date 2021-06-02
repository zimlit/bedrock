use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;
use crate::vm::VM;
use crate::assembler::program_parsers::program;

pub struct REPL {
    command_buffer: Vec<String>,

    vm:VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }
    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError>{
	let split = i.split(" ").collect::<Vec<&str>>();
	let mut results: Vec<u8> = vec![];
	for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
		Ok(result) => {
                    results.push(result);
		},
		Err(e) => {
                    return Err(e);
		}
            }
	}
	Ok(results)
    }

    
    pub fn run(&mut self) {
	loop {
            // This allocates a new String in which to store whatever the user types each iteration.
            // TODO: Figure out how create this outside of the loop and re-use it every iteration
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
            // have to do that there for the user to see our `>>> ` prompt.
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
	    self.command_buffer.push(buffer.to_string());
            match buffer {
		".quit" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
		},
		".history" => {
		    for command in &self.command_buffer {
			println!("{}", command);
		    }
		},
		".program" => {
		    println!("Listing instructions currently in VM's program vector:");
		    for instruction in &self.vm.program {
			println!("{}", instruction);
		    }
		    println!("End of Program Listing");
		},
		".registers" => {
		    println!("Listing registers and all contents:");
		    println!("{:#?}", self.vm.registers);
		    println!("End of Register Listing")
		},
		".heap" => {
		    println!("heap");
		    println!("{:#?}", self.vm.heap);
		},
		"" => {
		    break;
		},
		_ => {
		    // You can assign the result of a match to a variable
		    // Rust can convert types using `Into` and `From`
		    let program = match program(buffer.into()) {
			// Rusts pattern matching is pretty powerful an can even be nested
			Ok((_,  program)) => program,
			Err(_) => {
			    println!("Unable to parse input");
			    continue;
			}
		    };
		    // The `program` is `pub` anyways so you can just `append` to the `Vec`
		    self.vm.program.append(&mut program.to_bytes());
		    self.vm.execute_instruction();
		}
	    }
	}
    }
}
