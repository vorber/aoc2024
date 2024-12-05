use std::{cmp::Ordering, collections::HashSet, convert::identity, fs};

pub fn solve() {
    let data = fs::read_to_string("../inputs/day5").expect("Should be able to read input");
    println!("P1: {p1}", p1 = part1(&data));
    println!("P2: {p2}", p2 = part2(&data));
}

fn part1(data:&String) -> usize { _solve(data, identity) }
fn part2(data:&String) -> usize { _solve(data,|b| !b) }

fn _solve(data:&String, f:impl Fn(bool) -> bool) -> usize {
    let (cmp, originals) = parse(data);
    let mut sorted: Vec<_>= originals.iter().map(|s| s.clone()).collect();
    sorted.iter_mut().for_each(|s| s.sort_unstable_by(&cmp));
    sorted.iter().zip(originals)
        .filter(|(ss,o)| f(ss.eq(&o)))
        .map(|(s,_o)| s[s.len()/2] as usize)
        .sum()
}

fn get_cmp(rules:HashSet<(u8,u8)>) -> impl Fn(&u8,&u8) -> Ordering {
    move |a:&u8, b:&u8| 
    if rules.contains(&(*a,*b)) { Ordering::Less }
    else if rules.contains(&(*b,*a)) {Ordering::Greater}
    else {Ordering::Equal}
}

fn parse(data:&String) -> (impl Fn(&u8,&u8) -> Ordering, Vec<Vec<u8>>) {
    let mut split = data.split("\n\n");
    let (rules_str, seq_str) = (split.next().unwrap(), split.next().unwrap());
    let rules = rules_str.lines()
        .map(|l| {
            let mut s = l.split('|').map(|x| x.parse::<u8>().unwrap());
            (s.next().unwrap(), s.next().unwrap())
        })
        .collect::<HashSet<_>>();
    let seq = seq_str.lines()
        .map(|l| l.split(',').map(|x|x.parse::<u8>().unwrap()).collect())
        .collect();
    (get_cmp(rules), seq)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let data = String::from("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
");
        assert_eq!(part1(&data), 143);
        assert_eq!(part2(&data), 123);
    }
}
