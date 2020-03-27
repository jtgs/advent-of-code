use crate::intcode::*;

pub fn part_a() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test_of_new_opcodes() {
        let mut program = Intcode::from("3,0,4,0,99");
        program.input.push(256);
        program.process();
        assert_eq!(vec![256], program.output);
    }
}