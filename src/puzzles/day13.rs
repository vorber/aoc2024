use std::{fmt::Debug, fs, iter::Sum, str::FromStr};

use itertools::Itertools;
use num::{FromPrimitive, Integer, ToPrimitive};

pub fn solve() {
    let input = fs::read_to_string("../inputs/day13").expect("Should be able to read input");
    println!("P1: {p1}", p1 = part1(&ugly_parse::<i32>(&input)));
    println!("P2: {p2}", p2 = part2(&ugly_parse::<i128>(&input)));
}

fn ugly_parse<T>(data:&String) -> Vec<(T,T,T,T,T,T)> 
where T:FromStr, <T as FromStr>::Err: Debug {
    data.split("\n\n")
        .map(|ll| {
            let mut l = ll.split('\n');
            let (a1s, b1s) = l.next().unwrap().split_once(", Y+").unwrap();
            let a1 = a1s.chars().skip("Button A: X+".len()).collect::<String>().parse::<T>().unwrap();
            let b1 = b1s.parse().unwrap();
            let (a2s, b2s) = l.next().unwrap().split_once(", Y+").unwrap();
            let a2 = a2s.chars().skip("Button A: X+".len()).collect::<String>().parse::<T>().unwrap();
            let b2 = b2s.parse().unwrap();
            let (c1s, c2s) = l.next().unwrap().split_once(", Y=").unwrap();
            let c1 = c1s.chars().skip("Prize: X=".len()).collect::<String>().parse::<T>().unwrap();
            let c2 = c2s.parse().unwrap();
            (a1,b1,c1,a2,b2,c2)
        })
        .collect_vec()
}

fn part1(data:&Vec<(i32,i32,i32,i32,i32,i32)>) -> usize {
    _solve(data, |x| x <= 100)
}

fn part2(data:&Vec<(i128,i128,i128,i128,i128,i128)>) -> usize {
    let fixed = data.into_iter()
        .map(move |(a1,a2,a3,a4,a5,a6)| (*a1,*a2, 10000000000000 + *a3, *a4, *a5, 10000000000000 + *a6))
        .collect_vec();
    _solve(&fixed, |_x| true)
}

fn _solve<T>(data:&Vec<(T,T,T,T,T,T)>, limit: impl Fn(T)->bool) -> usize 
where T:Integer+ToPrimitive+FromPrimitive+Sum+Copy {
    data.into_iter()
        .filter_map(|vs| filter(*vs, &limit))
        .sum::<T>()
        .to_u64().unwrap() as usize
}

fn filter<T>(vs:(T,T,T,T,T,T), extra: &impl Fn(T) -> bool) -> Option<T> where T: Integer+FromPrimitive+Copy {
    let three = T::from_i8(3).unwrap();
    let (a1,b1,c1,a2,b2,c2) = vs;
    let d = det(a1,a2,b1,b2);
    let d1 = det(c1,a2,c2,b2);
    let d2 = det(a1,c1,b1,c2);
    (!d.is_zero() && (d1 % d).is_zero() && (d2 % d).is_zero() && extra(d1/d) && extra(d2/d))
        .then_some(three*d1/d + d2/d)
}

fn det<T>(a1:T, a2:T, b1:T, b2:T) -> T where T:Integer { a1*b2 - a2*b1 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let i = String::from("Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
");

        assert_eq!(part1(&ugly_parse(&i)), 480);
        assert_eq!(part2(&ugly_parse(&i)), 875318608908);
    }
}
