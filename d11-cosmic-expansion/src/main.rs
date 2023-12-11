#![allow(dead_code)]
#![allow(unused_variables)]
use nom::{
    bytes::complete::is_a,
    character::complete::{self},
    multi::many0,
    sequence::preceded,
    IResult,
};
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn parse(s: &str) -> IResult<&str, Vec<char>> {
    many0(preceded(is_a(".\n"), complete::char('#')))(s)
}

fn process(s: &str) -> u32 {
    let (_, input) = parse(s).unwrap();
    println!("{:?}", input);

    todo!()
}

fn process2(s: &str) -> u32 {
    let (_, input) = parse(s).unwrap();
    println!("{:?}", input);

    todo!()
}

static TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 374)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
