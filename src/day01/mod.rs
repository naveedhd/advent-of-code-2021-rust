use itertools::Itertools;


pub fn solve(file_string: &str) {
    let data = parse(file_string);
    println!("{:?}", solve1(&data));
    println!("{:?}", solve2(&data));
}

fn parse(file_string: &str) -> Vec<isize> {
    file_string.lines().map(|line| line.parse::<isize>().unwrap()).collect()
}


fn inc_if_le(val: usize, (a, b): (&isize, &isize)) -> usize {
    if a < b { val + 1 } else { val }
}

fn triple_inc_if_le(val: usize, (a, b, c, d): (&isize, &isize, &isize, &isize)) -> usize {
    inc_if_le(val, (&(a + b + c), &(b + c + d)))
}

fn solve1(data: &Vec<isize>) -> usize {
    data.iter().tuple_windows().fold(0, inc_if_le)
}

fn solve2(data: &Vec<isize>) -> usize {
    data.iter().tuple_windows().fold(0, triple_inc_if_le)
}
