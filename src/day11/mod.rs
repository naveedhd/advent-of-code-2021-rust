use std::collections::VecDeque;
use std::collections::HashSet;


pub fn solve(file_string: &str) {
    let data = parse(&file_string);

    println!("{}", solve1(data.clone().as_mut_slice()));
    println!("{}", solve2(data.clone().as_mut_slice()));
}


fn parse(file_string: &str) -> Vec<usize> {
    file_string
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}


fn neighbours(idx: usize) -> Vec<usize> {

    let mut neighs = Vec::new();


    let col = idx / 10;
    let row = idx % 10;

    let to_idx = |col, row| 10 * col + row;

    if col > 0 {
        if row > 0 {
            neighs.push(to_idx(col - 1, row - 1));
        }
        neighs.push(to_idx(col - 1, row));
        if row < 9 {
            neighs.push(to_idx(col - 1, row + 1));
        }
    }

    if row > 0 {
        neighs.push(to_idx(col, row - 1));
    }
    if row < 9 {
        neighs.push(to_idx(col, row + 1));
    }

    if col < 9 {
        if row > 0 {
            neighs.push(to_idx(col + 1, row - 1));
        }
        neighs.push(to_idx(col + 1, row));
        if row < 9 {
            neighs.push(to_idx(col + 1, row + 1));
        }

    }

    neighs
}

fn step(data: &mut [usize]) -> usize {
    let mut idx_flashed = HashSet::new();

    let mut idx_to_process: VecDeque<usize> = (0..100).collect();

    while let Some(idx) = idx_to_process.pop_front() {
        if idx_flashed.contains(&idx) { continue; }

        match data[idx] {
            9 => {
                data[idx] = 0;
                idx_flashed.insert(idx);

                for neighbour in neighbours(idx) {
                    idx_to_process.push_back(neighbour);
                }
            },
            _ => { data[idx] += 1; }
        }
    }

    idx_flashed.len()
}


fn solve1(data: &mut [usize]) -> usize {
    let mut times = 0;

    for _ in 0..100 {
        times += step(data);
    }

    times
}

fn solve2(data: &mut [usize]) -> usize {
    for step_num in 1..=1000 {
        if step(data) == 100 {
            return step_num;
        }
    }

    0
}
