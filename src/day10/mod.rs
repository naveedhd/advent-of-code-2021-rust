use std::collections::HashMap;


pub fn solve(file_string: &str) {
    println!("{}", solve1(&file_string));
    println!("{}", solve2(&file_string));
}


fn corrupted_line(line: &str) -> Option<char> {
    let mut stack = Vec::new();

    let bracket_match = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    line
        .chars()
        .find_map(|ch| {
            match bracket_match.get(&ch) {
                Some(_) => { stack.push(ch); None },
                None => { if stack.is_empty() || ch != bracket_match[&stack.pop().unwrap()] { Some(ch) } else { None }}
            }
        })
}

fn incomplete_line(line: &str) -> Option<Vec<char>> {
    let mut stack = Vec::new();

    let bracket_match = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    for ch in line.chars() {
        if bracket_match.contains_key(&ch) {
            stack.push(ch);
        } else {
            if stack.is_empty() || ch != bracket_match[&stack.pop().unwrap()] {
                return None;
            }
        }
    }

    let completion_chunk = stack.iter().rev().map(|ch| bracket_match[&ch]).collect();
    Some(completion_chunk)
}

fn solve1(data: &str) -> usize {
    let bracket_score = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    data
        .lines()
        .filter_map(corrupted_line)
        .map(|ch| bracket_score[&ch])
        .sum()
}

fn solve2(data: &str) -> usize {
    let bracket_score = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);

    let score_calculator = |chs: Vec<char>| { chs.iter().fold(0, |acc, ch| acc * 5 + bracket_score[&ch]) };

    let mut scores: Vec<usize> = data
            .lines()
            .filter_map(incomplete_line)
            .map(score_calculator)
            .collect();

    let middle_idx = scores.len() /  2;
    scores.select_nth_unstable(middle_idx);

    scores[middle_idx]
}
