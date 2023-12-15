#![allow(dead_code)]
#![allow(unused_variables)]

use itertools::Itertools;
use nom::{bytes::complete::take, multi::many0, IResult, Parser};
use nom_locate::LocatedSpan;
#[allow(unused_imports)]
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance_to(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RockType {
    Round,
    Cube,
    Unknown,
}

#[derive(Debug, Clone)]
struct Rock {
    point: Point,
    rock: RockType,
}

impl Rock {
    fn new(p: Point, t: RockType) -> Rock {
        Rock { point: p, rock: t }
    }
}

fn as_rock(span: Span) -> Rock {
    let p = Point {
        x: span.get_column() as isize - 1,
        y: span.location_line() as isize - 1,
    };
    match *span.fragment() {
        "O" => Rock::new(p, RockType::Round),
        "#" => Rock::new(p, RockType::Cube),
        _ => Rock::new(p, RockType::Unknown),
    }
}
fn calculate_load(v: &[Point], h: isize) -> isize {
    v.iter().map(|r| h + 1 - r.y).sum()
}

fn build_shiftmap_up(v: &[Rock]) -> Vec<Vec<isize>> {
    let width = v.iter().map(|r| r.point.x).max().unwrap();
    let height = v.iter().map(|r| r.point.y).max().unwrap();

    (0..=width)
        .map(|x| {
            (0..=height)
                .map(|y| {
                    v.iter()
                        .filter(|r| r.rock == RockType::Cube)
                        .filter(|r| r.point.x == x && r.point.y < y)
                        .map(|r| r.point.y + 1)
                        .max()
                        .unwrap_or(0)
                })
                .collect()
        })
        .collect()
}

fn shift_up(v: &mut [&mut Point], map: &[isize]) {
    for (key, group) in v
        .iter_mut()
        .sorted_by(|a, b| a.y.cmp(&b.y))
        .group_by(|v| map[v.y as usize])
        .into_iter()
    {
        for (offset, point) in group.enumerate() {
            point.y = key + offset as isize;
        }
    }
}

fn parse(s: Span) -> IResult<Span, Vec<Rock>> {
    let (s, result) = many0(take(1u8).map(as_rock))(s)?;
    Ok((
        s,
        result
            .into_iter()
            .filter(|s| s.rock != RockType::Unknown)
            .collect(),
    ))
}

fn process(s: &str) -> isize {
    let (_, input) = parse(LocatedSpan::new(s)).unwrap();
    let width = input.iter().map(|r| r.point.x).max().unwrap();
    let height = input.iter().map(|r| r.point.y).max().unwrap();

    let map = build_shiftmap_up(input.as_slice());

    let mut points = input
        .into_iter()
        .filter(|r| r.rock == RockType::Round)
        .map(|r| r.point)
        .collect::<Vec<_>>();

    for (x, group) in &points
        .iter_mut()
        .sorted_by(|a, b| a.x.cmp(&b.x))
        .group_by(|r| r.x)
    {
        let mut b = group.collect::<Vec<_>>();
        shift_up(b.as_mut_slice(), map[x as usize].as_ref());
    }

    calculate_load(points.as_slice(), height)
}

fn process2(s: &str) -> isize {
    let (_, input) = parse(LocatedSpan::new(s)).unwrap();
    let width = input.iter().map(|r| r.point.x).max().unwrap();
    let height = input.iter().map(|r| r.point.y).max().unwrap();

    todo!()
}

static TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 136)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 64)
}
