use std::{collections::HashMap, fs, usize};

use itertools::Itertools;

pub fn solve() {
    let data = fs::read_to_string("../inputs/day11").expect("Should be able to read input");
    let numbers = data.split_whitespace().map(|s| s.parse::<u64>().expect("should be a number")).collect_vec();
    let mut counts = numbers.into_iter().counts();
    println!("P1: {p1}", p1 = _solve(25, &mut counts));
    println!("P2: {p2}", p2 = _solve(50, &mut counts));
}

fn _solve(cnt:usize, counts: &mut HashMap<u64,usize>) -> usize {
    (0..cnt).for_each(|_i| { blink_once(counts); });
    counts.values().sum()
}

fn blink_once(counts: &mut HashMap<u64,usize>) {
        let mut nxt = HashMap::new();
        counts.iter().for_each(|(&n,&c)| {
            blink_single(n).iter().for_each(|&x| { *nxt.entry(x).or_insert(0) += c; });
        });
        *counts = nxt;
}

fn blink_single(n:u64) -> Vec<u64> {
    if n==0 { return vec![1];}    
    let l = n.ilog10() + 1;
    if l % 2 == 0 { return vec![n/10u64.pow(l/2), n % 10u64.pow(l/2)];}
    vec![n*2024]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blink_num() {
        assert_eq!(blink_single(0), vec![1]);
        assert_eq!(blink_single(1), vec![2024]);
        assert_eq!(blink_single(2), vec![4048]);
        assert_eq!(blink_single(2024), vec![20,24]);
        assert_eq!(blink_single(20), vec![2,0]);
        assert_eq!(blink_single(123), vec![123*2024]);
    }

    #[test]
    fn sample_data() {
        let mut counts = [125, 17].into_iter().counts();
        assert_eq!(_solve(6, &mut counts), 22);
        assert_eq!(_solve(19, &mut counts), 55312);
    }
}
