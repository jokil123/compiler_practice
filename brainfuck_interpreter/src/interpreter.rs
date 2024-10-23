use std::{
    io::{self, Read, Stdin, Write},
    slice,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret_brainfuck(bf: &[u8]) {
        // const MEM_WIDTH: usize = 100;
        const MEM_WIDTH: usize = 20;

        let mut mem: [i16; MEM_WIDTH] = [0; MEM_WIDTH];
        let mut mem_ptr = 0;

        let program_length = bf.len();
        let mut program_counter = 0;

        let pairs = Interpreter::cache_brackets(bf);

        loop {
            if program_counter > program_length - 1 {
                return;
            }

            match bf[program_counter] as char {
                '>' => {
                    if mem_ptr >= MEM_WIDTH - 1 {
                        mem_ptr = 0;
                    } else {
                        mem_ptr += 1;
                    }
                }
                '<' => {
                    if mem_ptr == 0 {
                        mem_ptr = MEM_WIDTH - 1;
                    } else {
                        mem_ptr -= 1;
                    }
                }
                '+' => mem[mem_ptr] += 1,
                '-' => mem[mem_ptr] -= 1,
                '.' => Interpreter::printCell(mem[mem_ptr]),
                ',' => ,
                '[' => {
                    if mem[mem_ptr] == 0 {
                        program_counter = pairs[program_counter] + 1;
                    }
                }
                ']' => {
                    if mem[mem_ptr] != 0 {
                        program_counter = pairs[program_counter];
                    }
                }
                _ => (),
            }

            // println!(
            //     "mem_ptr: {}, program_counter: {}, instruction: {}",
            //     mem_ptr, program_counter, bf[program_counter] as char
            // );
            // println!("{:?}", mem);

            program_counter += 1;
        }
    }

    fn printCell<T>(c: T) {
        io::stdout().write(c.into());
    }

    fn readCell() {
        _ = io::stdin().read(slice::from_mut(&mut mem[mem_ptr]))
    }

    pub fn cache_brackets(s: &[u8]) -> Vec<usize> {
        let mut pairs: Vec<usize> = vec![0; s.len()];

        let mut indent_stack: Vec<usize> = vec![0; s.len()];
        let mut indent_level = 0;

        for i in 0..s.len() {
            match s[i] as char {
                '[' => {
                    indent_level += 1;
                    indent_stack[indent_level] = i;
                }
                ']' => {
                    pairs[i] = indent_stack[indent_level];
                    pairs[indent_stack[indent_level]] = i;
                    indent_level -= 1;
                }
                _ => (),
            }
        }

        return pairs;
    }
}
