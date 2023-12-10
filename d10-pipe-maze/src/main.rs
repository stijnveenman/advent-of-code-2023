#![allow(dead_code)]
#![allow(unused_variables)]
mod util;

use nom::{
    character::complete::{self, none_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
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
    fn next_options(&self, pos: (i32, i32)) -> Option<((i32, i32), (i32, i32))> {
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

    fn next(&self, pos: (i32, i32), prev: (i32, i32)) -> Option<(i32, i32)> {
        self.next_options(pos).and_then(|(a, b)| {
            if a == prev {
                return Some(b);
            }
            if b == prev {
                return Some(a);
            }
            None
        })
    }
}

#[allow(clippy::ptr_arg)]
fn find_start(i: &Vec<Vec<PipeTile>>) -> (i32, i32) {
    i.iter()
        .enumerate()
        .find(|l| l.1.iter().any(|t| *t == PipeTile::Start))
        .map(|i| {
            (
                i.1.iter()
                    .enumerate()
                    .find(|t| *t.1 == PipeTile::Start)
                    .unwrap()
                    .0 as i32,
                i.0 as i32,
            )
        })
        .unwrap()
}

#[allow(clippy::ptr_arg)]
fn get(i: &Vec<Vec<PipeTile>>, pos: (i32, i32)) -> Option<&PipeTile> {
    i.get(pos.1 as usize).and_then(|l| l.get(pos.0 as usize))
}

fn try_find_loop(i: &Vec<Vec<PipeTile>>, x: i32, y: i32) -> Option<Vec<(i32, i32)>> {
    let start = find_start(i);

    let mut prev = start;
    let mut cur = (start.0 + x, start.1 + y);
    let mut pipe = vec![prev, cur];
    loop {
        let Some(n) = get(i, cur).and_then(|n| n.next(cur, prev)) else {
            return None;
        };
        prev = cur;
        cur = n;

        if cur == start {
            break;
        }
        pipe.push(n);
    }
    Some(pipe)
}

#[allow(clippy::ptr_arg)]
fn find_loop(i: &Vec<Vec<PipeTile>>) -> Option<Vec<(i32, i32)>> {
    try_find_loop(i, -1, 0)
        .or_else(|| try_find_loop(i, 1, 0))
        .or_else(|| try_find_loop(i, 0, 1))
        .or_else(|| try_find_loop(i, 0, -1))
}

fn parse(s: &str) -> IResult<&str, Vec<Vec<PipeTile>>> {
    separated_list1(complete::char('\n'), many1(none_of("\n").map(|c| c.into())))(s)
}

fn process(s: &str) -> i32 {
    let (_, input) = parse(s).unwrap();
    let l = find_loop(&input);

    l.unwrap().len() as i32 / 2
}

#[allow(clippy::ptr_arg)]
fn count_outside(i: &Vec<Vec<PipeTile>>, l: &[(i32, i32)]) -> i32 {
    let mut c = 0;

    for y in 0..i.len() {
        let mut outside = true;
        for x in 0..i.first().unwrap().len() {
            let pos = (x as i32, y as i32);

            if l.contains(&pos) {
                match get(i, pos).unwrap() {
                    PipeTile::Vertical => outside = !outside,
                    PipeTile::Horizontal => (),
                    PipeTile::UpRight => (),
                    PipeTile::UpLeft => (),
                    PipeTile::DownLeft => outside = !outside,
                    PipeTile::DownRight => outside = !outside,
                    PipeTile::Start => (),
                    PipeTile::Ground => (),
                }
            } else if !outside {
                println!("{:?}", pos);
                c += 1
            }
        }
    }

    c
}

#[allow(clippy::ptr_arg)]
fn fix_start(v: &mut Vec<Vec<PipeTile>>, l: &[(i32, i32)]) {
    let first = l.get(1).unwrap();
    let end = l.last().unwrap();

    let diff = (first.0 - end.0, first.1 - end.1);

    let new_start = match diff {
        (1, -1) => PipeTile::DownRight,
        (-1, -1) => PipeTile::DownLeft,
        _ => panic!("unhandled diff {:?}", diff),
    };

    let start = l.first().unwrap();
    let row = v.get_mut(start.1 as usize).unwrap();
    row[start.0 as usize] = new_start;
}

fn process2(s: &str) -> i32 {
    let (_, mut input) = parse(s).unwrap();
    let l = find_loop(&input).unwrap();
    fix_start(&mut input, l.as_slice());
    let _ = input.iter().dbg().collect::<Vec<_>>();

    count_outside(&input, l.as_slice())
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
fn test_part1_2() {
    let result = process(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
    );

    assert_eq!(dbg!(result), 4)
}

#[test]
fn test_part1_3() {
    let result = process(
        "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
    );

    assert_eq!(dbg!(result), 8)
}

#[test]
fn test_part2() {
    let result = process2(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
    );

    assert_eq!(dbg!(result), 4)
}

#[test]
fn test_part2_2() {
    let result = process2(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
    );

    assert_eq!(dbg!(result), 10)
}

#[test]
fn test_part2_3() {
    let result = process2(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
    );

    assert_eq!(dbg!(result), 8)
}
