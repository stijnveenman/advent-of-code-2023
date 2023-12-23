#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::{HashSet, VecDeque};

use aoc_toolbox::{char_grid::CharGrid, point::Point};
#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
static TEST_PART1_RESULT: usize = 16;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input, 26501365))
}

fn step_count(grid: &mut CharGrid<char>, step_count: usize) -> usize {
    let start = *grid
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(p, _)| p)
        .unwrap();
    grid.remove(&start);

    let mut open = VecDeque::new();
    open.push_back((start, step_count));
    let mut visited = HashSet::new();
    let mut closed = HashSet::new();

    while let Some((current, steps)) = open.pop_front() {
        if visited.contains(&(current, steps)) {
            continue;
        }
        visited.insert((current, steps));
        if steps == 0 {
            closed.insert(current);
            continue;
        }

        for n in current.neighbours().into_iter().filter(|p| {
            let bounds = grid.upper();
            let mut x = p.x % (bounds.x + 1);
            let mut y = p.y % (bounds.y + 1);
            while x < 0 {
                x += bounds.x + 1;
            }
            while y < 0 {
                y += bounds.y + 1;
            }
            let np = Point::new(x, y);
            grid.get(&np).is_none()
        }) {
            open.push_back((n, steps - 1));
        }
    }

    closed.len()
}

fn process(s: &str, count: usize) -> usize {
    let mut grid = CharGrid::new(s, |c| match c {
        '#' => Some('#'),
        'S' => Some('S'),
        _ => None,
    });

    step_count(&mut grid, count)
}

#[test]
fn test_part1() {
    let result = process(TEST_INPUT, 6);

    assert_eq!(dbg!(result), TEST_PART1_RESULT)
}

#[test]
fn test_part2() {
    assert_eq!(process(TEST_INPUT, 6), 16);
    assert_eq!(process(TEST_INPUT, 10), 50);
    assert_eq!(process(TEST_INPUT, 50), 1594);
    assert_eq!(process(TEST_INPUT, 100), 6536);
    //    assert_eq!(process(TEST_INPUT, 500), 167004);
    //    assert_eq!(process(TEST_INPUT, 1000), 668697);
    //    assert_eq!(process(TEST_INPUT, 5000), 16733044);
}
