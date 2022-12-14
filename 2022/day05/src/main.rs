use std::{collections::VecDeque, ops::Index};

#[derive(Debug, Clone, Copy)]
struct Instruction {
    amount: u32,
    from: usize,
    to: usize,
}

fn main() {
    let input = include_str!("../input.txt");
    let mut stacks: Vec<VecDeque<String>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut parsing_instructions_now = false;
    for line in input.lines() {
        let line = String::from(line);
        if !parsing_instructions_now {
            for (i, stack) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                if stacks.len() < i + 1 {
                    stacks.push(VecDeque::new());
                }

                let stack_string = stack.iter().collect::<String>().to_string();
                if stack_string.trim().len() == 0 {
                    // This stack is empty
                    continue;
                }

                if !stack_string.contains("[") && !stack_string.contains("]") {
                    continue;
                }

                let character = stack_string.index(1..=1);
                // println!("{} {:?} -> {}, '{}'", i, stack, stack_string, character);
                stacks[i].push_front(character.to_string());
            }
        }

        if parsing_instructions_now {
            let parts = line
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            let amount: u32 = parts.get(1).unwrap().to_string().parse::<u32>().unwrap();
            let source_stack: usize = parts.get(3).unwrap().to_string().parse().unwrap();
            let destination_stack: usize = parts.get(5).unwrap().to_string().parse().unwrap();

            let inst = Instruction {
                amount,
                from: source_stack,
                to: destination_stack,
            };

            instructions.push(inst);
        }

        if line == "" {
            parsing_instructions_now = true;
            continue;
        }
    }

    // part1(&mut stacks, instructions)
    part2(&mut stacks, instructions)
}

fn part2(stacks: &mut Vec<VecDeque<String>>, instructions: Vec<Instruction>) {
    for instruction in instructions {
        let mut items: Vec<String> = Vec::new();
        for _ in 0..instruction.amount {
            let item = stacks[instruction.from - 1].pop_back().unwrap();
            items.push(item);
        }

        items.reverse();
        for item in items {
            stacks[instruction.to - 1].push_back(item);
        }
    }

    let answer = stacks
        .iter()
        .map(|s| s.back().unwrap().to_string())
        .collect::<String>();

    println!("part 1: {:?}", answer);
}

fn part1(stacks: &mut Vec<VecDeque<String>>, instructions: Vec<Instruction>) {
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let item = stacks[instruction.from - 1].pop_back().unwrap();
            stacks[instruction.to - 1].push_back(item);
        }
    }

    let answer = stacks
        .iter()
        .map(|s| s.back().unwrap().to_string())
        .collect::<String>();

    println!("part 2: {:?}", answer);
}
