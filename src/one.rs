use std::fs::File;
use std::io::{BufRead, BufReader};

fn fuel_for_module(mass: isize) -> isize {
    let divided = (mass as f64 / 3f64).floor();
    divided as isize - 2
}

fn full_fuel_for_module(mut mass: isize) -> isize {
    let mut fuel = 0;

    loop {
        let extra = fuel_for_module(mass);

        if extra < 0 {
            return fuel;
        } else {
            fuel += extra;
            mass = extra;
        }
    }
}

pub fn part_a() -> isize {
    let filename = "input1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let x = line.unwrap().trim().parse().unwrap();
        total += fuel_for_module(x);
    }

    total
}

pub fn part_b() -> isize {
    let filename = "input1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let x = line.unwrap().trim().parse().unwrap();
        total += full_fuel_for_module(x);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(2, fuel_for_module(12));
    }

    #[test]
    fn example_two() {
        assert_eq!(2, fuel_for_module(14));
    }

    #[test]
    fn example_three() {
        assert_eq!(654, fuel_for_module(1969));
    }

    #[test]
    fn example_four() {
        assert_eq!(33583, fuel_for_module(100756));
    }
}
