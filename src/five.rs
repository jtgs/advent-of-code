use crate::intcode::*;

pub fn part_a() -> i64 {
    let mut program = Intcode::from_file("input5.txt");

    // Input is '1'
    program.input.push(1);

    program.run();

    info!("Output: {:?}", program.output);

    *program.output.last().unwrap()
}

pub fn part_b() -> i64 {
    let mut program = Intcode::from_file("input5.txt");

    // Input is '5'
    program.input.push(5);

    program.run();

    *program.output.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn five_example_one() {
        init();
        let mut program = Intcode::from("3,9,8,9,10,9,4,9,99,-1,8");
        program.input.push(8);
        program.run();
        assert_eq!(vec![1], program.output);
    }

    #[test]
    fn five_example_two() {
        init();
        let mut program = Intcode::from("3,9,7,9,10,9,4,9,99,-1,8");
        program.input.push(8);
        program.run();
        assert_eq!(vec![0], program.output);
    }

    #[test]
    fn five_example_three() {
        init();
        let mut program = Intcode::from("3,3,1108,-1,8,3,4,3,99");
        program.input.push(88);
        program.run();
        assert_eq!(vec![0], program.output);
    }

    #[test]
    fn five_example_four() {
        init();
        let mut program = Intcode::from("3,3,1107,-1,8,3,4,3,99");
        program.input.push(2);
        program.run();
        assert_eq!(vec![1], program.output);
    }

    #[test]
    fn five_big_example() {
        init();
        let program = Intcode::from("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");

        let mut prog1 = program.clone();
        prog1.input.push(7);
        prog1.run();
        assert_eq!(vec![999], prog1.output);

        let mut prog2 = program.clone();
        prog2.input.push(8);
        prog2.run();
        assert_eq!(vec![1000], prog2.output);

        let mut prog3 = program.clone();
        prog3.input.push(9);
        prog3.run();
        assert_eq!(vec![1001], prog3.output);
    }
}
