use std::{fs, str::FromStr};

use itertools::Itertools;
use solutions::misc::point::Point;

pub fn solve() {
    let input = fs::read_to_string("../inputs/day14").expect("Should be able to read input");
    let data = parse(&input);
    println!("P1: {p1}", p1 = part1(&data, 101, 103));
    println!("P2: {p2}", p2 = part2(&data, 101, 103));
}

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point
}

impl Robot {
    fn step(&self, seconds: usize, w:usize, h:usize) -> Point {
        (self.position + seconds*self.velocity).wrap(w, h)
    }
}

#[derive(Debug)]
struct ParseRobotError;

impl FromStr for Robot {
    type Err = ParseRobotError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ps, vs) = s.split_once(" ").ok_or(ParseRobotError)?;
        let (psx,psy) = ps.strip_prefix("p=").and_then(|s| s.split_once(",")).ok_or(ParseRobotError)?;
        let (vsx,vsy) = vs.strip_prefix("v=").and_then(|s| s.split_once(",")).ok_or(ParseRobotError)?;
        let px = psx.parse().map_err(|_| ParseRobotError)?;
        let py = psy.parse().map_err(|_| ParseRobotError)?;
        let vx = vsx.parse().map_err(|_| ParseRobotError)?;
        let vy = vsy.parse().map_err(|_| ParseRobotError)?;
        let p = Point::new(px, py);
        let v = Point::new(vx, vy);
        Ok(Robot {position:p, velocity: v})
    }
}

fn parse(input:&String) -> Vec<Robot> {
    input.lines().map(|l| l.parse().unwrap()).collect_vec()
}

fn part1(data:&Vec<Robot>, w: usize, h:usize) -> usize {
    quadrant_count_product(data, w, h, 100)
}

fn part2(data:&Vec<Robot>, w:usize, h:usize) -> usize {
    (1..=101*103)
        .map(|t| (t, quadrant_count_product(data, w, h, t)))
        .min_by(|&x,&y| x.1.cmp(&y.1))
        .unwrap()
    .0
}

fn quadrant_count_product(data:&Vec<Robot>, w:usize, h:usize, seconds:usize) -> usize {
     data.into_iter()
        .filter_map(|r| quadrant(r.step(seconds, w, h), w, h))
        .counts()
        .values()
        .product::<usize>()
}

fn quadrant(p:Point, w:usize, h:usize) -> Option<usize> {
    let w = (w/2) as i32;
    let h = (h/2) as i32;
    match p {
        Point {x,y} if x < w && y < h => Some(0),
        Point {x,y} if x > w && y < h => Some(1),
        Point {x,y} if x < w && y > h => Some(2),
        Point {x,y} if x > w && y > h => Some(3),
        _ => None
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let i = String::from("p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
");

        let data = parse(&i);
        assert_eq!(part1(&data, 11, 7), 12);
    }
}
