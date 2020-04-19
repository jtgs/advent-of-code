use crate::intcode::*;

pub fn part_a() -> i64 {
    let mut program = Intcode::from_file("input9.txt");
    program.input.push(1);
    program.run();
    debug!("output: {:?}", program.output);

    program.output[0]
}

pub fn part_b() -> i64 {
    let mut program = Intcode::from_file("input9.txt");
    program.input.push(2);
    program.run();
    debug!("output: {:?}", program.output);

    program.output[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn nine_example_one() {
        init();
        let mut program = Intcode::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let output = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        program.run();
        assert_eq!(output, program.output);
    }

    #[test]
    fn nine_example_two() {
        init();
        let mut program = Intcode::from("1102,34915192,34915192,7,4,7,99,0");
        program.run();
        let output = program.output[0].to_string();
        assert_eq!(16, output.chars().count());
    }

    #[test]
    fn nine_example_three() {
        init();
        let mut program = Intcode::from("104,1125899906842624,99");
        program.run();
        assert_eq!(1125899906842624, program.output[0]);
    }

    #[test]
    fn example_203() {
        init();
        let mut program = Intcode::from("203,1,99");
        program.input.push(5);
        program.run();
        assert_eq!(5, program.program[1]);
    }
}