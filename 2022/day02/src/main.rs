use core::fmt;

enum Type {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match &self {
            Type::Rock => "Rock",
            Type::Paper => "Paper",
            Type::Scissors => "Scissors",
        };
        return write!(f, "{}", s);
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match &self {
            Outcome::Win => "Win",
            Outcome::Loss => "Loss",
            Outcome::Draw => "Draw",
        };
        return write!(f, "{}", s);
    }
}

fn points_for_type(a: &Type) -> i32 {
    match a {
        Type::Rock => 1,
        Type::Paper => 2,
        Type::Scissors => 3,
    }
}

fn outcome(a: &Type, b: &Type) -> Outcome {
    return match (a, b) {
        (Type::Rock, Type::Scissors) => Outcome::Win,
        (Type::Scissors, Type::Rock) => Outcome::Loss,

        (Type::Scissors, Type::Paper) => Outcome::Win,
        (Type::Paper, Type::Scissors) => Outcome::Loss,

        (Type::Paper, Type::Rock) => Outcome::Win,
        (Type::Rock, Type::Paper) => Outcome::Loss,

        (Type::Rock, Type::Rock) => Outcome::Draw,
        (Type::Paper, Type::Paper) => Outcome::Draw,
        (Type::Scissors, Type::Scissors) => Outcome::Draw,
    };
}

fn pt1(rounds: &Vec<(&str, &str)>) {
    let mut total_score = 0;
    for r in rounds {
        let opp = r.0;
        let you = r.1;

        let opp = match opp {
            "A" => Type::Rock,
            "B" => Type::Paper,
            "C" => Type::Scissors,
            _ => panic!("unhandled type"),
        };

        let you = match you {
            "X" => Type::Rock,
            "Y" => Type::Paper,
            "Z" => Type::Scissors,
            _ => panic!("unhandled type"),
        };

        let outcome = outcome(&you, &opp);
        let score = points_for_type(&you)
            + match outcome {
                Outcome::Win => 6,
                Outcome::Loss => 0,
                Outcome::Draw => 3,
            };

        total_score += score;
    }

    println!("part 1 total score: {}", total_score);
}

fn pt2(rounds: &Vec<(&str, &str)>) {
    let mut total_score = 0;
    for r in rounds {
        let opp = r.0;

        let opp = match opp {
            "A" => Type::Rock,
            "B" => Type::Paper,
            "C" => Type::Scissors,
            _ => panic!("unhandled type"),
        };

        let desired_outcome = match r.1 {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("unhandled type"),
        };

        let what_i_will_show = me(&opp, &desired_outcome);

        let outcome = outcome(&what_i_will_show, &opp);
        let score = points_for_type(&what_i_will_show)
            + match outcome {
                Outcome::Win => 6,
                Outcome::Loss => 0,
                Outcome::Draw => 3,
            };

        total_score += score;
    }

    println!("part 2 total score: {}", total_score);
}

fn me(opp: &Type, outcome: &Outcome) -> Type {
    match (outcome, opp) {
        (Outcome::Win, Type::Rock) => Type::Paper,
        (Outcome::Draw, Type::Rock) => Type::Rock,
        (Outcome::Loss, Type::Rock) => Type::Scissors,

        (Outcome::Win, Type::Paper) => Type::Scissors,
        (Outcome::Draw, Type::Paper) => Type::Paper,
        (Outcome::Loss, Type::Paper) => Type::Rock,

        (Outcome::Win, Type::Scissors) => Type::Rock,
        (Outcome::Draw, Type::Scissors) => Type::Scissors,
        (Outcome::Loss, Type::Scissors) => Type::Paper,
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let lines = input.split("\n");

    let mut rounds: Vec<(&str, &str)> = Vec::new();
    for line in lines {
        if line == "" {
            continue;
        }

        let round: Vec<&str> = line.split(" ").collect();
        if round.len() != 2 {
            panic!("round != 2 items, len:{}", round.len())
        }

        rounds.push((round[0], round[1]))
    }

    pt1(&rounds);
    pt2(&rounds);
}
