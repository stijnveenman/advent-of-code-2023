#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::str::FromStr;

use itertools::Itertools;
#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
static TEST_PART1_RESULT: usize = 5;
static TEST_PART2_RESULT: usize = 420;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug)]
struct Vec3 {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Vec3 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');

        Ok(Self::new(
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        ))
    }
}

impl Vec3 {
    fn new(x: usize, y: usize, z: usize) -> Vec3 {
        Vec3 { x, y, z }
    }
}

#[derive(Debug)]
struct Brick {
    start: Vec3,
    end: Vec3,
}

impl Brick {
    fn new(start: Vec3, end: Vec3) -> Brick {
        Brick { start, end }
    }

    fn min_z(&self) -> usize {
        self.start.z.min(self.end.z)
    }
}

fn parse(s: &str) -> Vec<Brick> {
    s.lines()
        .map(|l| {
            let (start, end) = l.split_once('~').unwrap();
            Brick::new(start.parse().unwrap(), end.parse().unwrap())
        })
        .collect_vec()
}

fn settle(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_by_key(|b| b.min_z());
    let mut settled = vec![];

    for brick in bricks {
        if brick.min_z() == 1 {
            settled.push(brick);
            continue;
        }
    }

    settled
}

fn process(s: &str) -> usize {
    let input = parse(s);
    let bricks = settle(input);
    println!("{:?}", bricks);

    todo!()
}

fn process2(s: &str) -> usize {
    let input = parse(s);
    println!("{:?}", input);

    todo!()
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
