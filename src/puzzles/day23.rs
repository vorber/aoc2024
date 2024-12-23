use std::{collections::{HashMap, HashSet}, fs, hash::Hash};
use itertools::Itertools;
use solutions::misc::measure::measure;

#[derive(Clone, Debug, PartialEq)]
enum Errors {
    NoFile,
    InvalidInput,
}

pub fn solve() {
    let (p1, p2) = fs::read_to_string("../inputs/day23").map_err(|_| Errors::NoFile)
        .and_then(|input| parse(input))
        .map(|ns| (measure(part1, &ns), measure(part2, &ns)))
        .unwrap();
    println!("P1: {p1:?}\nP2:{p2:?}");
}

fn parse(input: String) -> Result<Vec<(String, String)>, Errors> {
    input.lines()
        .map(|l| l
            .split_once('-').ok_or(Errors::InvalidInput)
            .map(|p| (p.0.to_string(), p.1.to_string())))
        .collect()
}

fn part1(es:&Vec<(String,String)>) -> usize {
    let mut neighbors = HashSet::new();
    let mut vs = HashSet::new();
    for (a,b) in es {
        vs.insert(a);
        vs.insert(b);
        neighbors.insert((a,b));
        neighbors.insert((b,a));
    }
    let mut l3 = 0;
    for (a,b) in es {
        for v in &vs {
            if !a.starts_with("t") && !b.starts_with("t") && !v.starts_with("t") { continue; } 
            if neighbors.contains(&(v,a)) && neighbors.contains(&(b,v)) {
                l3+=1;
            }
        }
    }
    l3/3
}

fn part2(es:&Vec<(String,String)>) -> String {
    let mut neighbors = HashMap::new();
    let mut vs = HashSet::new();
    for (a,b) in es {
        let v1 = Vertex {label: a}; let v2 = Vertex {label: b};
        vs.insert(v1); vs.insert(v2);
        neighbors.entry(v1).and_modify(|vs: &mut HashSet<Vertex>| {vs.insert(v2);}).or_insert(HashSet::from([v2]));
        neighbors.entry(v2).and_modify(|vs: &mut HashSet<Vertex>| {vs.insert(v1);}).or_insert(HashSet::from([v1]));
    }
    bron_kerbosch(vs, HashSet::new(), HashSet::new(), &neighbors).iter()
        .max_by(|c1,c2| c1.len().cmp(&c2.len()))
        .map(|vs| vs.iter().map(|v| v.label).sorted().join(","))
        .unwrap_or(String::new())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex<'a> {
    label: &'a str
}

fn bron_kerbosch<'a>(mut p: HashSet<Vertex<'a>>, r:HashSet<Vertex<'a>>, mut x: HashSet<Vertex<'a>>, n:&HashMap<Vertex<'a>, HashSet<Vertex<'a>>>) -> Vec<HashSet<Vertex<'a>>> {
    if p.is_empty() && x.is_empty() { return vec![r]; }
    let mut res = Vec::new();
    while let Some(v) = pop(&mut p) {
        let mut nr = r.clone(); nr.insert(v);
        let np = p.clone().intersection(&n[&v]).copied().collect();
        let nx = x.clone().intersection(&n[&v]).copied().collect();
        res.extend(bron_kerbosch(np, nr, nx, n));
        x.insert(v);
    }
    res
}

fn pop<T>(s: &mut HashSet<T>) -> Option<T> 
where T: Hash + Eq + Copy
{
    let e = s.iter().next().copied()?;
    s.take(&e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_p1() {
        let i = String::from("kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
");
        let es = parse(i).unwrap();
        assert_eq!(part1(&es), 7);
        assert_eq!(part2(&es), "co,de,ka,ta");
    }
}
