#![allow(dead_code)]
#![allow(unused_variables)]
use nom::{
    character::complete::{newline, space1},
    multi::separated_list1,
    IResult,
};
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn parse(s: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(newline, separated_list1(space1, u32))(s)
}

fn process(s: &str) -> usize {
    let (_, input) = parse(s).unwrap();
    println!("{:?}", input);

    todo!()
}

fn process2(s: &str) -> usize {
    let (_, input) = parse(s).unwrap();
    println!("{:?}", input);

    todo!()
}

static TEST_INPUT: &str = "";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
