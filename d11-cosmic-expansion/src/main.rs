#![allow(dead_code)]
#![allow(unused_variables)]
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self},
    multi::many0,
    sequence::preceded,
    IResult, Parser,
};
mod util;
use nom_locate::LocatedSpan;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

fn as_point(span: Span) -> Point {
    Point {
        x: span.get_column() as isize - 1,
        y: span.location_line() as isize - 1,
    }
}

fn parse(s: Span) -> IResult<Span, Vec<Point>> {
    many0(preceded(is_a(".\n"), tag("#").map(as_point)))(s)
}

fn process(s: &str) -> u32 {
    let (_, input) = parse(LocatedSpan::new(s)).unwrap();
    println!("{:?}", input);

    todo!()
}

fn process2(s: &str) -> u32 {
    let (_, input) = parse(LocatedSpan::new(s)).unwrap();
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
