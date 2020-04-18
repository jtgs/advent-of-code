use std::collections::HashMap;
use std::str;

pub fn part_a() -> i32 {
    let input = std::fs::read_to_string("input8.txt").expect("Unable to read file");
    let layers = split_into_layers(&input, 6, 25);
    let mut counts: Vec<HashMap<char, i32>> = layers.iter().map(|l| count_occurences(l)).collect();

    counts.sort_by(|a, b| a[&'0'].cmp(&b[&'0']));

    counts[0][&'1'] * counts[0][&'2']
}

pub fn part_b() -> i32 {
    unimplemented!()
}

/// Split an input string into a vector of layers of the specified size.
fn split_into_layers(input: &str, height: i32, width: i32) -> Vec<&str> {
    let block_size = (height * width) as usize;

    input
        .as_bytes()
        .chunks(block_size)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
}

/// For a block of input, count the occurrences of each character within and
/// return a HashMap of those counts.
fn count_occurences(block: &str) -> HashMap<char, i32> {
    let mut map: HashMap<char, i32> = HashMap::new();

    for ch in block.chars() {
        map.entry(ch).and_modify(|v| *v += 1).or_insert(1);
    }

    map
}
