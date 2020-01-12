pub fn part_a() -> i32 {
    let data = std::fs::read_to_string("input2.txt").expect("Unable to read file");

    let mut program: Vec<i32> = data.trim()
                                 .split(',')
                                 .map(|s| s.parse().expect(&format!("Invalid entry: {}", s)))
                                 .collect();

    program[1] = 12;
    program[2] = 2;

    program = process_intcode(program);
    
    program[0]
}

pub fn part_b() -> i32 {
    let data = std::fs::read_to_string("input2.txt").expect("Unable to read file");

    let base: Vec<i32> = data.trim()
                                 .split(',')
                                 .map(|s| s.parse().expect(&format!("Invalid entry: {}", s)))
                                 .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = base.clone();
            program[1] = noun;
            program[2] = verb;
            
            program = process_intcode(program);

            if program[0] == 19690720 {
                return 100 * noun + verb;
            }

        }
    }

    -1
}

fn process_intcode(mut program: Vec<i32>) -> Vec<i32>{
    let mut prog_counter = 0;

    loop {
        // println!("prog_counter is {}", prog_counter);
        let opcode = program[prog_counter];

        if opcode == 99 {
            return program;
        }

        let a = program[prog_counter + 1] as usize;
        let b = program[prog_counter + 2] as usize;
        let c = program[prog_counter + 3] as usize;
        // println!("{} ({}, {}, {})", opcode, a, b, c);

        match opcode {
            1 => {
                program[c] = program[a] + program[b];
            },
            2 => {
                program[c] = program[a] * program[b];
            },
            _ => panic!("unimplemented opcode!")
        }

        prog_counter += 4;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut program = vec![1,0,0,0,99];
        program = process_intcode(program);
        assert_eq!(vec![2,0,0,0,99], program);
    }
}