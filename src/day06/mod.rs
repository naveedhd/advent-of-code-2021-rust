

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

fn grow_fishes(data: &[usize], steps: usize) -> usize {
    let mut fish_life_count = [0; 9];

    for d in data {
        fish_life_count[*d] += 1;
    }

    for _ in 0..steps {
        fish_life_count.rotate_left(1);
        fish_life_count[6] += fish_life_count[8];
    }

    fish_life_count.iter().sum()
}


fn solve1(data: &[usize]) -> usize {
    grow_fishes(data, 80)
}

fn solve2(data: &[usize]) -> usize {
    grow_fishes(data, 256)
}
