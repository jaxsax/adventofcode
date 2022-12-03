fn main() {
    let input = include_str!("../input.txt");
    let split = input.split("\n");
    let mut elf_index = 0;
    let mut calories_to_elf_id = vec![0];
    for s in split {
        if s == "" {
            elf_index += 1;
            calories_to_elf_id.push(0);
            continue;
        }

        let calories = s.parse::<i32>().unwrap();
        calories_to_elf_id[elf_index] += calories
    }

    calories_to_elf_id.sort_by(|a, b| b.cmp(a));
    println!("1: {}", calories_to_elf_id[0]);
    println!("2: {}", calories_to_elf_id[1]);
    println!("3: {}", calories_to_elf_id[2]);
    println!(
        "total: {}",
        calories_to_elf_id[0] + calories_to_elf_id[1] + calories_to_elf_id[2]
    );
}
