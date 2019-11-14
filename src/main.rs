use std::io::Read;

struct Engine {
    dp: usize, // data pointer
    memory: Vec<u8>, // memory buffer
    code: Vec<u8>, // instructions
}

impl Engine {
    fn new() -> Self {
        Engine {
            dp: 0,
            memory: Vec::with_capacity(128),
            code: vec![],
        }
    }

    fn execute(&mut self) {
        let mut i = 0;
        while i < self.code.len() {
            match self.code[i] {
                b'>' => self.dp += 1,
                b'<' => self.dp -= 1,
                b'+' => self.memory[self.dp] += 1,
                b'-' => self.memory[self.dp] -= 1,
                b'.' => print!("{}", self.memory[self.dp]),
                b',' => self.memory[self.dp] = read_byte(),
                b'[' => if self.memory[self.dp] == 0 {
                    while self.code[i] != b']' { i += 1; }
                    i += 1;
                },
                b']' => if self.memory[self.dp] != 0 {
                    while self.code[i] != b'[' { i -= 1; }
                    i += 1;
                }
                _ => continue,
            }
            i += 1;
        }
    }
}

fn read_byte() -> u8 {
    use std::io;
    let mut one_buffer = [0u8]; // one element for reading one byte
    if io::stdin().read(&mut one_buffer).unwrap() != 1 {
        panic!("ERROR reading input"); // have to read only one byte
    }
    println!(); // put a new line
    one_buffer[0]
}

fn main() {
    let mut e = Engine::new();
    let a = [0u8; 128];
    e.memory.append(&mut Vec::from(&a[..]));
    //let code = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let code = b"+++>++++>++<<.>.>.";
    e.code = Vec::from(&code[..]);
    e.execute();
    //println!("{} {} {}", e.memory[0], e.memory[1], e.memory[2])
}
