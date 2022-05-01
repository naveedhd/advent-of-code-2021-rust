use std::fs;
use structopt::StructOpt;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

#[derive(StructOpt)]
struct Cli {
    day: String,
}

fn read_file_string(day: &str) -> String {
    fs::read_to_string(format!("./src/day{}/input", day)).unwrap()
}

fn main() {
    match Cli::from_args().day.as_str() {
        "1" => day01::solve(&read_file_string("01")),
        "2" => day02::solve(&read_file_string("02")),
        "3" => day03::solve(&read_file_string("03")),
        "4" => day04::solve(&read_file_string("04")),
        "5" => day05::solve(&read_file_string("05")),
        "6" => day06::solve(&read_file_string("06")),
        "7" => day07::solve(&read_file_string("07")),
        "8" => day08::solve(&read_file_string("08")),
        "9" => day09::solve(&read_file_string("09")),
        "10" => day10::solve(&read_file_string("10")),
        "11" => day11::solve(&read_file_string("11")),
        "12" => day12::solve(&read_file_string("12")),
        "13" => day13::solve(&read_file_string("13")),
        "14" => day14::solve(&read_file_string("14")),
        "15" => day15::solve(&read_file_string("15")),
        _ => {
            eprintln!("please provide day!");
        }
    }
}
