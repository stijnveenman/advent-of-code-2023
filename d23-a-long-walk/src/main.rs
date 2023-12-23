#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::HashMap;

use aoc_toolbox::{char_grid::CharGrid, point::Point};
use itertools::Itertools;
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
static TEST_PART2_RESULT: usize = 154;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn next<T>(v: &mut Vec<(T, usize)>) -> Option<(T, usize)> {
    if v.is_empty() {
        return None;
    }
    let highest = v.iter().map(|v| v.1).max().unwrap();
    let Some(index) = v.iter().enumerate().find(|v| v.1 .1 == highest) else {
        return None;
    };

    Some(v.remove(index.0))
}

fn on_path(closed_set: &HashMap<Point, (usize, Vec<Point>)>, from: &Point, p: &Point) -> bool {
    if let Some(cur) = closed_set.get(from) {
        return cur.1.contains(p);
    }
    false
}

fn process(s: &str) -> usize {
    let grid = CharGrid::new(s, |c| match c {
        '.' => None,
        _ => Some(c),
    });
    let start = Point::new(1, 0);
    let goal = grid.upper() + Point::new(-1, 0);

    let mut open = vec![(vec![start], 0)];
    let mut max = 0;

    while let Some((list, value)) = next(&mut open) {
        let current = *list.last().unwrap();
        for n in current
            .neighbours()
            .into_iter()
            .filter(|n| grid.is_within(n))
            .filter(|n| match grid.get(&current) {
                Some('#') => false,
                Some('^') => *n == current + Point::UP,
                Some('v') => *n == current + Point::DOWN,
                Some('>') => *n == current + Point::RIGHT,
                Some('<') => *n == current + Point::LEFT,
                Some(c) => panic!("char missing {}", c),
                None => true,
            })
        {
            if list.contains(&n) {
                continue;
            }

            let mut nv = list.to_vec();
            nv.push(n);
            open.push((nv, value + 1))
        }

        if current == goal && value > max {
            max = value;
            println!("{}", value);
        }
    }

    max
}

fn walk(grid: &CharGrid<char>, list: &mut Vec<Point>, goal: Point) -> Vec<usize> {
    let mut current = *list.last().unwrap();

    loop {
        let nbs = current
            .neighbours()
            .into_iter()
            .filter(|n| grid.is_within(n))
            .filter(|n| !list.contains(n))
            .filter(|n| grid.get(n).map(|c| *c != '#').unwrap_or(true))
            .collect_vec();

        if nbs.is_empty() && current != goal {
            grid.draw(|p, c| {
                if *p == current {
                    '?'
                } else if list.contains(p) {
                    'O'
                } else {
                    *c.unwrap_or(&'.')
                }
            });
            println!("deadend {:?}", current);
        }

        if current == goal {
            println!("{}", list.len());
        }

        if nbs.len() == 1 {
            let n = nbs.first().unwrap();
            list.push(*n);
            current = *n;
        } else {
            return nbs
                .into_iter()
                .flat_map(|n| {
                    let mut nv = list.to_vec();
                    nv.push(n);
                    walk(grid, &mut nv, goal)
                })
                .collect_vec();
        }
    }
}

fn process2(s: &str) -> usize {
    let grid = CharGrid::new(s, |c| match c {
        '.' => None,
        _ => Some(c),
    });
    let start = Point::new(1, 0);
    let goal = grid.upper() + Point::new(-1, 0);

    walk(&grid, &mut vec![start], goal)
        .into_iter()
        .max()
        .unwrap()
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
