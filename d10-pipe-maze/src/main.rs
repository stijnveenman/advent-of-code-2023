#![allow(dead_code)]
#![allow(unused_variables)]
mod util;

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
        pipe.push(n);

        if cur == start {
            break;
        }
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

fn parse(s: &str) -> Vec<Vec<PipeTile>> {
    s.lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect()
}

fn process(s: &str) -> i32 {
    let input = parse(s);
    let l = find_loop(&input);

    l.unwrap().len() as i32 / 2
}

#[allow(clippy::ptr_arg)]
fn detect_crossings(l: &Vec<(i32, i32)>, pos: (i32, i32), x: i32, y: i32, w: i32, h: i32) -> i32 {
    let xr = 0..=w;
    let yr = 0..=h;

    let mut pos = pos;
    let mut crossings = 0;

    while xr.contains(&pos.0) && yr.contains(&pos.1) {
        if l.contains(&pos) {
            crossings += 1;
        }

        pos.0 += x;
        pos.1 += y;
    }

    crossings
}

#[allow(clippy::ptr_arg)]
fn contained(i: &Vec<Vec<PipeTile>>, l: &Vec<(i32, i32)>, pos: (i32, i32)) -> bool {
    if l.contains(&pos) {
        return false;
    }
    if detect_crossings(
        l,
        pos,
        -1,
        0,
        i.first().unwrap().len() as i32,
        i.len() as i32,
    ) % 2
        == 1
        && detect_crossings(
            l,
            pos,
            1,
            0,
            i.first().unwrap().len() as i32,
            i.len() as i32,
        ) % 2
            == 1
        && detect_crossings(
            l,
            pos,
            0,
            1,
            i.first().unwrap().len() as i32,
            i.len() as i32,
        ) % 2
            == 1
        && detect_crossings(
            l,
            pos,
            0,
            -1,
            i.first().unwrap().len() as i32,
            i.len() as i32,
        ) % 2
            == 1
    {
        return true;
    }

    false
}

fn process2(s: &str) -> i32 {
    let input = parse(s);
    let l = find_loop(&input).unwrap();

    let w = input.first().unwrap().len() as i32;
    let h = input.len() as i32;

    (0..w)
        .flat_map(|x| (0..h).map(move |y| (x, y)))
        .filter(|pos| contained(&input, &l, *pos))
        .dbg()
        .count() as i32
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
