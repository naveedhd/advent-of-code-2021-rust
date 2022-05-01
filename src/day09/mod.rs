use std::collections::VecDeque;


pub fn solve(file_string: &str) {
    let data = parse(&file_string);

    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
}

fn parse(file_string: &str) -> Vec<Vec<u32>> {
    file_string
        .lines()
        .map(|line| line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect())
        .collect()
}

fn lowest_indices(data: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let num_cols = data.len();
    let num_rows = data[0].len();

    let mut visited = vec![vec![false; num_rows]; num_cols];

    let mut result = Vec::new();

    for col in 0..num_cols {
        for row in 0..num_rows {
            if visited[col][row] { continue; }

            visited[col][row] = true;

            let this = data[col][row];

            // check top
            if col > 0 {
                if data[col - 1][row] <= this { continue; }
                else { visited[col - 1][row] = true; }
            }

            // check down
            if col < num_cols - 1 {
                if data[col + 1][row] <= this { continue; }
                else { visited[col + 1][row] = true; }
            }

            // check left
            if row > 0 {
                if data[col][row - 1] <= this { continue; }
                else { visited[col][row - 1] = true; }
            }

            // check right
            if row < num_rows - 1 {
                if data[col][row + 1] <= this { continue; }
                else { visited[col][row + 1] = true; }
            }

            // found the low value
            result.push((col, row));
        }
    }

    result
}

fn solve1(data: &[Vec<u32>]) -> u32 {
    lowest_indices(data)
        .into_iter()
        .fold(0u32, |acc, (col, row)| acc + data[col][row] + 1)
}

fn expand_drain(data: &[Vec<u32>], idx: (usize, usize)) -> usize {
    let num_cols = data.len();
    let num_rows = data[0].len();
    let mut processed = 0;
    let mut deque = VecDeque::new();

    let mut visited = vec![vec![false; num_rows]; num_cols];

    deque.push_back(idx);

    while !deque.is_empty() {
        let (col, row) = deque.pop_front().unwrap();

        if visited[col][row] { continue; }

        visited[col][row] = true;

        // check top
        if col > 0 && data[col - 1][row] != 9 {
            deque.push_back((col - 1, row));
        }

        // check down
        if col < num_cols - 1 && data[col + 1][row] != 9 {
            deque.push_back((col + 1, row));
        }

        // check left
        if row > 0 && data[col][row - 1] != 9 {
            deque.push_back((col, row - 1));
        }

        // check right
        if row < num_rows - 1 && data[col][row + 1] != 9 {
            deque.push_back((col, row + 1));
        }

        processed += 1;
    }

    processed
}

fn solve2(data: &[Vec<u32>]) -> usize {
    let mut drain_lens: Vec<usize> = lowest_indices(data)
                        .into_iter()
                        .map(|idx| expand_drain(data, idx))
                        .collect();

    drain_lens.sort();

    drain_lens.iter().rev().take(3).product()
}
