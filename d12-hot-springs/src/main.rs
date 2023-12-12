#![allow(dead_code)]
#![allow(unused_variables)]
use nom::{
    character::complete::{i32, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug)]
enum Point {
    On,
    Off,
    Unkown,
}

fn to_point(c: char) -> Point {
    match c {
        '.' => Point::Off,
        '#' => Point::On,
        '?' => Point::Unkown,
        _ => panic!("unknown char"),
    }
}

type Line = (Vec<Point>, Vec<i32>);

fn parse(s: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(
        one_of("\n"),
        separated_pair(
            many1(one_of(".?#").map(to_point)),
            one_of(" "),
            separated_list1(one_of(","), i32),
        ),
    )(s)
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

static TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 21)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
