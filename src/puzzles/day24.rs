use std::{collections::{HashMap, HashSet}, fs};
use itertools::Itertools;
use solutions::misc::measure::measure;

#[derive(Clone, Debug, PartialEq)]
enum Errors {
    NoFile,
    InvalidInput,
}

pub fn solve() {
    let (p2, p1) = fs::read_to_string("../inputs/day24").map_err(|_| Errors::NoFile)
        .and_then(|input| parse(input))
        .map(|mut m| (measure(part2, &m), measure(part1, &mut m)))
        .unwrap();
    println!("P1: {p1:?}\nP2:{p2:?}");
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum OP { OR, AND, XOR }

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Gate {
    in1: String,
    in2: String,
    out: String,
    op: OP
}

#[derive(Debug, Clone)]
struct Machine {
    inputs: HashMap<String, u8>,
    finished: HashSet<Gate>,
    unfinished: HashSet<Gate>
}

impl Machine {
    fn new(gates:Vec<Gate>, inputs: HashMap<String, u8>) -> Self {
        Machine { inputs, finished: HashSet::new(), unfinished: gates.into_iter().collect() }
    }

    fn is_output(label: &String) -> bool {label.starts_with('z')}

    fn waiting_outputs(&self) -> HashSet<String> {
        self.unfinished.iter()
            .filter_map(|g| Self::is_output(&g.out).then_some(g.out.clone()))
            .collect()
    }

    fn calc(inputs: &HashMap<String, u8>, gate: &Gate) -> Option<u8>{
        let in1 = inputs.get(&gate.in1)?;
        let in2 = inputs.get(&gate.in2)?;
        match gate.op {
            OP::OR => Some(in1|in2),
            OP::AND => Some(in1&in2),
            OP::XOR => Some(in1^in2),
        }
    }

    fn output(&self) -> u64 {
        self.inputs.iter()
        .filter_map(|(l,v)| Self::is_output(l).then_some((l,v)))
            .sorted_by(|a,b| a.0.cmp(b.0))
            .rev()
            .fold(0, |acc,x| (acc * 2) + *x.1 as u64)
    }

    fn tick(&mut self) {
        let mut f = Vec::new();
        let mut i = Vec::new();
        let mut u = Vec::new();
        for g in self.unfinished.drain() {
            if let Some(v) = Self::calc(&self.inputs, &g) {
                i.push((g.out.clone(), v));
                f.push(g);
            } else { u.push(g);}
        }
        self.inputs.extend(i);
        self.finished.extend(f);
        self.unfinished = u.into_iter().collect();
    }
}

fn parse(input: String) -> Result<Machine, Errors> {
    let (ws, gs) = input.split_once("\n\n").ok_or(Errors::InvalidInput)?;
    let wires = Ok(ws).and_then(
        |sw| sw.lines().map(
            |sl| {
                let (l, v) = sl.split_once(": ").ok_or(Errors::InvalidInput)?;
                let label = l.to_string(); 
                let value = v.parse().map_err(|_| Errors::InvalidInput)?;
                Ok((label, value))
            })
            .collect());
    let gates = Ok(gs).and_then(|sg| sg.lines().map(
        |sl| {
            let (lhs, rhs) = sl.split_once(" -> ").ok_or(Errors::InvalidInput)?;
            let tokens = lhs.split_whitespace().collect_vec();
            Ok(Gate { 
                in1: tokens[0].to_string(),
                in2: tokens[2].to_string(),
                out: rhs.to_string(),
                op: match tokens[1] {
                    "OR" => Ok(OP::OR),
                    "AND" => Ok(OP::AND),
                    "XOR" => Ok(OP::XOR),
                    _ => Err(Errors::InvalidInput)
                }?
            })
        })
        .collect());
    Ok(Machine::new(gates?, wires?))
}

fn part1(m:&mut Machine) -> u64 {
    while !m.waiting_outputs().is_empty() { m.tick(); }
    m.output()
}

fn part2(m:&Machine) -> String {
    let gs: HashMap<(&String, &String, OP), &Gate> = m.unfinished.iter()
        .map(|g| ((&g.in1, &g.in2, g.op), g))
        .collect();
    let mut swapped = Vec::new();
    for g in &m.unfinished {
        if swapped.contains(&&g.out) { continue; }
        //g is OR/AND and ouputs zXX - swap with 2nd xor from xXX yXX -> xor ->xor
        if g.op != OP::XOR && g.out.starts_with('z') {
            let idx = g.out.strip_prefix('z').unwrap();
            if idx == "45" {continue;} //last one is fine
            let (p1,p2) = (String::from("x")+idx, String::from("y")+idx);
            let x1 = gs.get(&(&p1, &p2, OP::XOR)).or(gs.get(&(&p2,&p1,OP::XOR))).unwrap();
            let x2 = m.unfinished.iter().find(|g| g.op == OP::XOR && (g.in1 == x1.out || g.in2 == x1.out)).unwrap();
            swapped.push(&g.out);
            swapped.push(&x2.out);
        }

        //XOR connected to OR - swap with AND from same inputs
        if g.op == OP::XOR {
            if m.unfinished.iter().find(|x| x.op == OP::OR && (x.in2 == g.out || x.in1 == g.out)).is_some() {
                let x = gs.get(&(&g.in1, &g.in2, OP::AND)).or(gs.get(&(&g.in2,&g.in1,OP::AND))).unwrap();
                swapped.push(&g.out);
                swapped.push(&x.out);
            }
        }
    }
    swapped.iter().sorted().join(",")
}

fn op_str(op:&OP) -> String {
    match op {
        OP::OR => String::from("Or"),
        OP::AND => String::from("And"),
        OP::XOR => String::from("Xor"),
    }
}

fn print_dot(m:&Machine) {
    println!("copy to graphwiz and analyze manually:");
    for (l,v) in &m.inputs {
        println!("{l}[label=\"{l}({v})\"]")
    }
    let mut ns = Vec::new();
    let mut es = Vec::new();
    for (i,g) in m.unfinished.iter().enumerate() {
        let l = op_str(&g.op);
        let node = format!("{l}{i}");
        ns.push(format!("{node}[label={l}]"));
        for (j, d) in m.unfinished.iter().enumerate() {
            if g.out == d.in1 || g.out == d.in2 {
            let ld = op_str(&d.op);
            let nd = format!("{ld}{j}");
                es.push(format!("{node}->{nd}[label={x}]", x = g.out));
            }

        }
        if g.in1.starts_with('x') || g.in1.starts_with('y') {
            es.push(format!("{inp}->{node}", inp = g.in1));
        }
        if g.in2.starts_with('x') || g.in2.starts_with('y') {
            es.push(format!("{inp}->{node}", inp = g.in2));
        }

        if g.out.starts_with('z') {
            es.push(format!("{node}->{out}", out = g.out));
        }
    }

    ns.iter().for_each(|s| println!("{s}"));
    es.iter().for_each(|s| println!("{s}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_p1() {
        let i = String::from("x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
");
        let mut m = parse(i).unwrap();
        assert_eq!(part1(&mut m), 2024);
    }
}
