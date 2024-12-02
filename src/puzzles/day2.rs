use std::fs;
use itertools::Itertools;

fn parse_line(l:&str) -> Vec<i64> {
    l.split_whitespace()
        .map(|x| x.parse::<i64>().expect("Should be an int"))
        .collect()
}

fn parse(data:&String) -> Vec<Vec<i64>> {
    data.lines().map(parse_line).collect()
}

fn is_safe_p1<'a>(ri:impl Iterator<Item = &'a i64>) -> bool {
    let ds = ri
        .tuple_windows::<(_,_)>()
        .map(|(a,b)| a - b)
        .collect::<Vec<_>>();

    let inc = ds.iter().all(|&x| x > 0);
    let dec = ds.iter().all(|&x| x < 0);
    let lim = ds.iter().all(|&x| x.abs() < 4);
    (inc | dec) & lim
}

fn check<'a>(mut i: impl Iterator<Item = &'a i64>) -> bool {
    let mut unsafe_levels = 0;

    let Some(mut prev) = i.next() else {panic!("empty report")};
    let Some(mut next) = i.next() else {return true;};
    let dir = (next - prev).signum();
    loop {
        let d = next - prev;
        if (d == 0) | (d.signum() != dir) | (d.abs() > 3) {
            if unsafe_levels == 0 {
                unsafe_levels += 1;
                let Some(n) = i.next() else {return true;};
                next = n;
                continue;
            }
            else {
                return false;
            }
        }
        prev = next;
        let Some(n) = i.next() else {return true;};
        next = n;
    }
}

fn is_safe_p2(report:&Vec<i64>) -> bool {
    check(report.iter()) | check(report.iter().rev())
}

fn d2p1(data:&String) -> i64 {
    parse(data).iter().filter(|&r| is_safe_p1(r.iter())).count() as i64
}

fn d2p2(data:&String) -> i64 {
    parse(data).iter().filter(|&r| is_safe_p2(r)).count() as i64
}

pub fn solve() {
    let data = fs::read_to_string("../inputs/day2").expect("Should be able to read input");
    println!("P1: {p1}", p1 = d2p1(&data));
    println!("P2: {p2}", p2 = d2p2(&data));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let data = String::from("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(d2p1(&data), 2);
        assert_eq!(d2p2(&data), 4);
    }
}

