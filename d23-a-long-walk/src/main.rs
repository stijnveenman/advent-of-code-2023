#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::HashMap;

use aoc_toolbox::{char_grid::CharGrid, point::Point};
#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
static TEST_PART1_RESULT: usize = 94;
static TEST_PART2_RESULT: usize = 420;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(TEST_INPUT))
}

fn next(v: &mut Vec<(Point, usize)>) -> Option<(Point, usize)> {
    if v.is_empty() {
        return None;
    }
    let highest = v.iter().map(|v| v.1).min().unwrap();
    let Some(index) = v.iter().enumerate().find(|v| v.1 .1 == highest) else {
        return None;
    };

    Some(v.remove(index.0))
}

fn process(s: &str) -> usize {
    let grid = CharGrid::new(s, |c| match c {
        '.' => None,
        _ => Some(c),
    });
    let start = Point::new(1, 0);
    let goal = grid.upper() + Point::new(-1, 0);

    let mut open = vec![(start, 0)];
    let mut closed_set = HashMap::new();

    while let Some((current, value)) = next(&mut open) {
        closed_set.insert(current, value);

        for n in current
            .neighbours()
            .into_iter()
            .filter(|n| grid.is_within(n))
        {
            if let Some(c) = grid.get(&n) {
                if c == &'#' {
                    continue;
                }
            }

            if let Some(existing_value) = closed_set.get(&n) {
                if *existing_value <= value + 1 {
                    continue;
                }
            }

            open.push((n, value + 1))
        }

        if current == goal {
            return value;
        }
    }

    panic!()
}

fn process2(s: &str) -> usize {
    let grid = CharGrid::new(s, |c| match c {
        '.' => None,
        _ => Some(c),
    });
    grid.draw_char();

    todo!()
}

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), TEST_PART1_RESULT)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), TEST_PART2_RESULT)
}
