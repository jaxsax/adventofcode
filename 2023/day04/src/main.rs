use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", p1(include_str!("../../data/04/input")));
    println!("{}", p2(include_str!("../../data/04/input")));
}

fn p1(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|x| {
            let mut amount: u32 = 0;
            for i in 0..x.numbers.len() {
                if i == 0 {
                    amount += 1;
                } else {
                    amount *= 2
                }
            }
            amount
        })
        .sum()
}

fn p2(input: &str) -> u32 {
    let original_cards = parse(input);
    let mut cards_won: HashMap<usize, u32> = HashMap::new();

    for c in original_cards {
        _ = *cards_won.entry(c.index).or_insert(0) += 1;

        for i in 0..c.numbers.len() {
            let amount = *match cards_won.get(&c.index) {
                Some(v) => v,
                _ => &0,
            };

            _ = *cards_won.entry(c.index + (i + 1)).or_insert(0) += amount;
        }
    }

    cards_won.into_values().sum()
}

#[derive(Debug)]
struct Card {
    index: usize,
    numbers: Vec<u32>,
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let card = line.split_once(":").unwrap().1.trim();
            let (a, b) = card.split_once("|").unwrap();
            let winning_numbers: HashSet<u32> =
                HashSet::from_iter(a.split_whitespace().map(|x| x.parse::<u32>().unwrap()));

            let numbers = b
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .filter(|x| winning_numbers.contains(x))
                .collect();

            Card {
                index: idx + 1,
                numbers,
            }
        })
        .collect()
}

#[cfg(test)]
mod day04 {
    use crate::{p1, p2};
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part_a() {
        assert_eq!(p1(CASE_A), (13 as u32).into())
    }

    #[test]
    fn part_b() {
        assert_eq!(p2(CASE_A), (30 as u32).into())
    }
}
