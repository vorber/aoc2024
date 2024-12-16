use std::{cmp::Ordering, collections::{HashMap, HashSet}, fs};

use itertools::Itertools;
use solutions::misc::{graph::{Edge, Graph}, grid::Grid, point::{Point, EAST, ORTHO_DIR}};

pub fn solve() {
    let input = fs::read_to_string("../inputs/day16").expect("Should be able to read input");
    let (p1, p2) = _solve(&parse_input(&input));
    println!("P1: {p1}");
    println!("P2: {p2}");
}

fn parse_input(input:&String) -> Grid<char> {
    Grid::char_grid(input)
}

fn _solve(grid:&Grid<char>) -> (usize, usize) {
    let g = Graph::from_grid(grid);
    let start = Vertex::new(grid.find('S').unwrap(), EAST);
    let e = grid.find('E').unwrap();
    let ends = ORTHO_DIR.map(|d| Vertex::new(e, d)).to_vec();
    let visits = g.dijkstra(start);
    let shortest = ends.iter()
        .filter_map(|v| visits[v].as_ref().map(|p| p.0).or(Some(usize::MAX)))
        .min()
        .unwrap();
    let seats = ends.iter()
        .filter_map(|v| visits[v].as_ref().filter(|d| d.0 == shortest).and(Some(v)))
        .flat_map(|e| mark_path(start, *e, &visits))
        .count();
    (shortest, seats)
}

fn mark_path(start:Vertex, e:Vertex, distances: &HashMap<&Vertex, Option<(usize, Vec<Vertex>)>>) -> HashSet<Point> {
    let mut cur = vec![e];
    let mut r = HashSet::new();
    while !r.contains(&start) {
        let next = cur.iter()
            .flat_map(|v| distances[v].as_ref().unwrap().1.clone())
            .collect_vec();
        r.extend(cur);
        cur = next;
    }
    r.iter().map(|v| v.pos).collect()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Vertex {
    pos: Point,
    dir: Point
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.pos.cmp(&self.pos).then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Vertex {
    pub fn new(p: Point, d: Point) -> Self {
        Vertex {pos: p, dir: d}
    }
}

trait FromGrid {
    fn from_grid(grid:&Grid<char>) -> Self;
}

impl FromGrid for Graph<Vertex, usize> {
    fn from_grid(grid:&Grid<char>) -> Self {
        let vertices = grid.points_iter()
            .filter(|&p| grid[p] != '#')
            .flat_map(|p| ORTHO_DIR.map(|d| Vertex::new(p,d)))
            .collect_vec();
        let mut edges = HashMap::new();
        let mut costs = HashMap::new();
        for v in vertices.clone() {
            let es = get_edges(&v);
            let entry = edges.entry(v).or_insert(Vec::new());
            for (e,c) in es {
                costs.insert(e.clone(), c);
                entry.push(e);
            }
        }
        Graph { vertices, edges, costs }
    }
}

fn get_edges(v:&Vertex) -> [(Edge<Vertex>, usize);3] {[
    (Edge::new(v.clone(), Vertex::new(v.pos, v.dir.rotate_cw())), 1000),
    (Edge::new(v.clone(), Vertex::new(v.pos, v.dir.rotate_ccw())), 1000),
    (Edge::new(v.clone(), Vertex::new(v.pos + v.dir, v.dir)), 1)
]}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_1() {
        let i = String::from("###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
");
        let grid = parse_input(&i);
        let (p1,p2) = _solve(&grid);
        assert_eq!(p1, 7036);
        assert_eq!(p2, 45);
    }

    #[test]
    fn sample_data_large() {
        let i = String::from("#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
");
        let grid = parse_input(&i);
        let (p1,p2) = _solve(&grid);
        assert_eq!(p1, 11048);
        assert_eq!(p2, 64);
    }
}
