#![allow(dead_code)]
#![allow(unused_variables)]
use nom::{
    bytes::complete::{is_a, tag},
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

#[derive(Debug, Clone, Copy)]
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

fn get_size(points: &[Point]) -> Point {
    Point {
        x: points.iter().map(|p| p.x).max().unwrap(),
        y: points.iter().map(|p| p.y).max().unwrap(),
    }
}

fn expand(points: Vec<Point>) -> Vec<Point> {
    let size = get_size(points.as_slice());

    let empty_cols = (0..size.x)
        .filter(|x| points.iter().all(|p| p.x != *x))
        .collect::<Vec<_>>();
    let empty_rows = (0..size.y)
        .filter(|y| points.iter().all(|p| p.y != *y))
        .collect::<Vec<_>>();

    points
        .into_iter()
        .map(|p| {
            let dx = empty_cols.iter().filter(|c| p.x > **c).count() as isize;
            let dy = empty_rows.iter().filter(|c| p.y > **c).count() as isize;

            Point {
                x: p.x + dx,
                y: p.y + dy,
            }
        })
        .dbg()
        .collect()
}

fn process(s: &str) -> u32 {
    let (_, points) = parse(LocatedSpan::new(s)).unwrap();

    let points = expand(points);

    println!("{:?}", points);

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
