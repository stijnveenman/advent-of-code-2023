#![allow(dead_code)]
#![allow(unused_variables)]
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

#[derive(Debug, PartialEq, Eq)]
enum RockType {
    Round,
    Cube,
    Unknown,
}

#[derive(Debug)]
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

fn calculate_load(v: &[Rock], x: isize) -> usize {
    let mut rocks = v.iter().filter(|r| r.point.x == x).collect::<Vec<_>>();
    rocks.sort_by_key(|x| x.point.y);

    println!("{:?}", rocks);

    420
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

fn process(s: &str) -> usize {
    let (_, input) = parse(LocatedSpan::new(s)).unwrap();
    let width = input.iter().map(|r| r.point.x).max().unwrap();
    (0..=width)
        .map(|x| calculate_load(input.as_slice(), x))
        .sum()
}

fn process2(s: &str) -> usize {
    let (_, input) = parse(LocatedSpan::new(s)).unwrap();
    println!("{:?}", input);

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

    assert_eq!(dbg!(result), 0)
}
