use std::{collections::HashMap, fs, iter, usize};

use itertools::Itertools;
use solutions::misc::point::Point;

#[derive(Clone, Debug, PartialEq)]
enum Errors {
    NoFile,
    InvalidCodeNoSuffix,
    InvalidCodeNoNumeric,
    InvalidCodeCharacter
}

pub fn solve() {
    let (p1, p2) = fs::read_to_string("../inputs/day21").map_err(|_| Errors::NoFile)
        .and_then(|i| Ok((_solve(&i, 2)?, _solve(&i, 25)?)))
        .unwrap();
    println!("P1: {p1}\nP2:{p2}");
}

fn _solve(input:&String, robots: usize) -> Result<usize, Errors> {
    input.lines().into_iter()
        .map(|s| Ok((code_to_directions(s)?, numeric(s)?)))
        .map_ok(|(s,n)| n*shortest_seq(s.trim(),robots-1))
        .fold_ok(0, |a,b| a+b)
}

fn numeric(s:&str) -> Result<usize, Errors> {
    s.strip_suffix("A").ok_or(Errors::InvalidCodeNoSuffix)
        .and_then(|s| s.parse().map_err(|_| Errors::InvalidCodeNoNumeric))
}

fn code_to_directions(code:&str) -> Result<String, Errors> {
    fn position(c:char) -> Result<Point, Errors> {
        match c {
            '0' => Ok(Point::new(1, 3)),
            '1' => Ok(Point::new(0, 2)),
            '2' => Ok(Point::new(1, 2)),
            '3' => Ok(Point::new(2, 2)),
            '4' => Ok(Point::new(0, 1)),
            '5' => Ok(Point::new(1, 1)),
            '6' => Ok(Point::new(2, 1)),
            '7' => Ok(Point::new(0, 0)),
            '8' => Ok(Point::new(1, 0)),
            '9' => Ok(Point::new(2, 0)),
            'A' => Ok(Point::new(2, 3)),
            _ => Err(Errors::InvalidCodeCharacter)
        }
    }
    let mut prev = 'A';

    let mut res = String::from("");
    for c in code.chars() {
        let pp = position(prev)?;
        let pc = position(c)?;

        let d = pc-pp;
        let h = (d.x.abs() as usize, if d.x > 0 { Some('>') } else if d.x < 0 { Some('<') } else {None});
        let v = (d.y.abs() as usize, if d.y > 0 { Some('v') } else if d.y < 0 { Some('^') } else {None});
        let order = 
        if h.1 == Some('<') && pp.y == 3 && pc.x == 0 {[v,h]} 
        else if v.1 == Some('v') && pp.x == 0 && pc.y ==3 {[h,v]}
        else if h.1 == Some('<') {[h,v]}
        else {[v,h]};

        res += &order.iter().filter_map(|(n,o)| o.map(|c| str(c,*n))).join("");
        res.push('A');
        prev = c;
    }
    Ok(res)
}

fn str(c:char, n:usize) -> String { iter::repeat(c).take(n).collect::<String>() }

fn shortest_seq(s:&str, depth:usize) -> usize {
    let l0 = [
        (('A', 'A') ,"A"),    (('A', '^') ,"<A"),  (('A', '>') ,"vA"),  (('A', '<') ,"v<<A"), (('A', 'v') ,"<vA"),
        (('^', 'A') ,">A"),   (('^', '^') ,"A"),   (('^', '>') ,"v>A"), (('^', '<') ,"v<A"),  (('^', 'v') ,"vA"),
        (('v', 'A') ,"^>A"),  (('v', '^') ,"^A"),  (('v', '>') ,">A"),  (('v', '<') ,"<A"),   (('v', 'v') ,"A"),
        (('<', 'A') ,">>^A"), (('<', '^') ,">^A"), (('<', '>') ,">>A"), (('<', '<') ,"A"),    (('<', 'v') ,">A"),
        (('>', 'A') ,"^A"),   (('>', '^') ,"<^A"), (('>', '>') ,"A"),   (('>', '<') ,"<<A"),  (('>', 'v') ,"<A"),
    ].into_iter().map(|(k,v)| (k,v.to_string())).collect::<HashMap<_,_>>();

    let mut known = l0.clone().into_iter()
        .map(|(k,v)| ((k.0, k.1, 0), v.len()))
        .collect();

    fn shortest_char_seq<'a>(
        p:char, 
        c:char, 
        depth: usize, 
        l0: &HashMap<(char, char), String>,
        known: &'a mut HashMap<(char, char, usize), usize>) -> usize 
    {
        if known.contains_key(&(p,c,depth)) { return known[&(p,c,depth)];}
        let t = l0[&(p,c)].clone();
        let mut r = 0;
        let mut prev='A';
        for cc in t.chars() {
            r += shortest_char_seq(prev, cc, depth-1, l0, known);
            prev = cc;
        }
        known.insert((p,c,depth), r);
        known[&(p,c,depth)].clone()
    }

    let mut result = 0;
    let mut prev = 'A';
    for c in s.trim().chars() {
        result += shortest_char_seq(prev, c, depth, &l0, &mut known);
        prev = c;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_small() {
        let i = String::from("029A
980A
179A
456A
379A
");
        let p1 = _solve(&i,2).unwrap();
        assert_eq!(p1, 126384);
    }
}
