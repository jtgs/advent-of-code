use crate::intcode::*;

pub fn part_a() -> i32 {
    let mut program = Intcode::from_file("input5.txt");

    // Input is '1'
    program.input.push(1);

    program.run();

    info!("Output: {:?}", program.output);

    *program.output.last().unwrap()
}

pub fn part_b() -> i32 {
    0
}

