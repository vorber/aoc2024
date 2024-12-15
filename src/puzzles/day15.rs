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
    _solve(&mut g, moves)
}

fn part2(grid:&Grid<char>, moves:&Vec<Point>) -> usize {
    let mut g = extend_grid_p2(grid);
    _solve(&mut g, moves)
}

fn _solve(g: &mut Grid<char>, moves:&Vec<Point>) -> usize {
    let mut r = g.find('@').expect("Should have robot");
    for m in moves {
        match try_move(r, *m, &g) {
            Ok(to_move) => {
                to_move.iter().for_each(|(_, from, _)| g[from] = '.');
                to_move.iter().for_each(|(to, _, v)| g[to] = *v);
                r += *m;
            }
            Err(_) => continue
        }
    }
    score(&g)
}

fn extend_grid_p2(grid:&Grid<char>) -> Grid<char> {
    Grid {
        width: grid.width*2, 
        height: grid.height, 
        cells: grid.cells.clone().into_iter().flat_map(|c| match c {
            '.' => ['.', '.'],
            '@' => ['@', '.'],
            'O' => ['[', ']'],
            '#' => ['#', '#'],
            _ => unreachable!()
        }).collect_vec()
    }
}

enum CantMoveReason {
    OutOfBounds,
    Wall
}

fn try_move(from: Point, dir: Point, grid:&Grid<char>) -> Result<Vec<(Point, Point, char)>,CantMoveReason> {
    let to = from+dir;
    let mut result = vec![(to, from, grid[from])];
    let v = grid.try_get(to);
    match v {
        None => Err(CantMoveReason::OutOfBounds),
        Some('.') => Ok(result),
        Some('#') => Err(CantMoveReason::Wall),
        Some(c@'[') | Some(c@']') | Some(c@'O') => {
            match dir {
                EAST | WEST => { result.extend(try_move(to, dir, grid)?); Ok(result) }
                _ => {
                    let other = match c {
                        '[' => try_move(to+EAST, dir, grid)?,
                        ']' => try_move(to+WEST, dir, grid)?,
                        _ => Vec::new()
                    };
                    result.extend(other);
                    result.extend(try_move(to, dir, grid)?);
                    Ok(result)
                } 
            }
        },
        _ => unreachable!()
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
