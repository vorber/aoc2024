use std::fs;

type Number = i128;

pub fn solve() {
    let data = fs::read_to_string("../inputs/day7").expect("Should be able to read input");
    println!("P1: {p1}", p1 = part1(&data));
    println!("P2: {p2}", p2 = part2(&data));
}

fn part1(data:&String) -> Number { _solve(data, false) }
fn part2(data:&String) -> Number { _solve(data, true) }

fn _solve(data:&String, p2:bool) -> Number {
    data.lines()
        .map(parse_line)
        .filter(|(t,xs)| try_produce(*t, xs.as_slice(), p2))
        .map(|(t,_xs)| t)
        .sum()
}

fn parse_line(line:&str) -> (Number, Vec<u16>) {
    let (tt,xx) = line.split_once(':').unwrap();
    let t = tt.parse().unwrap();
    let xs = xx.split_whitespace().map(|s| s.parse().unwrap()).collect();
    (t,xs)
}

enum Ops { Add, Mul, Conc }

fn try_produce(target: Number, xs:&[u16], p2:bool) -> bool {
    match xs {
        [] => false,
        [x] => target == *x as Number,
        [rest @ .., x] => {
            let x = *x as Number;
            let l = l10(x);
            let mut ops = vec![Ops::Add];
            if p2 && target % l == x { ops.push(Ops::Conc); }
            if target % x == 0 { ops.push(Ops::Mul); }
            let rec  = |t| try_produce(t, rest, p2);
            try_ops(ops, target, x, rec)
        }
    }
}

fn try_ops(ops: Vec<Ops>, target: Number, x:Number, try_rec:impl Fn(Number) -> bool) -> bool {
    ops.iter()
        .map(|op| {
            match op {
                Ops::Add => try_rec(target-x),
                Ops::Mul => try_rec(target/x),
                Ops::Conc => try_rec(target/l10(x))
            }})
        .fold(false, |acc, r| acc || r)
}

fn l10(n:Number) -> Number {
    let mut v = 1; let mut r = n;
    while r > 0 { r /= 10; v *= 10; }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let data = String::from("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
");

        assert_eq!(part1(&data), 3749);
        assert_eq!(part2(&data), 11387);
    }
}
