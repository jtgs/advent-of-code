#[derive(Debug, Clone)]
// Struct to store an intcode program.
pub struct Intcode {
    pub program: Vec<i32>,
    pub input: Vec<i32>,
    pub output: Vec<i32>
}

impl Intcode {
    pub fn from(input: &str) -> Self {
        Self {
            program: input
                        .trim()
                        .split(',')
                        .map(|s| s.parse().expect(&format!("Invalid entry: {}", s)))
                        .collect(),
            input: Vec::new(),
            output: Vec::new()
        }
    }

    pub fn from_file(name: &str) -> Self {
        let data = std::fs::read_to_string(name).expect("Unable to read file");

        Self::from(&data)
    }

    pub fn process(&mut self) {
        let program = &mut self.program;

        let mut prog_counter = 0;
    
        loop {
            // println!("prog_counter is {}", prog_counter);
            let opcode = program[prog_counter];
    
            if opcode == 99 {
                return;
            }
    
            match opcode {
                1 => {
                    let a = program[prog_counter + 1] as usize;
                    let b = program[prog_counter + 2] as usize;
                    let c = program[prog_counter + 3] as usize;
                    program[c] = program[a] + program[b];
                    prog_counter += 4;
                },
                2 => {
                    let a = program[prog_counter + 1] as usize;
                    let b = program[prog_counter + 2] as usize;
                    let c = program[prog_counter + 3] as usize;
                    program[c] = program[a] * program[b];
                    prog_counter += 4;
                },
                3 => {
                    let a = program[prog_counter + 1] as usize;
                    let input = self.input.remove(0);
                    program[a] = input;
                    prog_counter += 2;
                },
                4 => {
                    let a = program[prog_counter + 1] as usize;
                    self.output.push(program[a]);
                    prog_counter += 2;
                }
                _ => panic!("unimplemented opcode!")
            }
    
        }
    }
}