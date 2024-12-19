use std::{collections::{HashMap, HashSet}, fs};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
enum Errors {
    NoFile,
    CantParse,
}

struct Input {
    available_stripes: HashSet<String>,
    target_designs: Vec<String>
}

pub fn solve() {
    fs::read_to_string("../inputs/day19").map_err(|_| Errors::NoFile).and_then(
        |input| parse_input(&input).and_then(|input| {
            let counts = counts(&input).filter(|x| *x > 0).collect_vec();
            println!("P1: {p1:?}", p1 = counts.len());
            println!("P2: {p2:?}", p2 = counts.iter().sum::<usize>());
            Ok(())
        }))
        .unwrap();
}

fn parse_input(input:&String) -> Result<Input, Errors> {
    let (a,t) = input.split_once("\n\n").ok_or(Errors::CantParse)?;
    let a = a.split(",").map(|s| s.trim().to_string()).collect();
    let t = t.lines().map(|s| s.to_string()).collect_vec();
    Ok(Input {available_stripes: a, target_designs: t})
}

fn counts<'a>(input:&'a Input) -> impl Iterator<Item = usize> + use<'a> {
    let mut known = HashMap::from([("",1)]);
    input.target_designs.iter().map(move |target| count(target.as_str(), &input.available_stripes, &mut known))
}

fn count<'a>(t: &'a str, a:&'a HashSet<String>, known: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(r) = known.get(t) { return *r; }
    let r = a.iter()
        .filter_map(|p| t.strip_prefix(p))
        .map(|t| count(t, a, known))
        .sum();
    known.entry(t).and_modify(|e| *e = r).or_insert(r);
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_small() {
        let i = String::from("r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
");
        let i = parse_input(&i).unwrap();
        let counts = counts(&i).filter(|x| *x>0).collect_vec();
        assert_eq!(counts.len(), 6);
        assert_eq!(counts.iter().sum::<usize>(), 16);
    }
}
