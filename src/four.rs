use std::collections::HashMap;

pub fn part_a() -> i32 {
    solve(265275, 781584, Mode::A)
}

pub fn part_b() -> i32 {
    solve(265275, 781584, Mode::B)
}

enum Mode {
    A,
    B,
}

fn solve(start: i32, finish: i32, mode: Mode) -> i32 {
    let mut good_nums = 0;

    for xx in start..finish {
        // This hashmap stores any digits seen in multiples (as the key) with
        // the number of times they're seen (the value).
        let mut digits_seen: HashMap<i32, i32> = HashMap::new();
        let mut increasing = true;
        let mut last_digit = 0;

        for ii in xx.to_string().chars() {
            let num: i32 = ii.to_string().parse().unwrap();

            // Check that the last digit isn't greater than this one
            if last_digit > num {
                increasing = false;
                break;
            } else if last_digit == num {
                // If we're here this is already the second time we've
                // seen this digit, so start the count at 1+1=2
                let count = digits_seen.entry(num).or_insert(1);
                *count += 1;
            }

            last_digit = num;
        }

        if increasing {
            match mode {
                Mode::A => {
                    if digits_seen.values().any(|&val| val >= 2) {
                        good_nums += 1;
                    }
                }
                Mode::B => {
                    if digits_seen.values().any(|&val| val == 2) {
                        good_nums += 1;
                    }
                }
            }
        }
    }

    good_nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good() {
        assert_eq!(solve(111111, 111112, Mode::A), 1)
    }

    #[test]
    fn not_increasing() {
        assert_eq!(solve(223450, 223451, Mode::A), 0)
    }

    #[test]
    fn no_double() {
        assert_eq!(solve(123789, 123790, Mode::A), 0)
    }
}
