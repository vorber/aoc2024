use std::{collections::HashSet, fs, hash::Hash};

use solutions::misc::{grid::Grid, point::Point};

pub fn solve() {
    let data = fs::read_to_string("../inputs/day6").expect("Should be able to read input");
    let grid: Grid<char> = Grid::char_grid(data.as_str());
    println!("P1: {p1}", p1 = part1(&grid));
    println!("P2: {p2}", p2 = part2(&grid));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Guard {
    pos: Point,
    dir: Point,
}
impl Guard {
    fn turn_right(self) -> Self {
        Guard {pos: self.pos, dir: self.dir.rotate_clockwise()}
    }
    
    fn next(self) ->  Self {
        Guard {pos: self.pos + self.dir, dir: self.dir}
    }
}
impl Hash for Guard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.dir.hash(state);
    }
}

fn part1(grid:&Grid<char>) -> usize {
    let initial = Guard { pos:grid.find('^').unwrap(), dir:Point::new(0, -1) };
    exit_path(initial, grid,  None)
        .unwrap()
        .iter()
        .collect::<HashSet<_>>()
        .len()
}

fn exit_path(initial:Guard, grid:&Grid<char>, extra_obstacle:Option<Point>) -> Option<Vec<Point>> {
    let mut visited = HashSet::new();
    let mut g = initial;
    let mut path = Vec::new();
    while grid.contains(&g.pos) {
        path.push(g.pos);
        let next = g.next();

        if Some(&'#') == grid.try_get(next.pos) || extra_obstacle.is_some_and(|p| p == next.pos) {
            if !visited.insert(g) { return None; }
            g = g.turn_right();
        } else {
            g = next;
        }
    }
    Some(path)
}

fn part2(grid:&Grid<char>) -> usize {
    let initial = Guard { pos:grid.find('^').unwrap(), dir:Point::new(0, -1) };
    let path = exit_path(initial, grid, None).unwrap();
    let mut cs = HashSet::new();
    path.iter().for_each(|&p| {
        if exit_path(initial, grid, Some(p)).is_none() { cs.insert(p); }
    });
    cs.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let data = String::from("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
");

        let grid: Grid<char> = Grid::char_grid(data.as_str());
        assert_eq!(part1(&grid), 41);
        assert_eq!(part2(&grid), 6);
    }
}
