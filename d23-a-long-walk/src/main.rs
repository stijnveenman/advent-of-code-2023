#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::{HashMap, HashSet};

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

    println!("{}", process2(TEST_INPUT))
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

fn compress(
    grid: &CharGrid<char>,
    map: &mut HashMap<Point, (Point, usize)>,
    visited: &mut HashSet<Point>,
    from: &Point,
    goal: &Point,
) {
    if from == goal {
        return;
    }
    let mut current = *from;
    let mut count = 0;

    loop {
        let nbs = current
            .neighbours()
            .into_iter()
            .filter(|n| grid.is_within(n))
            .filter(|n| grid.get(n).map(|c| c != &'#').unwrap_or(true))
            .filter(|n| !visited.contains(n))
            .collect_vec();
        visited.insert(current);

        if nbs.len() == 1 {
            current = *nbs.first().unwrap();
            count += 1;
            if current == *goal {
                map.insert(*from, (current, count));
                map.insert(current, (*from, count));
                return;
            }
        } else if nbs.is_empty() {
            if *from != current {
                map.insert(*from, (current, count));
                map.insert(current, (*from, count));
            }
            return;
        } else {
            map.insert(*from, (current, count));
            map.insert(current, (*from, count));
            if current == *goal {
                return;
            }

            for n in nbs.iter() {
                compress(grid, map, visited, n, goal);
            }
        }
    }
}

fn longest(
    map: &HashMap<Point, (Point, usize)>,
    start: &Point,
    goal: &Point,
    visited: HashSet<Point>,
    score: usize,
) -> usize {
    if start == goal {
        println!("done");
        return score;
    }
    start
        .neighbours()
        .into_iter()
        .filter(|n| !visited.contains(n))
        .filter_map(|n| map.get(&n))
        .dbg()
        .map(|(n, np)| {
            let mut nv = visited.clone();
            nv.insert(*n);

            longest(map, n, goal, nv, score + np)
        })
        .max()
        .unwrap_or(0)
}

fn process2(s: &str) -> usize {
    let grid = CharGrid::new(s, |c| match c {
        '.' => None,
        _ => Some(c),
    });
    let start = Point::new(1, 0);
    let goal = grid.upper() + Point::new(-1, 0);
    println!("{:?}", goal);

    let mut map = HashMap::new();
    let mut visited = HashSet::new();
    compress(&grid, &mut map, &mut visited, &start, &goal);
    map.iter().for_each(|n| println!("{:?}", n));

    let start = Point::new(0, 0);
    longest(&map, &start, &goal, HashSet::new(), 0)
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
