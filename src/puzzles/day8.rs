use core::range::Range;
use std::{cmp::max, collections::HashSet, fs};

use itertools::Itertools;
use solutions::misc::{grid::Grid, point::Point};

pub fn solve() {
    let data = fs::read_to_string("../inputs/day8").expect("Should be able to read input");
    let grid: Grid<char> = Grid::char_grid(data.as_str());
    println!("P1: {p1}", p1 = part1(&grid));
    println!("P2: {p2}", p2 = part2(&grid));
}

fn part1(grid:&Grid<char>) -> usize { count_antinodes(grid, |_p| 1..2) }

fn part2(grid:&Grid<char>) -> usize { count_antinodes(grid, |d|  0..max(grid.width/d.x.abs() as usize, grid.height/d.y.abs() as usize)) }

fn count_antinodes(grid:&Grid<char>, r:impl Fn(&Point) -> Range<usize> ) -> usize {
    let antennas = grid.points_iter()
        .filter(|&p| grid[p] != '.')
        .map(|p| (grid[p], p))
        .into_group_map();
    let antinodes = antennas.iter()
        .flat_map(|(_c, pts)| {
            pts.iter().combinations(2)
                .flat_map(|pair| {
                    let (a,b) = (pair[0], pair[1]);
                    let d = *b-*a;
                    let range = r(&d);
                    range.flat_map(|i| [*b+i*d, *a-i*d]).collect_vec()
                })
                .collect_vec()
        })
        .filter(|p| grid.contains(p))
        .collect::<HashSet<_>>();
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let data = String::from("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
");

        let grid: Grid<char> = Grid::char_grid(data.as_str());
        assert_eq!(part1(&grid), 14);
        assert_eq!(part2(&grid), 34);
    }
}
