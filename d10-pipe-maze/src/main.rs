#![allow(dead_code)]
#![allow(unused_variables)]
mod util;

#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug)]
enum PipeTile {
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    Start,
    Ground,
}

impl From<char> for PipeTile {
    fn from(value: char) -> Self {
        match value {
            '|' => PipeTile::Vertical,
            '-' => PipeTile::Horizontal,
            'L' => PipeTile::UpRight,
            'J' => PipeTile::UpLeft,
            '7' => PipeTile::DownLeft,
            'F' => PipeTile::DownRight,
            '.' => PipeTile::Ground,
            'S' => PipeTile::Start,
            c => panic!("invalid char {}", c),
        }
    }
}

fn parse(s: &str) -> Vec<Vec<PipeTile>> {
    s.lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .dbg()
        .collect()
}

fn process(s: &str) -> u32 {
    let input = parse(s);
    println!("{:?}", input);

    todo!()
}

fn process2(s: &str) -> u32 {
    let input = parse(s);
    println!("{:?}", input);

    todo!()
}

static TEST_INPUT: &str = ".....
.F-7.
.|.|.
.L-J.
.....";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 4)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
