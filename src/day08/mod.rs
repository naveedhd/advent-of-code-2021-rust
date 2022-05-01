use std::collections::HashMap;
use std::collections::HashSet;


type DigitStream = Vec<String>;
type InputOutput = (DigitStream, DigitStream);
type Data = Vec<InputOutput>;


pub fn solve(file_string: &str) {
    let data = parse(&file_string);

    println!("{:?}", solve1(&data));
    println!("{:?}", solve2(&data));
}

fn parse_line(line_string: &str) -> InputOutput {
    let split = line_string.split_once(" | ").unwrap();

    let input_str = split.0;
    let output_str = split.1;

    let inps = input_str.split(' ').map(|s| s.to_owned()).collect();
    let outs = output_str.split(' ').map(|s| s.to_owned()).collect();

    (inps, outs)
}

fn parse(file_string: &str) -> Data {
    file_string
        .lines()
        .map(parse_line)
        .collect()
}

fn solve1(data: &[InputOutput]) -> usize {
    data
        .iter()
        .flat_map(|line| &line.1)
        .map(|digit| digit.len())
        .filter(|&digit_len| (digit_len == 2 || digit_len == 3 || digit_len == 4 || digit_len == 7))
        .count()
}


fn digit_has_segment(inp: &str, segment_list: &str) -> bool {
    let inp_set: HashSet<char> = inp.chars().collect();
    segment_list.chars().all(|c| inp_set.contains(&c))
}

fn sort_string(s: &str) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v.sort();
    v.iter().collect()
}

fn analyze_stream(input: &[String], output: &[String]) -> usize {
    let mut one = "";
    let mut four = "";
    let mut seven = "";
    let mut eight = "";

    let mut two_or_three_or_five: Vec<&str> = Vec::new();
    let mut zero_or_six_or_nine: Vec<&str> = Vec::new();

    for digit in input {
        match digit.len() {
            2 => { one = digit },
            3 => { seven = digit },
            4 => { four = digit },
            5 => { two_or_three_or_five.push(digit); }
            6 => { zero_or_six_or_nine.push(digit); }
            7 => { eight = digit },
            _ => {}
        }
    }

    let three = two_or_three_or_five
                    .iter()
                    .filter(|digit| digit_has_segment(digit, one)).nth(0).unwrap();

    let nine = zero_or_six_or_nine
                    .iter()
                    .filter(|digit| digit_has_segment(digit, four))
                    .nth(0)
                    .unwrap();

    let (zero, six) = zero_or_six_or_nine
                        .iter()
                        .fold( ("", ""), |acc, digit| {
                            if digit == nine { acc }
                            else if digit_has_segment(digit, one) { (digit, acc.1) }
                            else { (acc.0, digit) }
                        });

    let (two, five) = two_or_three_or_five
                        .iter()
                        .fold( ("", ""), |acc, digit| {
                            if digit == three { acc }
                            else if digit_has_segment(six, digit) { (acc.0, digit) }
                            else { (digit, acc.1) }
                        });

    let translations = HashMap::from([
        (sort_string(&zero), 0),
        (sort_string(&one), 1),
        (sort_string(&two), 2),
        (sort_string(&three), 3),
        (sort_string(&four), 4),
        (sort_string(&five), 5),
        (sort_string(&six), 6),
        (sort_string(&seven), 7),
        (sort_string(&eight), 8),
        (sort_string(&nine), 9)
    ]);

    output.iter().fold(0, |acc, out| acc * 10 + translations[&sort_string(out)])
}


fn solve2(data: &[InputOutput]) -> usize {
    data
        .iter()
        .fold(0, |acc, (input, output)| acc + analyze_stream(input, output))
}
