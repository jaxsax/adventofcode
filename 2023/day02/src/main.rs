fn main() {
    println!("{}", p1(include_str!("../../data/02/input")));
    println!("{}", p2(include_str!("../../data/02/input")));
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Vec<Cube>>,
}

#[derive(Debug, PartialEq)]
struct Cube {
    color: Color,
    amount: u32,
}

#[derive(Debug, PartialEq)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

pub fn p1(input: &str) -> u32 {
    let mut amount: u32 = 0;
    for line in input.lines() {
        let game = read_line(line);
        let mut possible = true;
        for cubes in game.sets {
            for cube in cubes {
                if cube.color == Color::RED && cube.amount > 12 {
                    possible = false
                }

                if cube.color == Color::GREEN && cube.amount > 13 {
                    possible = false
                }

                if cube.color == Color::BLUE && cube.amount > 14 {
                    possible = false
                }
            }
        }

        if possible {
            amount += game.id
        }
    }

    amount
}

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn p2(input: &str) -> u32 {
    let mut amount: u32 = 0;
    for line in input.lines() {
        let game = read_line(line);
        let mut largest_cs = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        for cubes in game.sets {
            for cube in cubes {
                match cube.color {
                    Color::RED => {
                        if cube.amount > largest_cs.red {
                            largest_cs.red = cube.amount
                        }
                    }
                    Color::GREEN => {
                        if cube.amount > largest_cs.green {
                            largest_cs.green = cube.amount
                        }
                    }
                    Color::BLUE => {
                        if cube.amount > largest_cs.blue {
                            largest_cs.blue = cube.amount
                        }
                    }
                }
            }
        }

        amount += largest_cs.red * largest_cs.green * largest_cs.blue;
    }

    amount
}

fn convert_to_number(input: &str) -> u32 {
    let b: u32 = 10;
    input
        .chars()
        .filter_map(|x| x.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, v)| v * b.pow(i as u32))
        .sum()
}

fn read_line(input: &str) -> Game {
    let part0: Vec<&str> = input.split(":").collect();
    let game_id: u32 = convert_to_number(
        part0
            .get(0)
            .expect("game id")
            .split(" ")
            .collect::<Vec<&str>>()
            .get(1)
            .expect("game id 1"),
    );

    let sets: Vec<Vec<Cube>> = part0
        .get(1)
        .expect("set parser")
        .split(";")
        .map(|set| {
            let cubes = set.split(",");
            cubes
                .map(|cube| {
                    let mut m = cube.trim().split(" ");
                    let amount = convert_to_number(m.next().unwrap());
                    let color = match m.next().unwrap() {
                        "red" => Color::RED,
                        "green" => Color::GREEN,
                        "blue" => Color::BLUE,
                        _ => panic!("color match failed"),
                    };

                    Cube { amount, color }
                })
                .collect()
        })
        .collect();

    Game { id: game_id, sets }
}

#[cfg(test)]
mod day02 {
    use crate::{p1, p2, read_line, Cube, Game};
    use indoc::indoc;

    #[test]
    fn game_parser() {
        assert_eq!(
            read_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                sets: vec![
                    vec![
                        Cube {
                            color: crate::Color::BLUE,
                            amount: 3,
                        },
                        Cube {
                            color: crate::Color::RED,
                            amount: 4,
                        },
                    ],
                    vec![
                        Cube {
                            color: crate::Color::RED,
                            amount: 1,
                        },
                        Cube {
                            color: crate::Color::GREEN,
                            amount: 2,
                        },
                        Cube {
                            color: crate::Color::BLUE,
                            amount: 6,
                        },
                    ],
                    vec![Cube {
                        color: crate::Color::GREEN,
                        amount: 2,
                    },],
                ],
            }
        )
    }

    const CASE_A: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part_a() {
        assert_eq!(p1(CASE_A), (8 as u32).into())
    }

    #[test]
    fn part_b() {
        assert_eq!(p2(CASE_A), (2286 as u32).into())
    }
}
