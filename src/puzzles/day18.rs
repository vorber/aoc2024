use std::{collections::{HashMap, HashSet}, fs, iter, usize};

use itertools::Itertools;
use solutions::misc::{graph::{Edge, Graph}, point::Point};

#[derive(Clone, Debug, PartialEq)]
enum Errors {
    NoFile,
    CantParse,
    NoPath
}

pub fn solve() {
    fs::read_to_string("../inputs/day18").map_err(|_| Errors::NoFile).and_then(
        |input| parse_input(&input).and_then(|obstacles| {
            println!("P1: {p1:?}", p1 = part1(&obstacles.clone().into_iter().take(1024).collect_vec(), 71));
            println!("P2: {p2:?}", p2 = part2(&obstacles, 71));
            Ok(())
        }))
        .unwrap();
}

fn parse_input(input:&String) -> Result<Vec<Point>, Errors> {
    input.lines()
        .map(|l| l.split_once(',').ok_or(Errors::CantParse))
        .map(|r| r.and_then(|(sx, sy)| {
            let x = sx.parse().map_err(|_| Errors::CantParse)?;
            let y = sy.parse().map_err(|_| Errors::CantParse)?;
            Ok(Point::new(x, y))
        }))
        .collect()
}

fn part1(obstacles: &Vec<Point>, grid_size:usize) -> Result<usize, Errors> {
    shortest_path(obstacles.as_slice(), grid_size)
}

fn part2(obstacles: &Vec<Point>, grid_size:usize) -> Point {
    let mut paths = iter::repeat(None).take(obstacles.len()).collect_vec();
    let mut l = 0; let mut r = obstacles.len()-1;
    while r-l > 1 {
        let i = (l+r)/2;
        let p = shortest_path(&obstacles[0..i], grid_size);
        if p.is_ok() { l = i; } else {r = i}
        paths[i] = Some(p);
    }
    obstacles[l]
}

fn shortest_path(obstacles: &[Point], grid_size:usize) -> Result<usize, Errors> {
    let grid_size = grid_size as i32;
    let start = Point::new(0,0);
    let end = Point::new(grid_size-1,grid_size-1);
    let o = obstacles.iter().collect::<HashSet<_>>();
    let vs = (0..grid_size).cartesian_product(0..grid_size).map(Point::from_tuple).filter(|p| !o.contains(p)).collect_vec();
    let es = vs.iter().map(|p| (p.clone(), edges(p, &o, grid_size))).collect::<HashMap<_,_>>();
    let cs = es.iter().flat_map(|x| x.1.iter().map(|e| (e.clone(),1))).collect();
    let g = Graph {vertices: vs, edges: es, costs: cs};
    g.dijkstra(start) [&end]
        .as_ref()
        .map(|x| x.0)
        .ok_or(Errors::NoPath)
}

fn edges(p:&Point, obstacles: &HashSet<&Point>, grid_size:i32) -> Vec<Edge<Point>> {
    p.ortho_neighbors().iter()
        .filter(|pt| !obstacles.contains(pt) && pt.x >=0 && pt.y >= 0 && pt.x < grid_size && pt.y < grid_size)
        .map(move |pt| Edge {from: p.clone(), to:pt.clone()})
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_small() {
        let i = String::from("5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
");
        let obstacles = parse_input(&i).unwrap();
        let p1_obstacles = obstacles.clone().into_iter().take(12).collect_vec();
        assert_eq!(part1(&p1_obstacles,7).unwrap(), 22);
        assert_eq!(part2(&obstacles,7), Point::new(6, 1));
    }
}
