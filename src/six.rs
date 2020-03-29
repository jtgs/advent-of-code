use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Object = String;

#[derive(Debug)]
struct Orbit {
    left: Object,
    right: Object
}

impl Orbit {
    fn from_str(input: String) -> Self {
        let parts: Vec<&str> = input.split(')').collect();
        Self {left: parts[0].to_owned(), right: parts[1].to_owned()}
    }
}

struct Node {
    children: Vec<Object>
}

pub fn part_a() -> i32 {
    let orbits: Vec<Orbit> = BufReader::new(File::open("input6.txt").expect("Unable to open file"))
        .lines()
        .map(Result::unwrap)
        .map(Orbit::from_str)
        .collect();

    for orbit in orbits {
        debug!("{:?}", orbit);
    }

    0
}