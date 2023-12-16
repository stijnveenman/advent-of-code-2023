#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn check_horizontal(lines: &[&str], i: usize) -> bool {
    let mut j = 0;

    loop {
        let Some(bottom) = lines.get(i - 1 - j) else {
            return true;
        };
        let Some(top) = lines.get(i + j) else {
            return true;
        };

        if bottom != top {
            return false;
        }

        j += 1;
    }
}

fn reflect_horizontal(s: &str) -> Option<usize> {
    let lines = s.lines().collect::<Vec<_>>();
    for i in 1..lines.len() {
        let prev = lines[i - 1];
        let cur = lines[i];

        if cur == prev && check_horizontal(&lines, i) {
            return Some(i);
        }
    }
    None
}

fn reflection(s: &str) -> usize {
    reflect_horizontal(s).unwrap_or(0)
}

fn process(s: &str) -> usize {
    s.split("\n\n").map(reflection).dbg().sum()
}

fn process2(s: &str) -> usize {
    todo!()
}

static TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 405)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
