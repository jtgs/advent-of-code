use itertools::Itertools;
use crate::intcode::*;

fn amplifier(setting: i32, input: i32, mut program: Intcode) -> i32 {
    // Set up the input.
    program.input.push(setting);
    program.input.push(input);

    // Run the program.
    program.run();

    // Retrieve the output.
    program.output[0]
}

fn five_amplifiers(settings: Vec<i32>, program: Intcode) -> i32 {
    let mut value = 0;

    for ii in settings {
        value = amplifier(ii, value, program.clone());
        debug!(">>>>>>>>>> value is {}", value);
    }

    value
}

pub fn part_a() -> i32 {
    let options = (0..5).permutations(5);

    let program = Intcode::from_file("input7.txt");

    let mut top_option = 0;

    for option in options {
        debug!("{:?}", option);
        let output = five_amplifiers(option, program.clone());
        debug!("output: {}", output);

        if output > top_option {
            top_option = output;
        }
    }

    top_option
}

pub fn part_b() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn seven_example_one() {
        init();

        let program = Intcode::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");

        assert_eq!(43210, five_amplifiers(vec![4,3,2,1,0], program));
    }

    #[test]
    fn seven_example_two() {
        init();

        let program = Intcode::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");

        assert_eq!(54321, five_amplifiers(vec![0,1,2,3,4], program));
    }

    #[test]
    fn seven_example_three() {
        init();

        let program = Intcode::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");

        assert_eq!(65210, five_amplifiers(vec![1,0,4,3,2], program));
    }
}
