use std::io::Read;
use std::io;
use std::env;
use std::fs::File;
use std::process::exit;

struct Engine {
    dp: usize,          // data pointer
    ip: usize,          // instruction pointer
    memory: Vec<u8>,    // memory buffer
    code: Vec<u8>,      // instructions
}

impl Engine {
    fn new() -> Self {
        let mut memory = Vec::with_capacity(30000);
        memory.push(0u8);
        Engine {
            dp: 0,
            ip: 0,
            code: vec![],
            memory,
        }
    }

    fn execute(&mut self) {
        while self.ip < self.code.len() {
            match self.code[self.ip] {
                b'>' => {
                    if self.dp == self.memory.len() - 1 {
                        self.memory.push(0);
                    }
                    self.dp += 1
                },
                b'<' => {
                    if self.dp == 0 {
                        self.dp = self.memory.len() - 1;
                    }
                    self.dp -= 1
                },
                b'+' => self.memory[self.dp] += 1,
                b'-' => self.memory[self.dp] -= 1,
                b'.' => print!("{}", (self.memory[self.dp]) as char),
                b',' => self.memory[self.dp] = read_byte(),
                b'[' => if self.memory[self.dp] == 0 {
                    let mut temp = 0;
                    self.ip += 1;
                    while self.code[self.ip] != b']' || temp != 0 {
                        if self.code[self.ip] == b'[' {
                            temp += 1;
                        } else if self.code[self.ip] == b']' {
                            temp -= 1;
                        }
                        self.ip += 1;
                    }
                },
                b']' => if self.memory[self.dp] != 0 {
                    let mut temp = 0;
                    self.ip -= 1;
                    while self.code[self.ip] != b'[' || temp != 0 {
                        if self.code[self.ip] == b']' {
                            temp += 1;
                        } else if self.code[self.ip] == b'[' {
                            temp -= 1;
                        }
                        self.ip -= 1;
                    }
                }
                _ => (),
            }
            self.ip += 1;
        }
    }
}

fn read_byte() -> u8 {
    let mut one_buffer = [0u8]; // one element for reading one byte
    if io::stdin().read(&mut one_buffer).unwrap() != 1 {
        panic!("ERROR reading input"); // have to read only one byte
    }
    println!(); // put a new line
    one_buffer[0]
}

fn help() {
    println!(r"
    USAGE:
        ./bf <file|path>
    ");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Argument error");
        help();
        exit(-1)
    }

    let path = &args[1];
    let mut code = String::new();

    File::open(path)
        .unwrap_or_else(|_| {
            println!("File {} not found", path);
            exit(-1);
        })
        .read_to_string(&mut code)
        .expect("Can not read the file");

    let mut e = Engine::new();
    e.code = code.into_bytes();
    e.execute();
}
