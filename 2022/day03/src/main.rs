use std::collections::HashSet;

fn score_character(c: char) -> u32 {
    let c1 = c as u32;
    let priority = if c.is_lowercase() {
        c1 - 97 + 1
    } else if c.is_uppercase() {
        c1 - 65 + 1 + 26
    } else {
        panic!("??");
    };

    priority
}

fn part1(input: &str) {
    let things: u32 = input
        .lines()
        .map(|line| {
            if line.len() % 2 != 0 {
                panic!("unequal lengths {}", line)
            }
            let divide = line.len() / 2;
            let part1 = line.get(0..divide).unwrap();
            let part2 = line.get(divide..line.len()).unwrap();

            let p1_vec = part1.chars().collect::<HashSet<char>>();
            let p2_vec = part2.chars().collect::<HashSet<char>>();

            p1_vec
                .intersection(&p2_vec)
                .map(|x| *x)
                .collect::<Vec<char>>()
        })
        .filter(|c| c.len() > 0)
        .flatten()
        .map(score_character)
        .sum();

    println!("part 1: {:?}", things);
}

fn part2(input: &str) {
    let output: u32 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|rucksacks| {
            let mut rucksack_sets: Vec<HashSet<char>> = Vec::new();
            for r in rucksacks {
                let a = r.chars().collect::<HashSet<char>>();
                rucksack_sets.push(a);
            }

            let unique_characters = intersection_of_sets(rucksack_sets);
            return unique_characters.iter().map(|c| *c).collect::<Vec<char>>();
        })
        .flatten()
        .map(score_character)
        .sum();
    println!("part 2: {}", output);
}

fn intersection_of_sets(mut sets: Vec<HashSet<char>>) -> HashSet<char> {
    if sets.is_empty() {
        return HashSet::new();
    }
    if sets.len() == 1 {
        return sets.pop().unwrap();
    }

    let mut acc = sets.pop().unwrap();
    acc.retain(|item| sets.iter().all(|set| set.contains(item)));

    acc
}

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input)
}
