use std::fs;

use solutions::misc::{grid::Grid, point::Point};

pub fn solve() {
    let grid = Grid::char_grid( fs::read_to_string("../inputs/day4").expect("Should be able to read input").as_str());
    println!("P1: {p1}", p1 = d4p1(&grid));
    println!("P2: {p2}", p2 = d4p2(&grid));
}

fn d4p1(grid:&Grid<char>) -> usize {
    grid.points_iter()
        .map(|p| ["XMAS", "SAMX"].map(|s| count(s, &grid, p)).iter().sum::<usize>())
        .sum::<usize>()
}

fn d4p2(grid:&Grid<char>) -> usize {
    grid.points_iter().filter(|&p| is_xmas(p, &grid)).count()
}

fn count(s: &str, grid:&Grid<char>, pos:Point)-> usize
{
    [(-1,1), (0,1),(1,1),(1,0)]
        .map(|(x,y)| Point::new(x, y))
        .map(|dir| check(s, grid, pos, dir).then_some(1).unwrap_or(0)).iter()
        .sum()
}

fn check(s:&str, grid:&Grid<char>, p: Point, d: Point) -> bool {
    s.chars().enumerate()
        .map(|(i,c)| (p.offset(i, d), c))
        .filter(|(p,c)| Some(c) == grid.try_get(p))
        .count()
        .eq(&s.len())
}

fn is_xmas(p:Point, grid:&Grid<char>) -> bool {
    let c = ["MAS","SAM"];
    let fst = c.iter().any(|s| check(s, grid, p, Point::new(1,1)));
    let snd = c.iter().any(|s| check(s, grid, p+Point::new(0,2), Point::new(1,-1)));
    fst & snd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let grid = Grid::char_grid("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        assert_eq!(d4p1(&grid), 18);
        assert_eq!(d4p2(&grid), 9);
    }
}
