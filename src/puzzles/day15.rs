use std::fs;

use itertools::Itertools;
use solutions::misc::{grid::Grid, point::{Point, EAST, NORTH, SOUTH, WEST}};

pub fn solve() {
    let input = fs::read_to_string("../inputs/day15").expect("Should be able to read input");
    let (grid, moves) = parse_input(&input);
    println!("P1: {p1}", p1 = part1(&grid, &moves));
    println!("P2: {p2}", p2 = part2(&grid, &moves));
}

fn parse_input(input:&String) -> (Grid<char>, Vec<Point>) {
    let data = input.split_once("\n\n").unwrap();
    let grid = Grid::char_grid(&data.0);
    let moves = data.1.chars().filter_map(|c| 
        match c {
            'v' => Some(SOUTH),
            '^' => Some(NORTH),
            '<' => Some(WEST),
            '>' =>  Some(EAST),
            _ =>  None
        })
        .collect_vec();
    (grid, moves)
}

fn part1(grid:&Grid<char>, moves:&Vec<Point>) -> usize {
    let mut g = Grid {width: grid.width, height: grid.height, cells: grid.cells.clone()};
    let mut r = g.find('@').expect("Should have robot");
    for m in moves {
        let mut n = r+*m;
        while Some(&'O') == g.try_get(n) { n += *m; }
        match g.try_get(n) {
            Some('.') => {
                while n != r { g[&n] = g[&n-m]; n -= *m; }
                g[&r] = '.';
                r = r+*m;
            },
            Some('#') |  None => { continue; }
            _ => { unreachable!() }
        }
    }
    score(&g)
}

fn part2(grid:&Grid<char>, moves:&Vec<Point>) -> usize {
    let mut g = Grid {
        width: grid.width*2, 
        height: grid.height, 
        cells: grid.cells.clone().into_iter().flat_map(|c| match c {
            '.' => ['.', '.'],
            '@' => ['@', '.'],
            'O' => ['[', ']'],
            '#' => ['#', '#'],
            _ => unreachable!()
        }).collect_vec()
    };
    let mut r = g.find('@').expect("Should have robot");
    for m in moves {
        if let Some(c) = cells_to_move(&r, &m, &g) {
            let new = c.iter()
                .map(|p| (p, p+m, g[p]))
                .collect_vec();
            new.iter().for_each(|(p,n,v)| g[*p] = '.');
            new.iter().for_each(|(p,n,v)| g[n] = *v);
            g[&r] = '.';
            r += *m;
            g[&r] = '@';
        }
    }
    score(&g)
}

fn cells_to_move(from: &Point, dir: &Point, grid:&Grid<char>) -> Option<Vec<Point>> {
    if dir == &EAST || dir == &WEST {
        let mut n = 0;
        while let Some(v) = grid.try_get(from.offset(n, *dir)) {
            match v {
                '@' | '[' | ']' => n += 1,
                '.' => break,
                '#' => return None,
                _ => unreachable!()
            }
        }
        return Some((1..n).map(|i| from.offset(i, *dir)).collect_vec())
    } else {
        match grid.try_get(from) {
            None | Some('#') => return None,
            Some('.') => { return Some(Vec::new()); },
            Some('@') | Some('[') | Some(']') => {
                let n = from + dir;
                match grid.try_get(n) {
                    None | Some('#') => return None,
                    Some('.') => { return Some(Vec::new()); }
                    Some('[') => {
                        let c1 = cells_to_move(&n, dir, grid)?;
                        let c2 = cells_to_move(&(n + EAST), dir, grid)?;
                        return Some(vec![vec![n, n+EAST],c1,c2].into_iter().flatten().collect_vec());
                    },
                    Some(']') => {
                        let c1 = cells_to_move(&n, dir, grid)?;
                        let c2 = cells_to_move(&(n+WEST), dir, grid)?;
                        return Some(vec![vec![n, n+WEST],c1,c2].into_iter().flatten().collect_vec());
                    },
                    _ => unreachable!()
                }
            }
            _ => unreachable!()
        }
    }
}

fn score(grid:&Grid<char>) -> usize {
    grid.points_iter()
        .filter(|p| grid[p] == '[' || grid[p] == 'O')
        .map(|p| (100*p.y + p.x) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data_small() {
        let i = String::from("########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
");
        let (grid, moves) = parse_input(&i);
        assert_eq!(part1(&grid, &moves), 2028);
    }

    #[test]
    fn sample_data_large() {
        let i = String::from("##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
");
        let (grid, moves) = parse_input(&i);
        assert_eq!(part1(&grid, &moves), 10092);
        assert_eq!(part2(&grid, &moves), 9021);
    }
}
