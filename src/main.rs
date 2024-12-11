use core::panic;
use std::env;

pub mod puzzles;

fn solve(day:&String) {
    match day.as_str() {
        "1" => puzzles::day1::solve(),
        "2" => puzzles::day2::solve(),
        "3" => puzzles::day3::solve(),
        "4" => puzzles::day4::solve(),
        "5" => puzzles::day5::solve(),
        "6" => puzzles::day6::solve(),
        "7" => puzzles::day7::solve(),
        "8" => puzzles::day8::solve(),
        "9" => puzzles::day9::solve(),
        "10" => puzzles::day10::solve(),
        "11" => puzzles::day11::solve(),
        _ => panic!("no solution known!")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    solve(day)
}
