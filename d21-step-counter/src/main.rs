#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "";
static TEST_PART1_RESULT: usize = 420;
static TEST_PART2_RESULT: usize = 420;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn parse(s: &str) -> &str {
    s
}

fn process(s: &str) -> usize {
    let input = parse(s);
    println!("{:?}", input);

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
