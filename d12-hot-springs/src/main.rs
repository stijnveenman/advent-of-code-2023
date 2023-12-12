#![allow(dead_code)]
#![allow(unused_variables)]
use nom::{
    character::complete::{i32, one_of, u32},
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

#[derive(Debug)]
struct Line(Vec<Point>, Vec<usize>);
type LineSlice<'a> = (&'a [Point], &'a [usize]);

impl Line {
    fn as_ref(&self) -> LineSlice {
        (self.0.as_ref(), self.1.as_ref())
    }
}

fn parse(s: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(
        one_of("\n"),
        separated_pair(
            many1(one_of(".?#").map(to_point)),
            one_of(" "),
            separated_list1(one_of(","), u32.map(|d| d as usize)),
        )
        .map(|l| Line(l.0, l.1)),
    )(s)
}

fn count_options(l: LineSlice) -> usize {
    2
}

fn process(s: &str) -> u32 {
    let (_, input) = parse(s).unwrap();
    println!("{:?}", count_options(input.first().unwrap().as_ref()));

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
