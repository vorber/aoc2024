use std::{collections::HashMap, fs};

use itertools::Itertools;
use solutions::misc::{graph::{Edge, Graph}, grid::Grid, point::Point};

#[derive(Clone, Debug, PartialEq)]
enum Errors {
    NoFile,
    InvalidGrid,
    NoPath
}

#[derive(Debug)]
struct Cheat {
    value: usize
}

pub fn solve() {
    let (p1, p2) = fs::read_to_string("../inputs/day20").map_err(|_| Errors::NoFile)
        .map(|input| Grid::char_grid(&input))
        .and_then(|grid| Ok((_solve(&grid, 2, 100)?.len(), _solve(&grid, 20, 100)?.len())))
        .unwrap();
    println!("P1: {p1}\nP2:{p2}");
}

fn _solve(grid:&Grid<char>, max_cheat_distance:usize, threshold:usize) -> Result<Vec<usize>, Errors> {
    let path = shortest_path(grid)?;
    let path_values = path.iter().enumerate().map(|(i,p)| (p,i)).collect::<HashMap<_,_>>();
    let cheats = path.iter()
        .flat_map(|pt| {
            pt.manhattan_neighbors(max_cheat_distance).into_iter()
                .filter_map(|n| {
                    let d=pt.manhattan_distance(n) as usize;
                    let nv = path_values.get(&n).unwrap_or(&0);
                    let pv = path_values[pt];
                    (*nv>(pv+d)).then(|| *nv-pv-d )
                })
        })
        .filter(|&c| c >= threshold)
        .collect_vec();
    Ok(cheats)
}

fn shortest_path(grid:&Grid<char>) -> Result<Vec<Point>, Errors> {
    let start = grid.find('S').ok_or(Errors::InvalidGrid)?;
    let end = grid.find('E').ok_or(Errors::InvalidGrid)?;
    let vs = grid.points_iter().filter(|p| grid[p] != '#').collect_vec();
    let es = vs.iter().map(|p| (p.clone(), edges(p, grid))).collect::<HashMap<_,_>>();
    let cs = es.iter().flat_map(|x| x.1.iter().map(|e| (e.clone(),1))).collect();
    let g = Graph {vertices: vs, edges: es, costs: cs};
    g.find_path(start, end).map_err(|_| Errors::NoPath)
}

fn edges(p:&Point, grid:&Grid<char>) -> Vec<Edge<Point>> {
    p.ortho_neighbors().iter()
        .filter(|pt| grid.try_get(pt).is_some_and(|c| *c != '#'))
        .map(move |pt| Edge {from: p.clone(), to:pt.clone()})
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_small() {
        let i = String::from("###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
");
        let grid = parse_input(&i).unwrap();
        let p1 = _solve(&grid, 2, 0).unwrap().into_iter().counts();
        assert_eq!(p1[&2], 14);
        assert_eq!(p1[&4], 14);
        assert_eq!(p1[&6], 2);
        assert_eq!(p1[&8], 4);
        assert_eq!(p1[&10], 2);
        assert_eq!(p1[&12], 3);
        assert_eq!(p1[&20], 1);
        assert_eq!(p1[&36], 1);
        assert_eq!(p1[&38], 1);
        assert_eq!(p1[&40], 1);
        assert_eq!(p1[&64], 1);

        let p2 = _solve(&grid, 20, 50).unwrap().into_iter().counts();
        assert_eq!(p2[&50], 32);
        assert_eq!(p2[&52], 31);
        assert_eq!(p2[&54], 29);
        assert_eq!(p2[&56], 39);
        assert_eq!(p2[&58], 25);
        assert_eq!(p2[&60], 23);
        assert_eq!(p2[&62], 20);
        assert_eq!(p2[&64], 19);
        assert_eq!(p2[&66], 12);
        assert_eq!(p2[&68], 14);
        assert_eq!(p2[&70], 12);
        assert_eq!(p2[&72], 22);
        assert_eq!(p2[&74], 4);
        assert_eq!(p2[&76], 3);
    }
}
