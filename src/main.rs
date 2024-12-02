use core::panic;
use std::env;

pub mod puzzles;

fn solve(day:&String) {
    match day.as_str() {
        "1" => puzzles::day1::solve(),
        "2" => puzzles::day2::solve(),
        _ => panic!("no solution known!")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    solve(day)
}
