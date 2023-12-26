use std::{ops::RangeInclusive, time::Instant};

fn main() {
    println!("{}", p1(include_str!("../../data/05/input")));

    let st = Instant::now();
    println!(
        "{} elapsed={:?}",
        p2(include_str!("../../data/05/input")),
        st.elapsed()
    );
}

fn p1(input: &str) -> usize {
    let pr = parse(input);

    pr.seeds
        .iter()
        .map(|x| {
            let location: usize = pr.category_ranges.iter().fold(*x as usize, |acc, cr| {
                let matching_ranges = cr.iter().filter(|x| x.source.contains(&acc)).next();
                if matching_ranges.is_some() {
                    return acc - matching_ranges.unwrap().source.start()
                        + matching_ranges.unwrap().dest.start();
                }
                acc
            });

            location
        })
        .min()
        .unwrap()
}

fn p2(input: &str) -> usize {
    // Kinda crappy, but it only takes 144s in release mode
    let pr = parse(input);
    let seeds = pr
        .seeds
        .chunks_exact(2)
        .flat_map(|x| (x[0]..=x[0] + x[1]))
        .collect::<Vec<u32>>();
    let size: usize = seeds.len();
    let one_percent = 0.01 * size as f64;
    seeds
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if i % one_percent as usize == 0 {
                println!("{}/{} - {:.2}", i, size, i as f64 / size as f64);
            }
            let location: usize = pr.category_ranges.iter().fold(*x as usize, |acc, cr| {
                let matching_ranges = cr.iter().filter(|x| x.source.contains(&acc)).next();
                if matching_ranges.is_some() {
                    return acc - matching_ranges.unwrap().source.start()
                        + matching_ranges.unwrap().dest.start();
                }
                acc
            });

            location
        })
        .min()
        .unwrap()
}

#[derive(Debug)]
struct CategoryRange {
    source: RangeInclusive<usize>,
    dest: RangeInclusive<usize>,
}

#[derive(Debug)]
struct ParseResult {
    seeds: Vec<u32>,
    category_ranges: Vec<Vec<CategoryRange>>,
}

fn parse(input: &str) -> ParseResult {
    let mut pr = ParseResult {
        seeds: vec![],
        category_ranges: vec![],
    };

    let mut acc: Vec<&str> = vec![];
    let mut ranges: Vec<Vec<CategoryRange>> = vec![];

    let process_acc = |s: Vec<&str>| -> Vec<CategoryRange> {
        s.iter()
            .map(|x| {
                let items: Vec<usize> = x
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let dest = items[0];
                let src = items[1];
                let count = items[2];

                CategoryRange {
                    source: src..=src + count - 1,
                    dest: dest..=dest + count - 1,
                }
            })
            .collect()
    };

    for line in input.lines() {
        if line.trim().len() == 0 {
            continue;
        }

        if line.starts_with("seeds:") {
            pr.seeds = line
                .split_once(":")
                .unwrap()
                .1
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            continue;
        }

        if line.ends_with("map:") {
            if acc.is_empty() {
                continue;
            }
            ranges.push(process_acc(acc.clone()));
            acc.clear();

            continue;
        }

        acc.push(line)
    }

    ranges.push(process_acc(acc.clone()));
    pr.category_ranges = ranges;
    pr
}

#[cfg(test)]
mod day05 {
    use crate::{p1, p2};
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    "};

    #[test]
    fn part_a() {
        assert_eq!(p1(CASE_A), (35 as usize).into())
    }

    #[test]
    fn part_b() {
        assert_eq!(p2(CASE_A), (46 as usize).into())
    }
}
