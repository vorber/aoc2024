use std::{collections::{HashMap, HashSet}, fs, num::ParseIntError};
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
enum Errors {
    NoFile,
    InvalidInput,
}

pub fn solve() {
    let (p1, p2) = fs::read_to_string("../inputs/day22").map_err(|_| Errors::NoFile)
        .and_then(|input| parse(input).map_err(|_| Errors::InvalidInput))
        .map(|ns| (part1(&ns), part2(&ns)))
        .unwrap();
    println!("P1: {p1}\nP2:{p2}");
}

fn parse(input: String) -> Result<Vec<i64>, ParseIntError> {
    input.lines().map(|l| Ok(l.parse()?)).collect()
}

fn part1(numbers:&Vec<i64>) -> i64 {
    numbers.into_iter()
        .map(|n| seq(*n)[2000])
        .sum()
}

fn part2(numbers:&Vec<i64>) -> i64 {
    let prices = numbers.into_iter().map(|n| seq(*n).iter().map(|v| v % 10).collect_vec()).collect_vec();
    let mut sums = HashMap::new();
    for s in prices.into_iter() {
        let mut m = HashSet::new();
        for i in 0..s.len()-4 {
            let p = (s[i+1]-s[i], s[i+2]-s[i+1], s[i+3]-s[i+2], s[i+4]-s[i+3]);
            let v = s[i+4];
            if m.insert(p) { sums.entry(p).and_modify(|e| *e += v).or_insert(v); };
        }
    }
    *sums.values().max().unwrap()
}

fn seq(number:i64) -> Vec<i64> {
    let mut res = Vec::new();
    let mut x = number;
    for _ in 0..=2000 { res.push(x); x = next(x); }
    res
}

fn next(number:i64) -> i64 {
    let mut x = number;
    x = (x ^ (x << 6)) % 0x1000000; 
    x = (x ^ (x >> 5)) % 0x1000000;
    x = (x ^ (x << 11)) % 0x1000000;
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_p1() {
        let i = String::from("1
10
100
2024
");
        let ns = parse(i).unwrap();
        assert_eq!(part1(&ns), 37327623);
    }

    #[test]
    fn sample_data_p2() {
        let i = String::from("1
2
3
2024
");
        let ns = parse(i).unwrap();
        assert_eq!(part2(&ns), 23);
    }
}
