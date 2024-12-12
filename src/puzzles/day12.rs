use std::{collections::HashSet, fs};

use itertools::Itertools;
use solutions::misc::{grid::Grid, point::{Point, ORTHO_DIR}};

pub fn solve() {
    let data = fs::read_to_string("../inputs/day12").expect("Should be able to read input");
    let grid: Grid<char> = Grid::char_grid(data.as_str());
    println!("P1: {p1}", p1 = part1(&grid));
    println!("P2: {p2}", p2 = part2(&grid));
}

fn part1(grid:&Grid<char>) -> usize {
    get_regions(grid).into_iter().map(|r| r.len() * perimeter(&r)).sum()
}

fn perimeter(r:&HashSet<Point>) -> usize {
    r.iter()
        .map(|&pt| ORTHO_DIR.map(|d| d+pt).iter().filter(|p| !r.contains(p)).count())
        .sum::<usize>()
}

fn part2(grid:&Grid<char>) -> usize {
    get_regions(grid).into_iter().map(|r| r.len() * sides(&r)).sum()
}

fn sides(r:&HashSet<Point>) -> usize {
    let outer = r.iter()
        .map(|pt| outer_neighbors(pt, r).tuple_combinations().filter(|(a,b)| has_corner(a, b)).count())
        .sum::<usize>();
    let inner = r.into_iter()
        .flat_map(|pt| outer_neighbors(pt, r).map(move |d| (*pt+d,d)).collect_vec())
        .into_group_map()
        .into_iter()
        .map(|(pt, ds)| ds.into_iter().tuple_combinations().filter(|(a,b)| has_inner_corner(&pt, a, b, r)).count())
        .sum::<usize>();
    outer + inner
}

fn outer_neighbors<'a>(pt: &'a Point, r:&'a HashSet<Point>) -> impl Iterator<Item = Point> + use<'a> + Clone {
    ORTHO_DIR.into_iter().filter(|d| !r.contains(&(d+pt)))
}

fn has_corner(dir1: &Point, dir2: &Point) -> bool { dir1.cross(*dir2) != 0 }

fn has_inner_corner(pt: &Point, d1: &Point, d2: &Point, r:&HashSet<Point>) -> bool {
    let d = d1 + d2;
    has_corner(d1, d2) && r.contains(&(pt-&d))
}

fn get_regions(grid:&Grid<char>) -> Vec<HashSet<Point>> {
    let mut visited = grid.dup::<bool>();
    let mut regions = Vec::new();
    grid.points_iter().for_each(|p| {
        if visited[p] {return;}
        let mut region = HashSet::new();
        let mut next = HashSet::from([p]);
        while next.len() > 0 {
            next.iter().for_each(|pt| { visited[pt] = true });
            region.extend(next.clone());
            next = next.into_iter()
                .flat_map(|p|
                    ORTHO_DIR.map(|d| d + p).into_iter()
                        .filter(|pt| grid.contains(pt) && !visited[pt] && grid[pt] == grid[p])
                        .collect_vec())
                .collect();
        }
        regions.push(region);
    });
    regions
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_1() {
        let data = String::from("AAAA
BBCD
BBCC
EEEC
");
        let grid: Grid<char> = Grid::char_grid(data.as_str());
        assert_eq!(part1(&grid), 140);
        assert_eq!(part2(&grid), 80);
    }

    #[test]
    fn sample_data_2() {
        let data = String::from("OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
");
        let grid: Grid<char> = Grid::char_grid(data.as_str());
        assert_eq!(part1(&grid), 772);
        assert_eq!(part2(&grid), 436);
    }

    #[test]
    fn sample_data_3() {
        let data = String::from("RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
");
        let grid: Grid<char> = Grid::char_grid(data.as_str());
        assert_eq!(part1(&grid), 1930);
        assert_eq!(part2(&grid), 1206);
    }

    #[test]
    fn sample_data_p2_1() {
        let data = String::from("EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
");
        let grid: Grid<char> = Grid::char_grid(data.as_str());
        assert_eq!(part2(&grid), 236);
    }

    #[test]
    fn sample_data_p2_2() {
        let data = String::from("AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
");
        let grid: Grid<char> = Grid::char_grid(data.as_str());
        assert_eq!(part2(&grid), 368);
    }

}
