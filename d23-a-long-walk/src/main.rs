#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::{HashMap, HashSet};

use aoc_toolbox::{char_grid::CharGrid, point::Point};
use chrono::Local;
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

    println!("{}", process2(input))
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

fn insert(map: &mut HashMap<Point, Vec<(Point, usize)>>, from: &Point, to: &Point, value: usize) {
    let mut v = map.get(from).unwrap_or(&vec![]).to_vec();
    v.push((*to, value));
    map.insert(*from, v);

    let mut v = map.get(to).unwrap_or(&vec![]).to_vec();
    v.push((*from, value));
    map.insert(*to, v);
}

fn compress(
    grid: &CharGrid<char>,
    from: Point,
    goal: Point,
) -> HashMap<Point, Vec<(Point, usize)>> {
    let mut map = HashMap::new();
    let mut visited = HashSet::new();
    let mut open = vec![(from, 0, from + Point::UP, from)];

    while let Some((current, value, prev, start)) = open.pop() {
        let nbs = current
            .neighbours()
            .into_iter()
            .filter(|n| grid.is_within(n))
            .filter(|n| grid.get(n).map(|c| c != &'#').unwrap_or(true))
            .filter(|n| *n != prev)
            .collect_vec();

        if nbs.is_empty() {
            insert(&mut map, &start, &current, value);
        } else if nbs.len() == 1 {
            let n = nbs.first().unwrap();
            open.push((*n, value + 1, current, start));
        } else {
            insert(&mut map, &start, &current, value);
            if !visited.contains(&current) {
                for n in nbs {
                    open.push((n, 1, current, current));
                }
            }
            visited.insert(current);
        }
    }

    map
}

fn longest(
    map: &HashMap<Point, Vec<(Point, usize)>>,
    start: Point,
    goal: Point,
    visited: HashSet<Point>,
    score: usize,
) -> usize {
    if start == goal {
        return score;
    }

    let next = map.get(&start).unwrap();
    if let Some(n) = next.iter().find(|v| v.0 == goal) {
        let mut v = visited.clone();
        v.insert(n.0);

        return longest(map, n.0, goal, v, score + n.1);
    }

    let mut max = 0;

    next.iter()
        .filter(|n| !visited.contains(&n.0))
        .map(|n| {
            let mut v = visited.clone();
            v.insert(n.0);

            longest(map, n.0, goal, v, score + n.1)
        })
        .for_each(|v| {
            if v > max {
                let now = Local::now();
                let res = now.format("%Y-%m-%d %H:%M:%S");
                max = v;
                println!("{}: new max {}", res, max);
            }
        });

    max
}

fn process2(s: &str) -> usize {
    let grid = CharGrid::new(s, |c| match c {
        '.' => None,
        _ => Some(c),
    });
    let start = Point::new(1, 0);
    let goal = grid.upper() + Point::new(-1, 0);

    let map = compress(&grid, start, goal);
    println!("done compressing: {} steps", map.len());

    longest(&map, start, goal, HashSet::new(), 0)
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
