use std::{fs, sync::LazyLock, u64};
use regex::{Captures, Regex};
static MUL_RE : &str = r"mul\((?<a>\d+),(?<b>\d+)\)";
static RE_MUL : LazyLock<Regex> = LazyLock::new(||Regex::new(MUL_RE).unwrap());

fn mul(s:&str) -> u64 { RE_MUL.captures(s) .map_or_else(|| 0, mul_cap) }

fn mul_cap<'h>(cs:Captures<'h>) -> u64 { cs["a"].parse::<u64>().unwrap() * cs["b"].parse::<u64>().unwrap() }

fn d3p1(data:&String) -> u64 { RE_MUL.captures_iter(data).map(mul_cap).sum() }

fn d3p2(data:&String) -> u64 {
    let re = Regex::new(&(format!("{}|{}", r"don't\(\)|do\(\)", MUL_RE))).unwrap();
    re.find_iter(data)
        .scan((1,0), |state, m| {
            *state = match m.as_str() {
                "don't()" => (0, state.1),
                "do()" => (1, state.1),
                s => (state.0, state.1 + state.0*mul(s))
            };
            Some(*state)
        })
        .map(|(_,sum)| sum)
        .last()
        .unwrap_or(0)
}

pub fn solve() {
    let data = fs::read_to_string("../inputs/day3").expect("Should be able to read input");
    println!("P1: {p1}", p1 = d3p1(&data));
    println!("P2: {p2}", p2 = d3p2(&data));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_p1() {
        let data = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(d3p1(&data), 161);
    }

    #[test]
    fn sample_data_p2() {
        let data = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(d3p2(&data), 48);
    }

    #[test]
    fn p2() {
        let data = String::from("ccmul(2,4) don't() mul(1,19) don't() mul(3,5)do()aaamul(1,1) do() mul(1,17) aaa don't()");
        assert_eq!(d3p2(&data), 26);
    }
}
