use crate::intcode::*;

pub fn part_a() -> i32 {
    unimplemented!()
}

pub fn part_b() -> i32 {
    unimplemented!()
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
}