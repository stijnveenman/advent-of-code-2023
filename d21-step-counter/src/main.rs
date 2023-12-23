#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::HashSet;

use aoc_toolbox::char_grid::CharGrid;
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
static TEST_PART2_RESULT: usize = 420;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input, 64))
}

fn step_count(grid: &mut CharGrid<char>, step_count: usize) -> usize {
    let start = *grid
        .iter()
        .find(|(_, c)| **c == 'S')
        .map(|(p, _)| p)
        .unwrap();
    grid.remove(&start);

    let mut open = vec![(start, step_count)];
    let mut visited = HashSet::new();
    let mut closed = HashSet::new();

    while let Some((current, steps)) = open.pop() {
        if visited.contains(&(current, steps)) {
            continue;
        }
        visited.insert((current, steps));
        if steps == 0 {
            closed.insert(current);
            continue;
        }

        for n in current
            .neighbours()
            .into_iter()
            .filter(|p| grid.is_within(p))
            .filter(|p| grid.get(p).is_none())
        {
            open.push((n, steps - 1));
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

fn process2(s: &str) -> usize {
    let grid = CharGrid::new(s, |c| match c {
        '#' => Some('#'),
        'S' => Some('S'),
        _ => None,
    });
    grid.draw_char();

    todo!()
}

#[test]
fn test_part1() {
    let result = process(TEST_INPUT, 6);

    assert_eq!(dbg!(result), TEST_PART1_RESULT)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), TEST_PART2_RESULT)
}
