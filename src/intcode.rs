#[derive(Debug, Clone, PartialEq)]
enum Opcode {
    Add,          // 1
    Multiply,     // 2
    StoreInput,   // 3
    PushOutput,   // 4
    JumpIfTrue,   // 5
    JumpIfFalse,  // 6
    Halt          // 99
}

#[derive(Debug)]
enum ParamMode {
    Position,     // 0
    Immediate,    // 1
    Reference,    // TODO: understand how this is position
}

#[derive(PartialEq, Debug)]
enum StepResult {
    Halt,
    Continue
}

#[derive(Debug)]
struct Operation {
    pub opcode: Opcode,
    pub num_params: i32,
    pub param_modes: Vec<ParamMode>
}

/// Parses an integer representing an opcode into an Opcode and vector of 
/// ParamModes. 
fn parse_opcode(input: i32) -> (Opcode, Vec<ParamMode>) {
    debug!("input: {}", input);

    let opcode = match input % 100 {
        1  => Opcode::Add,
        2  => Opcode::Multiply,
        3  => Opcode::StoreInput,
        4  => Opcode::PushOutput,
        5  => Opcode::JumpIfTrue,
        6  => Opcode::JumpIfFalse,
        99 => Opcode::Halt,
        _  => panic!("Unsupported opcode!")
    };
    debug!("opcode: {:?}", opcode);

    // Assume for now we're only going to get two parameters that might be 
    // immediate. (See the match below for why.)
    let param1 = match (input / 100) % 10 == 1 {
        true => ParamMode::Immediate,
        false => ParamMode::Position,
    };
    let param2 = match (input / 1000) % 10 == 1 {
        true => ParamMode::Immediate,
        false => ParamMode::Position,
    };
    debug!("param1: {:?}, param2: {:?}", param1, param2);

    let param_modes = match opcode {
        Opcode::Add | Opcode::Multiply => {
            // Params 1 and 2 can be Position or Immediate.
            // Param 3 provides the reference to write to.
            vec!(param1, param2, ParamMode::Reference)
        },
        Opcode::StoreInput => {
            // The single param here is the destination of the input. 
            vec!(ParamMode::Reference)
        },
        Opcode::PushOutput => {
            // This has a single parameter which could be immediate or position.
            vec!(param1)
        },
        Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
            // First parameter can be Position or Immediate;
            // second parameter is a reference
            vec!(param1, ParamMode::Reference)
        },
        Opcode::Halt => {
            // This has no parameters.
            Vec::new()
        }
    };
    debug!("param_modes: {:?}", param_modes);

    (opcode, param_modes)
}

impl Operation {
    /// Builds a new Operation from the provided program, starting at the point 
    /// indicated by the program counter (pc).
    pub fn from(program: &Vec<i32>, pc: i32) -> Self {
        debug!("New Operation from position {}", pc);
        let (opcode, param_modes) = parse_opcode(program[pc as usize]);

        // Work out how many parameters we need.
        let num_params = match opcode {
            Opcode::Add | Opcode::Multiply => 3,
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => 2,
            Opcode::StoreInput | Opcode::PushOutput => 1,
            Opcode::Halt => 0,
        };
        debug!("  no. params: {}", num_params);

        Self {opcode, num_params, param_modes}
    }

    /// Given a whole program, and the position of this Operation within it,
    /// works out what the parameters are for this Operation.
    pub fn get_params(&self, program: &Vec<i32>, pc: i32) -> Vec<i32> {
        let mut params: Vec<i32> = Vec::new();

        for ii in 0..self.num_params as usize {
            match self.param_modes[ii] {
                ParamMode::Position => {
                    // This is the number at the position indicated.
                    let index = program[pc as usize + ii + 1] as usize;
                    params.push(program[index]);
                },
                ParamMode::Immediate | ParamMode::Reference => {
                    // This is just the literal number in the parameter.
                    params.push(program[pc as usize + ii + 1]);
                }
            }
        }

        debug!("got params: {:?}", params);

        params
    }
}

// Struct to store an intcode program.
#[derive(Debug, Clone)]
pub struct Intcode {
    pub program: Vec<i32>,
    pub input: Vec<i32>,
    pub output: Vec<i32>,
    pc: i32
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
            output: Vec::new(),
            pc: 0
        }
    }

    pub fn from_file(name: &str) -> Self {
        let data = std::fs::read_to_string(name).expect("Unable to read file");

        Self::from(&data)
    }
    
    /// Perform a single operation, starting at the program counter (pc). 
    /// 
    /// Returns a StepResult (Halt or Continue). 
    fn step(&mut self) -> StepResult {
        let program = &mut self.program;

        // Calculate what the next operation is. 
        let op = Operation::from(program, self.pc);

        // Get the parameters. This deals with parameter modes so that the 
        // value in this vector is the one we need below.
        let params = op.get_params(program, self.pc);

        let mut result = StepResult::Continue;
        let mut pc_moved = false;

        // Perform the operation.
        match op.opcode {
            Opcode::Add => {
                // Add the first to the second, store in the third.
                debug!("{} + {} -> [{}]", params[0], params[1], params[2]);
                program[params[2] as usize] = params[0] + params[1];
            },
            Opcode::Multiply => {
                // Multiply the first and the second, store in the third.
                debug!("{} * {} -> [{}]", params[0], params[1], params[2]);
                program[params[2] as usize] = params[0] * params[1];
            },
            Opcode::StoreInput => {
                // Get the first value off the input stack; store it in the 
                // cell indicated by the first parameter.
                let input = self.input.remove(0);
                debug!("{} -> [{}]", input, params[0]);
                program[params[0] as usize] = input;
            },
            Opcode::PushOutput => {
                // Push the first parameter to the output stack.
                debug!("{} -> output", params[0]);
                self.output.push(params[0]);
            },
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => {
                // If the condition is met by the first param, move the 
                // instruction pointer to the second param.
                debug!("{:?} : {}?", op.opcode, params[0]);

                let condition: bool;

                // TODO: would be nice to do this without an if statement
                if op.opcode == Opcode::JumpIfTrue {
                    condition = params[0] != 0;
                } else {  // JumpIfFalse
                    condition = params[0] == 0;
                }

                if condition {
                    debug!("Set PC to {}", params[1]);
                    self.pc = params[1];
                    pc_moved = true;
                }
            }, 
            Opcode::Halt => {
                debug!("Halt!");
                result = StepResult::Halt;
            }
        };

        // Advance the program counter, if it hasn't already changed.
        if !pc_moved {
            debug!("Advancing PC");
            self.pc += op.num_params + 1;
        }
        debug!("PC is now {}", self.pc);

        debug!("Returning {:?}", result);
        result
    }

    /// Runs step-by-step until it encounters a Halt.
    pub fn run(&mut self) {
        let mut result = StepResult::Continue;

        while result != StepResult::Halt {
            result = self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn simple_test_of_io_opcodes() {
        init();
        let mut program = Intcode::from("3,0,4,0,99");
        program.input.push(256);
        program.run();
        assert_eq!(vec![256], program.output);
    }

    #[test]
    fn test_jump_if_true_1() {
        init();
        let mut program = Intcode::from("1105,2,77,99");
        program.step();
        assert_eq!(77, program.pc);
    }

    #[test]
    fn test_jump_if_true_2() {
        init();
        let mut program = Intcode::from("1105,0,77,99");
        program.step();
        assert_eq!(3, program.pc);
    }

    #[test]
    fn test_jump_if_false_1() {
        init();
        let mut program = Intcode::from("1106,0,77,99");
        program.step();
        assert_eq!(77, program.pc);
    }

    #[test]
    fn test_jump_if_false_2() {
        init();
        let mut program = Intcode::from("1106,2,77,99");
        program.step();
        assert_eq!(3, program.pc);
    }
}