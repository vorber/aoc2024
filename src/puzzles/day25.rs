use std::fs;
use itertools::Itertools;
use solutions::misc::measure::measure;

#[derive(Clone, Debug, PartialEq)]
enum Errors {
    NoFile,
    InvalidInput,
}

pub fn solve() {
    let (p1, p2) = fs::read_to_string("../inputs/day25").map_err(|_| Errors::NoFile)
        .and_then(|input| parse(input))
        .map(|x| (measure(part1, &x), measure(part2, &x)))
        .unwrap();
    println!("P1: {p1:?}\nP2:{p2:?}");
}

fn parse(input: String) -> Result<(Vec<[u8;5]>, Vec<[u8;5]>), Errors> {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for e in input.split("\n\n") {
        let l = e.lines()
            .map(|l| l.chars().map(|c| (c=='#') as u8).collect_vec())
            .fold(
                Ok([0,0,0,0,0]), 
                |acc,x| Ok(sum(&acc?,&x.try_into().map_err(|_| Errors::InvalidInput)?)));
        if e.starts_with('#') {locks.push(l?)} else {keys.push(l?)};
    }

    Ok((keys, locks))
}

fn sum(a:&[u8;5], b:&[u8;5]) -> [u8;5] {
    a.iter().zip(b.iter()).map(|(a,b)| a+b).collect_vec().try_into().unwrap()
}

fn part1(inputs:&(Vec<[u8;5]>, Vec<[u8;5]>)) -> usize {
    let (locks, keys) = inputs;
    locks.iter()
        .cartesian_product(keys.iter())
        .filter_map(|(l,k)| sum(l,k).iter()
            .all(|v| *v<=7)
            .then_some(1))
        .sum()
}

fn part2(_:&(Vec<[u8;5]>, Vec<[u8;5]>)) -> String {
    "Merry Christmas!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_p1() {
        let i = String::from("#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
");
        let es = parse(i).unwrap();
        assert_eq!(part1(&es), 3);
        assert_eq!(part2(&es), "Merry Christmas!");
    }
}
