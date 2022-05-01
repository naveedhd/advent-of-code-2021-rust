

pub fn solve(file_string: &str) {
    let data = parse(&file_string);

    println!("{:?}", solve1(&data));
    println!("{:?}", solve2(&data));
}

fn parse(file_string: &str) -> Vec<usize> {
    file_string
        .lines()
        .flat_map(|s| s.split(',').map(|x| x.parse::<usize>().unwrap()))
        .collect()
}


fn solve1(positions: &[usize]) -> usize {
    let min_value = positions.iter().min().unwrap();
    let max_value = positions.iter().max().unwrap();

    let value_range = max_value - min_value + 1;

    let mut fuel_for_positions = vec![0; value_range];

    for position in positions {
        for i in 0..value_range {
            let idx_position = min_value + i;
            let fuel = if idx_position < *position {
                position - idx_position
            } else {
                idx_position - position
            };
            fuel_for_positions[i] += fuel;
        }
    }

    *fuel_for_positions.iter().min().unwrap()
}

fn solve2(positions: &[usize]) -> usize {
    let min_value = positions.iter().min().unwrap();
    let max_value = positions.iter().max().unwrap();

    let value_range = max_value - min_value + 1;

    let mut fuel_for_positions = vec![0; value_range];

    for position in positions {
        for i in 0..value_range {
            let idx_position = min_value + i;
            let displacement = if idx_position < *position {
                position - idx_position
            } else {
                idx_position - position
            };

            let fuel = displacement * (displacement + 1 ) / 2;

            fuel_for_positions[i] += fuel;
        }
    }

    *fuel_for_positions.iter().min().unwrap()
}
