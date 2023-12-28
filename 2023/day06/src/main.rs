fn main() {
    println!("{}", p1(include_str!("../../data/06/input")));
    println!("{}", p2(include_str!("../../data/06/input")));
}

fn p1(input: &str) -> u32 {
    let (times, distance) = parse(input);
    let mut total: Vec<u32> = Vec::new();
    for i in 0..times.len() {
        let time = times[i];
        let distance = distance[i];

        let mut times_passed: u32 = 0;
        for x in (1..=time).rev() {
            let hold = time - x;
            if hold * x > distance {
                times_passed += 1;
            }
        }

        if times_passed > 0 {
            total.push(times_passed);
        }
    }

    total.iter().fold(1, |acc, x| acc * x)
}

fn p2(input: &str) -> u64 {
    let (times, distance) = parse(input);

    let time: u64 = times
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse()
        .unwrap();

    let distance: u64 = distance
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string())
        .parse()
        .unwrap();

    let mut won_at_hold = 0;
    for i in 1..=time {
        if (time - i) * i >= distance {
            won_at_hold = i;
            break;
        }
    }

    time - won_at_hold - won_at_hold + 1
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut times: Vec<u32> = Vec::new();
    let mut distance: Vec<u32> = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        let part = line.split_once(":").unwrap();

        if idx == 0 {
            times = part
                .1
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
        }
        if idx == 1 {
            distance = part
                .1
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
        }
    }

    (times, distance)
}

#[cfg(test)]
mod day06 {
    use crate::{p1, p2};
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part_a() {
        assert_eq!(p1(CASE_A), (288 as u32).into())
    }

    #[test]
    fn part_b() {
        assert_eq!(p2(CASE_A), (71503 as u64).into())
    }
}
