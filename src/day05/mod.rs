use std::collections::HashMap;
use std::cmp;
use std::iter::repeat;


type Point = (usize, usize);
type PointPair = (Point, Point);


pub fn solve(file_string: &str) {
    let data = parse(&file_string);

    println!("{:?}", solve1(&data));
    println!("{:?}", solve2(&data));
}

fn parse_point(point_str: &str) -> Point {
    let split = point_str.split_once(',').unwrap();
    (split.0.parse::<usize>().unwrap(), split.1.parse::<usize>().unwrap())
}

fn parse_line(line: &str) -> PointPair {
    let split = line.split_once(" -> ").unwrap();
    (parse_point(split.0), parse_point(split.1))
}

fn parse(file_string: &str) -> Vec<PointPair> {
    file_string.lines().map(parse_line).collect()
}

fn rook_iterator(point_pair: &PointPair) -> Vec<Point> {
    match point_pair {
        (a, b) if a.0 == b.0 => {
            let x = a.0;
            let min_y = cmp::min(a.1, b.1);
            let max_y = cmp::max(a.1, b.1);
            repeat(x).zip(min_y..=max_y).collect()
        },
        (a, b) if a.1 == b.1 => {
            let y = a.1;
            let min_x = cmp::min(a.0, b.0);
            let max_x = cmp::max(a.0, b.0);
            (min_x..=max_x).zip(repeat(y)).collect()
        },
        _ => Vec::new()
    }
}

fn diff(x: usize, y: usize) -> usize {
    let min_val = cmp::min(x, y);
    let max_val = cmp::max(x, y);

    return max_val - min_val + 1
}

fn queen_iterator(point_pair: &PointPair) -> Vec<Point> {
    match point_pair {
        (a, b) if diff(a.0, b.0) == diff(a.1, b.1) => {

            let x_range: Vec<usize> = if a.0 < b.0 {
                (a.0..=b.0).collect()
            } else {
                (b.0..=a.0).rev().collect()
            };

            let y_range: Vec<usize> = if a.1 < b.1 {
                (a.1..=b.1).collect()
            } else {
                (b.1..=a.1).rev().collect()
            };

            x_range.into_iter().zip(y_range.into_iter()).collect()
        },
        _ => rook_iterator(point_pair)
    }
}

fn solve1(point_pairs: &Vec<PointPair>) -> usize {
    let mut board_map = HashMap::new();

    for point_pair in point_pairs {
        for point in rook_iterator(point_pair) {
            let point_count = board_map.entry(point).or_insert(0);
            *point_count += 1;
        }
    }

    board_map.values().filter(|val| **val > 1).count()
}

fn solve2(point_pairs: &Vec<PointPair>) -> usize {
    let mut board_map = HashMap::new();

    for point_pair in point_pairs {
        for point in queen_iterator(point_pair) {
            let point_count = board_map.entry(point).or_insert(0);
            *point_count += 1;
        }
    }

    board_map.values().filter(|val| **val > 1).count()
}
