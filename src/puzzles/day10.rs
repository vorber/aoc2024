use std::{collections::HashSet, fs, usize};

use itertools::Itertools;
use solutions::misc::{grid::Grid, point::{Point, ORTHO_DIR}};

pub fn solve() {
    let data = fs::read_to_string("../inputs/day10").expect("Should be able to read input");
    let heights = parse_input(&data);
    println!("P1: {p1}", p1 = part1(&heights));
    println!("P2: {p2}", p2 = part2(&heights));
}

fn parse_input(data:&String) -> Grid<u8> {
    let grid = Grid::char_grid(data.as_str());
    Grid {height: grid.height, width: grid.width, cells: grid.cells.iter().map(|c| *c as u8 - b'0').collect_vec()}
}

fn part1(heights:&Grid<u8>) -> usize {
    let mut scores: Grid<Option<HashSet<Point>>> = heights.dup();
    heads(heights)
        .filter_map(|p| score_p1(heights, &mut scores, p))
        .map(|s| s.len())
        .sum()
}

fn part2(heights:&Grid<u8>) -> usize {
    let mut scores: Grid<Option<usize>> = heights.dup();
    heads(heights)
        .filter_map(|p| score_p2(heights, &mut scores, p))
        .sum()
}

fn score_p1(heights:&Grid<u8>, scores: &mut Grid<Option<HashSet<Point>>>, start:Point) -> Option<HashSet<Point>> {
    let s = scores[start].clone()
        .or_else(|| {
            match heights[start] {
                9 => Some(HashSet::from([start.clone()])),
                _ => Some(valid_moves(heights, start)
                        .filter_map(|p| score_p1(heights, scores, p))
                        .flatten()
                        .collect::<HashSet<_>>())
            }
        });
    scores[&start] = s.clone();
    s
}

fn score_p2(heights:&Grid<u8>, scores: &mut Grid<Option<usize>>, start:Point) -> Option<usize> {
    let score = scores[start]
        .or_else(|| {
            match heights[start] {
                9 => Some(1),
                _ => valid_moves(heights, start).filter_map(|p| score_p2(heights, scores, p)).sum1()
            }
        });
    scores[&start] = score;
    score
}

fn valid_moves(heights:&Grid<u8>, start:Point) -> impl Iterator<Item = Point> + use<'_> {
    ORTHO_DIR.into_iter()
        .map(move |d| start + d)
        .filter(|p| heights.contains(p))
        .filter(move |p| heights[p] == heights[start] + 1)
}

fn heads(heights:&Grid<u8>) -> impl Iterator<Item = Point> + use<'_> {
    heights.points_iter().filter(|p| heights[p] == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let data = String::from("89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
");

        let heights = parse_input(&data);
        assert_eq!(part1(&heights), 36);
        assert_eq!(part2(&heights), 81);
    }
}
