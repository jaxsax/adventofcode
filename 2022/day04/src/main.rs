use std::{cmp, collections::HashSet};

fn parse_assignment_range(s: &str) -> Vec<u32> {
    let thing: Vec<&str> = s.split("-").collect();
    let start = thing.get(0).unwrap().parse::<u32>().unwrap();
    let end = thing.get(1).unwrap().parse::<u32>().unwrap();
    let mut range = Vec::new();

    for i in start..end + 1 {
        range.push(i);
    }

    range
}

fn part1(input: &str) {
    let score: usize = input
        .lines()
        .map(|pair| {
            let pairs: Vec<&str> = pair.split(",").collect();

            let p0_assignments = parse_assignment_range(pairs.get(0).unwrap());
            let p1_assignments = parse_assignment_range(pairs.get(1).unwrap());

            let p0_hashset: HashSet<u32> =
                HashSet::from_iter(p0_assignments.iter().map(|x| *x).collect::<Vec<u32>>());
            let p1_hashset: HashSet<u32> =
                HashSet::from_iter(p1_assignments.iter().map(|x| *x).collect::<Vec<u32>>());

            let intersection = p0_hashset
                .intersection(&p1_hashset)
                .map(|x| *x)
                .collect::<Vec<u32>>();

            let size: usize = cmp::min(p0_assignments.len(), p1_assignments.len());
            if intersection.len() == size {
                return 1;
            }
            0
        })
        .sum();
    println!("part 1: {}", score)
}

fn part2(input: &str) {
    let score: usize = input
        .lines()
        .map(|pair| {
            let pairs: Vec<&str> = pair.split(",").collect();

            let p0_assignments = parse_assignment_range(pairs.get(0).unwrap());
            let p1_assignments = parse_assignment_range(pairs.get(1).unwrap());

            let p0_hashset: HashSet<u32> =
                HashSet::from_iter(p0_assignments.iter().map(|x| *x).collect::<Vec<u32>>());
            let p1_hashset: HashSet<u32> =
                HashSet::from_iter(p1_assignments.iter().map(|x| *x).collect::<Vec<u32>>());

            let intersection = p0_hashset
                .intersection(&p1_hashset)
                .map(|x| *x)
                .collect::<Vec<u32>>();

            if intersection.len() > 0 {
                return 1;
            }
            0
        })
        .sum();
    println!("part 2: {}", score)
}

fn main() {
    let input = include_str!("../input.txt");

    // Generate a range inclusively
    let thing: Vec<i32> = (0..=10).collect();
    println!("{:?}", thing);
    part1(input);
    part2(input);
}
