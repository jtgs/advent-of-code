use std::str::FromStr;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::AddAssign;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self(
            self.0 + other.0,
            self.1 + other.1
        );
    }
}

impl Point {
    fn distance(&self) -> i32 {
        self.0 + self.1
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    distance: i32
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let direction = match &input[0..1] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => {
                panic!("Unexpected direction: {}", input);
            }
        };

        let distance = input[1..].parse()?;

        Ok(Self { direction, distance })
    }
}

pub fn part_a() -> i32 {
    let lines: Vec<_> = BufReader::new(File::open("3.txt").expect("Unable to open file"))
        .lines()
        .map(Result::unwrap)
        .collect();

    let line_a: Vec<Instruction> = lines[0].split(',').map(Instruction::from_str).map(Result::unwrap).collect();
    let line_b: Vec<Instruction> = lines[1].split(',').map(Instruction::from_str).map(Result::unwrap).collect();

    let mut points_a: HashSet<Point> = HashSet::new();
    let mut points_b: HashSet<Point> = HashSet::new();

    let mut curr_point = Point(0, 0);

    for instruction in line_a {

        let step = match instruction.direction {
            Direction::Right => Point(1, 0),
            Direction::Left => Point(-1, 0),
            Direction::Up => Point(0, 1),
            Direction::Down => Point(0, -1)
        };

        for _ in 0..instruction.distance {
            curr_point += step; 
            points_a.insert(curr_point);
        }

    };

    for instruction in line_b {

        let step = match instruction.direction {
            Direction::Right => Point(1, 0),
            Direction::Left => Point(-1, 0),
            Direction::Up => Point(0, 1),
            Direction::Down => Point(0, -1)
        };

        for _ in 0..instruction.distance {
            curr_point += step; 
            points_b.insert(curr_point);
        }

    };

    let intersections = points_a.intersection(&points_b).map(Point::clone).collect::<Vec<Point>>();

    let distances: Vec<i32> = intersections.iter().map(Point::distance).collect();

    *distances.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_are_equal() {
        assert_eq!(
            Point(3, 2),
            Point(3, 2)
        );
    }

    #[test]
    fn points_are_not_equal() {
        assert_ne!(
            Point(3, 2),
            Point(2, 3)
        );
    }
}