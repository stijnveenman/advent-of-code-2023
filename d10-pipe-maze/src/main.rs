#![allow(dead_code)]
#![allow(unused_variables)]
mod util;

#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

impl PipeTile {
    fn next(&self, pos: (u32, u32)) -> Option<((u32, u32), (u32, u32))> {
        match self {
            PipeTile::Vertical => Some(((pos.0, pos.1 + 1), (pos.0, pos.1 - 1))),
            PipeTile::Horizontal => Some(((pos.0 + 1, pos.1), (pos.0 - 1, pos.1))),
            PipeTile::UpRight => Some(((pos.0, pos.1 - 1), (pos.0 + 1, pos.1))),
            PipeTile::UpLeft => Some(((pos.0, pos.1 - 1), (pos.0 - 1, pos.1))),
            PipeTile::DownLeft => Some(((pos.0, pos.1 + 1), (pos.0 - 1, pos.1))),
            PipeTile::DownRight => Some(((pos.0, pos.1 + 1), (pos.0 + 1, pos.1))),
            PipeTile::Start => None,
            PipeTile::Ground => None,
        }
    }
}

#[allow(clippy::ptr_arg)]
fn find_start(i: &Vec<Vec<PipeTile>>) -> (u32, u32) {
    i.iter()
        .enumerate()
        .find(|l| l.1.iter().any(|t| *t == PipeTile::Start))
        .map(|i| {
            (
                i.0 as u32,
                i.1.iter()
                    .enumerate()
                    .find(|t| *t.1 == PipeTile::Start)
                    .unwrap()
                    .0 as u32,
            )
        })
        .unwrap()
}

#[allow(clippy::ptr_arg)]
fn get(i: &Vec<Vec<PipeTile>>, pos: (u32, u32)) -> PipeTile {
    *i.get(pos.1 as usize).unwrap().get(pos.0 as usize).unwrap()
}

fn find_loop(i: Vec<Vec<PipeTile>>) {
    let mut start = find_start(&i);
    println!("{:?}", (start, get(&i, start).next(start)));
}

fn parse(s: &str) -> Vec<Vec<PipeTile>> {
    s.lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect()
}

fn process(s: &str) -> u32 {
    let input = parse(s);
    find_loop(input);

    420
}

fn process2(s: &str) -> u32 {
    let input = parse(s);
    println!("{:?}", input);

    todo!()
}

static TEST_INPUT: &str = ".....
.S-7.
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
