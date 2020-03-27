#[macro_use]
extern crate log;
extern crate env_logger;

// mod one; 
mod two;
// mod three;
// mod four;
mod five;
mod intcode;

fn main() {
    env_logger::init();

    // debug!("Start problem 1");
    // println!("Problem 1a: {}", one::part_a());
    // println!("Problem 1b: {}", one::part_b());
    // debug!("End problem 1");

    debug!("Start problem 2");
    println!("Problem 2a: {}", two::part_a());
    println!("Problem 2b: {}", two::part_b());
    debug!("End problem 2");

    // debug!("Start problem 3");
    // println!("Problem 3a: {}", three::part_a());
    // println!("Problem 3b: {}", three::part_b());
    // debug!("End problem 3");

    // debug!("Start problem 4");
    // println!("Problem 4a: {}", four::part_a());
    // println!("Problem 4b: {}", four::part_b());
    // debug!("End problem 4");

    // debug!("Start problem 5");
    // println!("Problem 5a: {}", five::part_a());
    // println!("Problem 5b: {}", five::part_b());
    // debug!("End problem 5");
}
