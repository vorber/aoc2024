use std::{collections::HashMap, fs, i64};

fn parse_line(l:&str) -> (i64, i64) {
    let parts :Vec<i64> = l
        .split_whitespace()
        .map(|x| x.parse::<i64>().expect("Should be an int"))
        .collect(); 
    (parts[0], parts[1])
}

fn parse(data:&String) -> (Vec<i64>, Vec<i64>) {
    data.lines().map(parse_line).unzip()
}

fn d1p1(data:&String) -> i64 {
    let (mut l, mut r) = parse(data);
    l.sort_unstable();
    r.sort_unstable();
    l.iter().zip(r.iter()).map(|(l,r)| (l-r).abs()).sum()
}

fn d1p2(data:&String) -> i64 {
    let (l,r) = parse(data);

    let mut rf = HashMap::new();
    for k in r {
        *rf.entry(k).or_insert(0) += 1;
    }

    l.iter()
        .map(|&x| x * rf.get(&x).unwrap_or(&0))
        .sum()
}

pub fn solve() {
    let data = fs::read_to_string("../inputs/day1_1").expect("Should be able to read input");
    println!("P1: {p1}", p1 = d1p1(&data));
    println!("P2: {p2}", p2 = d1p2(&data));
}
