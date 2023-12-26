use regex::Regex;
use std::collections::HashMap;

fn main() {
    println!("{}", p1(include_str!("../../data/03/input")));
    println!("{}", p2(include_str!("../../data/03/input")));
}

fn p1(input: &str) -> u32 {
    parse(input)
        .gears
        .iter()
        .filter(|x| x.part_number)
        .map(|x| x.value)
        .sum()
}

fn p2(input: &str) -> u32 {
    parse(input)
        .ratios
        .iter()
        .filter(|x| x.1.len() == 2)
        .map(|(_, vals)| vals[0] * vals[1])
        .sum()
}

#[derive(Debug)]
struct Gear {
    value: u32,
    part_number: bool,
}

struct ParseResult {
    gears: Vec<Gear>,
    ratios: HashMap<(usize, usize), Vec<u32>>,
}

fn parse(input: &str) -> ParseResult {
    let mut symbols = HashMap::new();
    let mut gears: Vec<Gear> = vec![];
    let mut ratios: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (y, v) in input.lines().enumerate() {
        for (x, v1) in v.char_indices() {
            if !v1.is_ascii_digit() && v1 != '.' {
                symbols.insert((x, y), v1);
            }
        }
    }

    let re = Regex::new(r"\d+").unwrap();

    for (y, line) in input.lines().enumerate() {
        for g in re.find_iter(line) {
            let value = g.as_str().parse::<u32>().unwrap();
            let mut gear = Gear {
                part_number: false,
                value,
            };

            for nx in g.start().saturating_sub(1)..=g.end() {
                for ny in y.saturating_sub(1)..=y + 1 {
                    let pos = (nx, ny);
                    let sym = symbols.get(&pos);

                    if sym.is_some() {
                        gear.part_number = true
                    }

                    if sym == Some(&'*') {
                        ratios.entry(pos).or_insert(Vec::new()).push(value);
                    }
                }
            }

            gears.push(gear)
        }
    }

    ParseResult { gears, ratios }
}

#[cfg(test)]
mod day03 {
    use crate::{p1, p2};
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part_a() {
        assert_eq!(p1(CASE_A), (4361 as u32).into())
    }

    #[test]
    fn part_b() {
        assert_eq!(p2(CASE_A), (467835 as u32).into())
    }
}
