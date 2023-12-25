pub fn main() {
    println!("{}", p1(include_str!("../../data/01/input")));
    println!("{}", p2(include_str!("../../data/01/input")));
}

pub fn p1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let one = digits.next().unwrap();
            let two = digits.last().unwrap_or(one);

            return one * 10 + two;
        })
        .sum()
}

pub fn p2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = extract_digits(line).into_iter();
            let one = digits.next().unwrap();
            let two = digits.last().unwrap_or(one);

            return one * 10 + two;
        })
        .sum()
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn extract_digits(i: &str) -> Vec<u32> {
    let mut things: Vec<u32> = vec![];
    let chars = i.as_bytes();
    for (i, c) in i.chars().enumerate() {
        if let Some(x) = c.to_digit(10) {
            things.push(x);
        } else {
            for (j, dg) in DIGITS.iter().enumerate() {
                if chars[i..].starts_with(dg.as_bytes()) {
                    things.push(j as u32 + 1)
                }
            }
        }
    }

    things
}

#[cfg(test)]
mod day01 {
    use crate::{p1, p2};
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn part_a() {
        assert_eq!(p1(CASE_A), (142 as u32).into())
    }

    const CASE_B: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn part_b() {
        assert_eq!(p2(CASE_A), (142 as u32).into());
        assert_eq!(p2(CASE_B), (281 as u32).into());
    }
}
