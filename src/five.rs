use crate::intcode::*;

pub fn part_a() -> i32 {
    let mut program = Intcode::from_file("input5.txt");

    // Input is '1'
    program.input.push(1);

    program.run();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test_of_new_opcodes() {
        let mut program = Intcode::from("3,0,4,0,99");
        program.input.push(256);
        program.run();
        assert_eq!(vec![256], program.output);
    }
}