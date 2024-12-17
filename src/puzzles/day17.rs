use std::{fs, u64};

use itertools::Itertools;

pub fn solve() {
    let input = fs::read_to_string("../inputs/day17").expect("Should be able to read input");
    let (abc, program) = parse_input(&input);
    println!("P1: {p1}", p1 = part1(abc, &program));
    println!("P2: {p2}", p2 = part2(&program));
}

pub fn run_machine(abc:(u32,u32,u32), program: &Vec<u8>) -> Vec<u32> {
    let mut out = Vec::new();
    let (mut a, mut b, mut c) = abc;
    let mut ip = 0;
    while ip < program.len() {
        let op = program[ip]; let arg = program[ip+1] as u32;
        print_machine_op(op, arg, a, b, c, ip);
        ip += 2;
        match op {
            0 => a = a/(1 << combo_op(arg,a,b,c)),
            1 => b = b ^ arg,
            2 => b = combo_op(arg,a,b,c) % 8,
            3 => ip = if a == 0 {ip+2} else {arg as usize},
            4 => b = b ^ c,
            5 => out.push(combo_op(arg,a,b,c) % 8),
            6 => b = a/(1 << combo_op(arg,a,b,c)),
            7 => c = a/(1 << combo_op(arg,a,b,c)),
            _ => panic!("invalid operation")
        }
    }
    out
}

fn print_machine_op(op:u8, arg: u32, a:u32, b:u32, c:u32, ip:usize) {
    print!("State: A={a:10},B={b:10},C={c:10}; IP={ip:2} ");
    let combo=combo_str(arg, a, b, c);
    let cv = combo_op(arg, a, b, c);
    let os = match op {
        0 => format!("a/2**{combo} == {r} >> a", r = a/(1<<cv)),
        1 => format!("b({b}) xor {arg} == {r} >> b", r = b ^ arg),
        2 => format!("{combo} % 8 = {r} >> b", r = cv % 8),
        3 => format!("jnz (a={a})\n"),
        4 => format!("b({b}) xor c({c}) == {r} >> b", r=b^c),
        5 => format!("out {combo} == {r}", r=cv % 8),
        6 => format!("a/2**{combo} == {r} >> b", r=a/(1<<cv)),
        7 => format!("a/2**{combo} == {r} >> c", r=a/(1<<cv)),
        _ => "invalid!".to_string(),
    };
    println!("Running {os}");
}

fn combo_str(arg:u32, a:u32, b:u32, c:u32) -> String {
    match arg {
        v@0|v@1|v@2|v@3 => format!("L{v}"),
        4 => format!("A({a})"),
        5 => format!("B({b})"),
        6 => format!("C({c})"),
        _ => panic!()
    }
}

fn combo_op(arg: u32, a:u32, b: u32, c: u32) -> u32 {
    match arg {
        v@0 | v@1 | v@2 | v@3 => v,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("invalid combo operand")
    }
}

fn parse_input(input:&String) -> ((u32,u32,u32), Vec<u8>) {
    let data = input.split_once("\n\n").unwrap();
    let regs = data.0.lines().map(|l| l.chars().skip("Register X: ".len()).collect::<String>().parse::<u32>().unwrap()).collect_vec();
    let p = data.1.chars()
        .skip("Program: ".len())
        .collect::<String>()
        .split(',')
        .map(|v| v.trim().parse::<u8>().unwrap())
        .collect_vec();
    ((regs[0], regs[1], regs[2]), p)
}

fn part1(abc:(u32,u32,u32), p:&Vec<u8>) -> String {
    run_machine(abc, p).iter()
        .map(|v| v.to_string())
        .join(",")
}

fn part1_decompiled(a: u32) -> String {
    let mut a = a;
    let mut out=Vec::new();
    while a>0 {
        out.push(((a>>((a%8)^2))^((a%8)^1))%8);
        a/=8;
    }
    out.iter().map(|v| v.to_string()).join(",")
}

fn part2(p:&Vec<u8>) -> u64 {
    let r = p.iter().rev().collect_vec();
    smallest_fit(&r.as_slice(), 0).unwrap()
}

#[derive(Debug)]
struct DidntFit;
fn smallest_fit(numbers:&[&u8], preset:u64) -> Result<u64, DidntFit> {
    match numbers {
        [] => Ok(preset),
        [&head, tail@..] => (0..8).map(|s| (preset << 3) + s)
            .filter(|c| (c>>((c%8)^2)) % 8 == (c%8)^(head as u64)^1)
            .filter_map(|p| smallest_fit(tail, p).ok())
            .min()
            .ok_or(DidntFit)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_1() {
        assert_eq!(run_machine((10, 0, 0), &vec![5,0,5,1,5,4]), vec![0,1,2]);
        assert_eq!(run_machine((2024, 0, 0), &vec![0,1,5,4,3,0]), vec![4,2,5,6,7,7,7,7,3,1,0]);
    }

    #[test]
    fn sample_data_1() {
        let i = String::from("Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
");
        let (m,p) = parse_input(&i);
        assert_eq!(part1(m, &p), "4,6,3,5,6,3,5,2,1,0");
    }

}
