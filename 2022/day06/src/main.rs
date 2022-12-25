use std::collections::HashSet;

fn part1(input: &str) {
    // Create a sliding window of 4 characters
    // returning when you encounter a window where all 4 characters are unique

    for i in 0..input.len() {
        if i <= 3 {
            continue;
        }

        let start = i - 4;
        let end = i;
        let window = &input[start..end];

        let unique_count = window.chars().collect::<HashSet<char>>().len();
        if unique_count == 4 {
            println!("part 1: {}; {}", window, end);
            return;
        }
    }
}

fn part2(input: &str) {
    // Create a sliding window of 4 characters
    // returning when you encounter a window where all 4 characters are unique

    for i in 0..input.len() {
        if i <= 14 - 1 {
            continue;
        }

        let start = i - 14;
        let end = i;
        let window = &input[start..end];

        let unique_count = window.chars().collect::<HashSet<char>>().len();
        if unique_count == 14 {
            println!("part 2: {}; {}", window, end);
            return;
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}
