use crate::intcode::*;

pub fn part_a() -> i32 {
    let mut program = Intcode::from_file("input2.txt");

    program.program[1] = 12;
    program.program[2] = 2;

    program.process();
    
    program.program[0]
}

pub fn part_b() -> i32 {
    let base = Intcode::from_file("input2.txt");

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = base.clone();
            program.program[1] = noun;
            program.program[2] = verb;
            
            program.process();

            if program.program[0] == 19690720 {
                return 100 * noun + verb;
            }

        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut program = Intcode::from("1,0,0,0,99");
        program.process();
        assert_eq!(vec![2,0,0,0,99], program.program);
    }
}