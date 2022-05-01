use std::cmp::max;
use std::collections::HashSet;

type XY = (usize, usize);
type Paper = HashSet<XY>;
type PaperBounds = XY;
type FoldInstructions = Vec<XY>;

pub fn solve(file_string: &str) {
    let (paper, paper_bounds, fold_instructions) = parse(file_string);

    let (first_fold_paper, first_fold_paper_bounds) =
        solve1(&paper, &paper_bounds, &fold_instructions);
    println!("{}", first_fold_paper.len());

    let (final_paper, final_paper_bounds) = solve2(
        &first_fold_paper,
        &first_fold_paper_bounds,
        &fold_instructions[1..],
    );

    print_paper(&final_paper, &final_paper_bounds);
}

fn print_paper(paper: &Paper, paper_bounds: &PaperBounds) {
    for y in 0..=paper_bounds.1 {
        for x in 0..=paper_bounds.0 {
            print!("{}", if paper.contains(&(x, y)) { "#" } else { " " });
        }
        println!();
    }
}

fn parse_fold_instructions(fold_instructions_str: &str) -> FoldInstructions {
    fold_instructions_str
        .lines()
        .map(|line| {
            let fold_value = line.split_once("=").unwrap().1.parse::<usize>().unwrap();
            if line.starts_with("fold along y=") {
                (0, fold_value)
            } else {
                (fold_value, 0)
            }
        })
        .collect()
}

fn parse_paper(marks_str: &str) -> (Paper, PaperBounds) {
    let mut paper = Paper::new();
    let mut paper_bounds: PaperBounds = (0, 0);

    for line in marks_str.lines() {
        let split = line.split_once(",").unwrap();
        let x = split.0.parse::<usize>().unwrap();
        let y = split.1.parse::<usize>().unwrap();
        let mark = (x, y);
        paper.insert(mark);

        paper_bounds.0 = max(x, paper_bounds.0);
        paper_bounds.1 = max(y, paper_bounds.1);
    }

    (paper, paper_bounds)
}

pub fn parse(file_string: &str) -> (Paper, PaperBounds, FoldInstructions) {
    let (paper_str, fold_instructions_str) = file_string.split_once("\n\n").unwrap();
    let (paper, paper_bounds) = parse_paper(paper_str);
    let fold_instructions = parse_fold_instructions(fold_instructions_str);

    (paper, paper_bounds, fold_instructions)
}

fn fold_once(
    paper: &Paper,
    paper_bounds: &PaperBounds,
    fold_instruction: &XY,
) -> (Paper, PaperBounds) {
    let mut new_paper = Paper::new();

    let fold_horizontally = fold_instruction.0 > 0;

    for mark in paper {
        if fold_horizontally {
            if mark.0 > fold_instruction.0 {
                new_paper.insert((paper_bounds.0 - mark.0, mark.1));
            } else {
                new_paper.insert(mark.clone());
            }
        } else {
            if mark.1 > fold_instruction.1 {
                new_paper.insert((mark.0, paper_bounds.1 - mark.1));
            } else {
                new_paper.insert(mark.clone());
            }
        }
    }

    let new_paper_bounds = if fold_horizontally {
        (fold_instruction.0 - 1, paper_bounds.1)
    } else {
        (paper_bounds.0, fold_instruction.1 - 1)
    };

    (new_paper, new_paper_bounds)
}

pub fn solve1(
    paper: &Paper,
    paper_bounds: &PaperBounds,
    fold_instructions: &[XY],
) -> (Paper, PaperBounds) {
    fold_once(&paper, &paper_bounds, &fold_instructions[0])
}

pub fn solve2(
    paper: &Paper,
    paper_bounds: &PaperBounds,
    fold_instructions: &[XY],
) -> (Paper, PaperBounds) {
    let mut new_paper = paper.clone();
    let mut new_paper_bounds = paper_bounds.clone();

    for fold_instruction in fold_instructions {
        let result = fold_once(&new_paper, &new_paper_bounds, &fold_instruction);

        new_paper = result.0;
        new_paper_bounds = result.1;
    }

    (new_paper, new_paper_bounds)
}
