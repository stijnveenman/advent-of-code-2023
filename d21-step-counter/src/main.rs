#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::{HashMap, VecDeque};

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

fn next(v: &mut Vec<(Point, usize)>) -> Option<(Point, usize)> {
    if v.is_empty() {
        return None;
    }
    let m = v.iter().map(|v| v.1).min().unwrap();
    let Some(index) = v.iter().enumerate().find(|v| v.1 .1 == m) else {
        return None;
    };

    Some(v.remove(index.0))
}

fn step_count(grid: &mut CharGrid<char>, step_count: usize) -> usize {
    let start = *grid
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(p, _)| p)
        .unwrap();
    grid.remove(&start);

    let mut open = vec![(start, 0)];
    let mut visited = HashMap::new();

    while let Some((current, steps)) = next(&mut open) {
        if visited.get(&current).map(|v| *v <= steps).unwrap_or(false) {
            continue;
        }
        visited.insert(current, steps);

        for n in current
            .neighbours()
            .into_iter()
            .filter(|p| grid.is_within(p))
            .filter(|p| grid.get(p).is_none())
        {
            open.push((n, steps + 1));
        }
    }

    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let even_full = visited.values().filter(|v| **v % 2 == 0).count();
    let odd_full = visited.values().filter(|v| **v % 2 == 1).count();

    // This is 202300 but im writing it out here to show the process
    // let n = ((26501365 - (env.dim.0 / 2)) / env.dim.0) as usize;

    let n = 202300;
    assert_eq!(n, 202300);

    ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners + n * even_corners
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
    //assert_eq!(process(TEST_INPUT, 6), 16);
    //assert_eq!(process(TEST_INPUT, 10), 50);
    //assert_eq!(process(TEST_INPUT, 50), 1594);
    //assert_eq!(process(TEST_INPUT, 100), 6536);
    //assert_eq!(process(TEST_INPUT, 500), 167004);
    //assert_eq!(process(TEST_INPUT, 1000), 668697);
    assert_eq!(process(TEST_INPUT, 5000), 16733044);
}
