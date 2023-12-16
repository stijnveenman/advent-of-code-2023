#![allow(dead_code)]
#![allow(unused_variables)]
use std::ops::Add;
use std::{collections::HashMap, ops::AddAssign};

use nom::{bytes::complete::take, multi::many0, IResult, Parser};
use nom_locate::LocatedSpan;
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}
type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    const RIGHT: Point = Point { x: 1, y: 0 };
    const LEFT: Point = Point { x: -1, y: 0 };
    const DOWN: Point = Point { x: 0, y: 1 };
    const UP: Point = Point { x: 0, y: -1 };

    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn bounded(&self, w: isize, h: isize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < w && self.y < h
    }

    fn distance_to(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn reflect(&self, tile: &TileType) -> Point {
        if tile == &TileType::MirrorLeft {
            return Point {
                x: self.y,
                y: self.x,
            };
        }
        if tile == &TileType::MirrorRight {
            return Point {
                x: -self.y,
                y: -self.x,
            };
        }

        panic!("unimplemented mirror");
    }

    fn split(&self, tile: &TileType) -> Option<(Point, Point)> {
        if tile == &TileType::SplitterVertical {
            if self.x == 0 {
                return None;
            }
            return Some((Point::UP, Point::DOWN));
        }

        panic!("unimplemented!() split");
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum TileType {
    MirrorRight,
    MirrorLeft,
    SplitterHorizontal,
    SplitterVertical,
    Unknown,
}

#[derive(Debug, Clone)]
struct Tile {
    point: Point,
    tile: TileType,
}

impl Tile {
    fn new(p: Point, t: TileType) -> Tile {
        Tile { point: p, tile: t }
    }
}

fn as_rock(span: Span) -> Tile {
    let p = Point {
        x: span.get_column() as isize - 1,
        y: span.location_line() as isize - 1,
    };
    match *span.fragment() {
        "-" => Tile::new(p, TileType::SplitterHorizontal),
        "|" => Tile::new(p, TileType::SplitterVertical),
        "/" => Tile::new(p, TileType::MirrorRight),
        "\\" => Tile::new(p, TileType::MirrorLeft),
        _ => Tile::new(p, TileType::Unknown),
    }
}

fn parse(s: Span) -> IResult<Span, HashMap<Point, TileType>> {
    let (s, result) = many0(take(1u8).map(as_rock))(s)?;
    Ok((
        s,
        result
            .into_iter()
            .filter(|s| s.tile != TileType::Unknown)
            .map(|s| (s.point, s.tile))
            .collect(),
    ))
}

fn process(s: &str) -> usize {
    let (_, input) = parse(LocatedSpan::new(s)).unwrap();
    let width = input.iter().map(|x| x.0.x).max().unwrap() + 1;
    let height = input.iter().map(|x| x.0.y).max().unwrap() + 1;

    let mut remaining = vec![(Point::new(-1, 0), Point::RIGHT)];

    while let Some(p) = remaining.pop() {
        let mut dir = p.1;
        let mut pos = p.0 + dir;

        while pos.bounded(width, height) {
            println!("{:?}", pos);

            if let Some(tile) = input.get(&pos) {
                match tile {
                    TileType::MirrorRight => dir = dir.reflect(tile),
                    TileType::MirrorLeft => dir = dir.reflect(tile),
                    TileType::SplitterHorizontal => (),
                    TileType::SplitterVertical => {
                        if let Some(split) = dir.split(tile) {
                            println!("{:?}", split);
                            remaining.push((pos, split.0));
                            remaining.push((pos, split.1));
                            break;
                        };
                    }
                    TileType::Unknown => (),
                }
            }

            pos += dir;
        }
    }

    todo!()
}

fn process2(s: &str) -> usize {
    let (_, input) = parse(LocatedSpan::new(s)).unwrap();
    println!("{:?}", input);

    todo!()
}

static TEST_INPUT: &str = include_str!("./test-input.txt");

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 46)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
