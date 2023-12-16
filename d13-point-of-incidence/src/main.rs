#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn count_diff(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}

fn check_horizontal(lines: &[&str], i: usize) -> bool {
    let mut j = 0;
    let mut diff = 0;

    loop {
        let Some(idx) = i.checked_sub(1).and_then(|i| i.checked_sub(j)) else {
            break;
        };
        let Some(bottom) = lines.get(idx) else {
            break;
        };
        let Some(top) = lines.get(i + j) else {
            break;
        };

        diff += count_diff(top, bottom);

        j += 1;
    }

    diff == 1
}

fn reflect(lines: &[&str]) -> Option<usize> {
    for i in 1..lines.len() {
        let prev = lines[i - 1];
        let cur = lines[i];

        if count_diff(prev, cur) <= 1 && check_horizontal(lines, i) {
            return Some(i);
        }
    }
    None
}

fn reflect_horizontal(s: &str) -> Option<usize> {
    let lines = s.lines().collect::<Vec<_>>();
    reflect(&lines)
}

fn flip(s: &str) -> Vec<String> {
    let mut lines = s.lines().map(|c| c.chars()).collect::<Vec<_>>();
    let mut r = vec![];

    while let Some(s) = lines
        .iter_mut()
        .map(|c| c.next())
        .collect::<Option<String>>()
    {
        r.push(s);
    }

    r
}

fn reflect_vertical(s: &str) -> Option<usize> {
    let lines = flip(s);
    reflect(&lines.iter().map(|s| s.as_str()).collect::<Vec<_>>())
}

fn reflection(s: &str) -> usize {
    reflect_horizontal(s)
        .map(|r| r * 100)
        .unwrap_or_else(|| reflect_vertical(s).unwrap())
}

fn process(s: &str) -> usize {
    s.split("\n\n").map(reflection).sum()
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
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 400)
}
