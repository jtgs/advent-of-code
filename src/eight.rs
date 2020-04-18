use std::collections::HashMap;
use std::str;

const BLACK: char = '0';
const WHITE: char = '1';
const TRANSPARENT: char = '2';

pub fn part_a() -> i32 {
    let input = std::fs::read_to_string("input8.txt").expect("Unable to read file");
    let layers = split_into_layers(&input, 6, 25);
    let mut counts: Vec<HashMap<char, i32>> = layers.iter().map(|l| count_occurences(l)).collect();

    counts.sort_by(|a, b| a[&BLACK].cmp(&b[&BLACK]));

    counts[0][&WHITE] * counts[0][&TRANSPARENT]
}

pub fn part_b() {
    let input = std::fs::read_to_string("input8.txt").expect("Unable to read file");
    let layers = split_into_layers(&input, 6, 25);
    debug!("layers: {}", layers.len());
    let pixels = stack_pixels(layers);

    for row in pixels_to_grid(&pixels, 25) {
        println!("{:?}", row);
    }
}
/// Turn a string of pixels into a vector of strings, each the width of the 
/// image, suitable to be printed out one-by-one to form the image.
fn pixels_to_grid(pixels: &str, width: i32) -> Vec<&str> {
    pixels
        .as_bytes()
        .chunks(width as usize)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
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

/// Given a vector of image layers, work out the value of each pixel by stacking 
/// the layers.
/// 
/// To do this, all we have to do is, for each pixel, run through the values 
/// from each layer and stop when we find one that's not transparent.
fn stack_pixels(layers: Vec<&str>) -> String {
    let mut result: String = "".to_string();

    // Assume that all of the layers are the same length (this should be the size
    // of the image). If that's not true, then we're in trouble!
    let pixels = layers[0].len();

    for ii in 0..pixels {
        let mut jj = 0;
        while layers[jj].as_bytes()[ii] == TRANSPARENT as u8 {
            jj += 1;
        }
        result.push(match layers[jj].as_bytes()[ii] as char {
            BLACK => 'X',
            WHITE => ' ',
            _ => unreachable!(),
        });
    }

    debug!("{}", result.len());

    result
}
