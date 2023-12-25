fn main() {
    let input = include_str!("../example.txt");

    let mut directory: Vec<Option<String>> = vec![];
    let mut previous_command: &str = "";
    for line in input.lines() {
        let is_command = line.starts_with("$");
        if is_command {
            let cmd = line[1..].trim().split(" ").collect::<Vec<&str>>();
            match cmd[0] {
                "cd" => {
                    let arg = cmd[1].to_string();
                    if arg == ".." {
                        directory.pop();
                    } else {
                        directory.push(Some(arg));
                    }
                }
                _ => println!("{:?}", cmd),
            }

            previous_command = cmd[0];
            continue;
        }

        match previous_command {
            "ls" => {
                println!("{:?}\n\t{}", directory, line)
            }
            _ => (),
        }
    }
}
