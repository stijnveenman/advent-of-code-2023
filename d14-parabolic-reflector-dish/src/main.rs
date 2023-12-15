#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

use nom::{bytes::complete::take, multi::many0, IResult, Parser};
use nom_locate::LocatedSpan;
#[allow(unused_imports)]

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

fn shift_up(v: &[Rock], x: isize, h: isize) -> Vec<Rock> {
    let rocks = v
        .iter()
        .filter(|r| r.point.x == x)
        .map(|r| (r.point.y, r))
        .collect::<HashMap<_, _>>();
    let mut count = 0;
    let mut new_rocks = vec![];

    for y in (0..=h).rev() {
        if let Some(r) = rocks.get(&y) {
            match r.rock {
                RockType::Round => count += 1,
                RockType::Unknown => todo!(),
                RockType::Cube => {
                    for i in 0..count {
                        new_rocks.push(Rock::new(Point { x, y: y + i + 1 }, RockType::Round))
                    }
                    count = 0;
                }
            }
        }
    }

    for i in 0..count {
        println!("{} {}", x, i);
        new_rocks.push(Rock::new(Point { x, y: i }, RockType::Round));
    }

    new_rocks.append(
        &mut v
            .iter()
            .filter(|&x| x.rock == RockType::Cube)
            .cloned()
            .collect(),
    );
    new_rocks
}

fn shift_right(v: &[Rock], y: isize, w: isize) -> Vec<Rock> {
    let rocks = v
        .iter()
        .filter(|r| r.point.y == y)
        .map(|r| (r.point.x, r))
        .collect::<HashMap<_, _>>();
    let mut count = 0;
    let mut new_rocks = vec![];

    for x in (0..=w).rev() {
        if let Some(r) = rocks.get(&x) {
            match r.rock {
                RockType::Round => count += 1,
                RockType::Unknown => todo!(),
                RockType::Cube => {
                    for i in 0..count {
                        new_rocks.push(Rock::new(Point { x: x - i - 1, y }, RockType::Round))
                    }
                    count = 0;
                }
            }
        }
    }

    for i in 0..count {
        new_rocks.push(Rock::new(Point { x: i, y }, RockType::Round));
    }

    new_rocks.append(
        &mut v
            .iter()
            .filter(|&x| x.rock == RockType::Cube)
            .cloned()
            .collect(),
    );
    new_rocks
}

fn calculate_load(v: &[Rock], h: isize) -> isize {
    v.iter()
        .filter(|r| r.rock == RockType::Round)
        .map(|r| h + 1 - r.point.y)
        .sum()
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
    (0..=width)
        .map(|x| shift_up(input.as_slice(), x, height))
        .map(|x| calculate_load(x.as_slice(), height))
        .sum()
}

fn process2(s: &str) -> isize {
    let (_, mut input) = parse(LocatedSpan::new(s)).unwrap();
    let width = input.iter().map(|r| r.point.x).max().unwrap();
    let height = input.iter().map(|r| r.point.y).max().unwrap();

    for _ in 0..1000000000 {
        input = (0..width)
            .flat_map(|x| shift_up(input.as_slice(), x, height))
            .collect();

        input = (0..height)
            .flat_map(|y| shift_right(input.as_slice(), y, width))
            .collect();
    }
    calculate_load(input.as_slice(), height)
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

    assert_eq!(dbg!(result), 0)
}