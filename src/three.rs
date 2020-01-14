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
        self.0.abs() + self.1.abs()
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Eq, Hash, Debug)]
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
    let lines: Vec<_> = BufReader::new(File::open("input3.txt").expect("Unable to open file"))
        .lines()
        .map(Result::unwrap)
        .collect();

    solve_a(lines)
}

pub fn solve_a(lines: Vec<String>) -> i32 {
    let (line_a, line_b) = get_instructions(lines);
    let (points_a, points_b) = get_points(&line_a, &line_b);
    let intersections = get_intersections(points_a, points_b);

    let distances: Vec<i32> = intersections.iter().map(Point::distance).collect();

    println!("distances: {:?}", distances);

    *distances.iter().min().unwrap()
}

pub fn part_b() -> i32 {
    let lines: Vec<_> = BufReader::new(File::open("input3.txt").expect("Unable to open file"))
        .lines()
        .map(Result::unwrap)
        .collect();
    
    solve_b(lines)
}

fn solve_b(lines: Vec<String>) -> i32 {
    let (line_a, line_b) = get_instructions(lines);
    let (points_a, points_b) = get_points(&line_a, &line_b);

    let mut steps: Vec<i32> = Vec::new();

    for is in get_intersections(points_a, points_b) {
        println!("{:?}", is);

        let mut steps_a = 0;
        let mut curr_point = Point(0, 0);
        let mut done = false;

        for instruction in &line_a {

            let step = match instruction.direction {
                Direction::Right => Point(1, 0),
                Direction::Left => Point(-1, 0),
                Direction::Up => Point(0, 1),
                Direction::Down => Point(0, -1)
            };
    
            for _ in 0..instruction.distance {
                curr_point += step;
                steps_a += 1;
                if curr_point == is {
                    done = true;
                    break;
                }
            }

            if done {
                break;
            }
    
        };

        let mut steps_b = 0;
        let mut curr_point = Point(0, 0);
        let mut done = false;

        for instruction in &line_b {

            let step = match instruction.direction {
                Direction::Right => Point(1, 0),
                Direction::Left => Point(-1, 0),
                Direction::Up => Point(0, 1),
                Direction::Down => Point(0, -1)
            };
    
            for _ in 0..instruction.distance {
                curr_point += step;
                steps_b += 1;
                if curr_point == is {
                    done = true;
                    break;
                }
            }

            if done {
                break;
            }
    
        };

        steps.push(steps_a + steps_b); 
    }

    *steps.iter().min().unwrap()
}

fn get_instructions(lines: Vec<String>) -> (Vec<Instruction>, Vec<Instruction>) {
    let line_a: Vec<Instruction> = lines[0].split(',').map(Instruction::from_str).map(Result::unwrap).collect();
    // println!("{:?}", line_a);
    let line_b: Vec<Instruction> = lines[1].split(',').map(Instruction::from_str).map(Result::unwrap).collect();
    // println!("{:?}", line_b);

    (line_a, line_b)
}

fn get_points(line_a: &Vec<Instruction>, line_b: &Vec<Instruction>) -> (HashSet<Point>, HashSet<Point>) {

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

    // println!("points_a: {:?}", points_a);

    let mut curr_point = Point(0, 0);

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

    // println!("points_b: {:?}", points_b);

    (points_a, points_b)
}

fn get_intersections(points_a: HashSet<Point>, points_b: HashSet<Point>) -> Vec<Point> {
    let intersections = points_a.intersection(&points_b).collect::<Vec<&Point>>();
    // println!("intersections: {:?}", intersections);

    let intersections = points_a.intersection(&points_b).map(Point::clone).collect::<Vec<Point>>();
    // println!("intersections: {:?}", intersections);

    intersections
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

    #[test]
    fn point_distance() {
        assert_eq!(
            Point(3, 2).distance(),
            5
        )
    }

    #[test]
    fn point_distance_negative() {
        assert_eq!(
            Point(3, -2).distance(),
            5
        )
    }

    #[test]
    fn point_distance_map() {
        let points = vec![Point(3, 2), Point(4, 5)];
        let distances: Vec<i32> = points.iter().map(Point::distance).collect();
        assert_eq!(distances[0], 5);
        assert_eq!(distances[1], 9);
    }

    #[test]
    fn simple_example() {
        assert_eq!(
            solve_a(vec!["U3,R3".to_string(), "R3,U3".to_string()]),
            6
        )
    }

    #[test]
    fn example_one() {
        assert_eq!(
            solve_a(vec!["R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(), "U62,R66,U55,R34,D71,R55,D58,R83".to_string()]),
            159
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            solve_a(vec!["R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string(), "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string()]),
            135
        );
    }

    #[test]
    fn simple_example_b() {
        assert_eq!(
            solve_b(vec!["R8,U5,L5,D3".to_string(), "U7,R6,D4,L4".to_string()]),
            30
        )
    }
}