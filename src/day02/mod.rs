

pub fn solve(file_string: &str){
    let data = parse(file_string);
    println!("{:?}", solve1(&data));
    println!("{:?}", solve2(&data));
}

fn parse(file_string: &str) -> Vec<Command> {
    file_string.lines().map(|line| parse_command_input(line)).collect()
}

enum Command {
    Down(usize),
    Forward(usize),
    Up(usize)
}

fn parse_command_input(s: &str) -> Command {
    let command_str = s.split_once(' ').unwrap();

    let magnitude = command_str.1.parse::<usize>().unwrap();

    match command_str.0 {
        "forward" => Command::Forward(magnitude),
        "down" => Command::Down(magnitude),
        "up" => Command::Up(magnitude),
        _ => panic!("Unknown Command")
    }
}

fn solve1(commands: &Vec<Command>) -> usize {
    let final_pos = commands.iter().fold((0, 0), |pos, command| {
        match command {
            Command::Forward(val) => (pos.0 + val, pos.1),
            Command::Down(val) => (pos.0, pos.1 + val),
            Command::Up(val) => (pos.0, pos.1 - val)
        }
    });

    final_pos.0 * final_pos.1
}

fn solve2(commands: &Vec<Command>) -> usize {
    let final_pos = commands.iter().fold((0, 0, 0), |pos, command| {
        match command {
            Command::Forward(val) => (pos.0 + val, pos.1 + pos.2 * val, pos.2),
            Command::Down(val) => (pos.0, pos.1, pos.2 + val),
            Command::Up(val) => (pos.0, pos.1, pos.2 - val)
        }
    });

    final_pos.0 * final_pos.1
}
