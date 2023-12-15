#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

use itertools::Itertools;
use nom::{bytes::complete::take, multi::many0, IResult, Parser};
use nom_locate::LocatedSpan;
#[allow(unused_imports)]
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
}

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance_to(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn xy(&self, h: isize) -> isize {
        self.x + self.y * h
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

fn build_shiftmap_down(v: &[Rock]) -> Vec<Vec<isize>> {
    let width = v.iter().map(|r| r.point.x).max().unwrap();
    let height = v.iter().map(|r| r.point.y).max().unwrap();

    (0..=width)
        .map(|x| {
            (0..=height)
                .map(|y| {
                    v.iter()
                        .filter(|r| r.rock == RockType::Cube)
                        .filter(|r| r.point.x == x && r.point.y > y)
                        .map(|r| r.point.y - 1)
                        .min()
                        .unwrap_or(height)
                })
                .collect()
        })
        .collect()
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
fn draw(v: &[Point], b: &[Point]) {
    let width = v.iter().map(|r| r.x).max().unwrap();
    let height = v.iter().map(|r| r.y).max().unwrap();

    for y in 0..=height {
        for x in 0..=width {
            if v.iter().any(|r| r.x == x && r.y == y) {
                print!("O");
            } else if b.iter().any(|r| r.x == x && r.y == y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn build_shiftmap_left(v: &[Rock]) -> Vec<Vec<isize>> {
    let width = v.iter().map(|r| r.point.x).max().unwrap();
    let height = v.iter().map(|r| r.point.y).max().unwrap();

    (0..=height)
        .map(|y| {
            (0..=width)
                .map(|x| {
                    v.iter()
                        .filter(|r| r.rock == RockType::Cube)
                        .filter(|r| r.point.y == y && r.point.x < x)
                        .map(|r| r.point.x + 1)
                        .max()
                        .unwrap_or(0)
                })
                .collect()
        })
        .collect()
}

fn build_shiftmap_right(v: &[Rock]) -> Vec<Vec<isize>> {
    let width = v.iter().map(|r| r.point.x).max().unwrap();
    let height = v.iter().map(|r| r.point.y).max().unwrap();

    (0..=height)
        .map(|y| {
            (0..=width)
                .map(|x| {
                    v.iter()
                        .filter(|r| r.rock == RockType::Cube)
                        .filter(|r| r.point.y == y && r.point.x > x)
                        .map(|r| r.point.x - 1)
                        .min()
                        .unwrap_or(width)
                })
                .collect()
        })
        .collect()
}

fn shift_down(points: &mut [Point], maps: &[Vec<isize>]) {
    for (x, group) in &points
        .iter_mut()
        .sorted_by(|a, b| a.x.cmp(&b.x))
        .group_by(|r| r.x)
    {
        let map = &maps[x as usize];
        for (key, group) in group
            .sorted_by(|a, b| a.y.cmp(&b.y))
            .group_by(|v| map[v.y as usize])
            .into_iter()
        {
            for (offset, point) in group.enumerate() {
                point.y = key - offset as isize;
            }
        }
    }
}

fn shift_right(points: &mut [Point], maps: &[Vec<isize>]) {
    for (y, group) in &points
        .iter_mut()
        .sorted_by(|a, b| a.y.cmp(&b.y))
        .group_by(|r| r.y)
    {
        let map = &maps[y as usize];
        for (key, group) in group
            .sorted_by(|a, b| a.x.cmp(&b.x))
            .group_by(|v| map[v.x as usize])
            .into_iter()
        {
            for (offset, point) in group.enumerate() {
                point.x = key - offset as isize;
            }
        }
    }
}

fn shift_left(points: &mut [Point], maps: &[Vec<isize>]) {
    for (y, group) in &points
        .iter_mut()
        .sorted_by(|a, b| a.y.cmp(&b.y))
        .group_by(|r| r.y)
    {
        let map = &maps[y as usize];
        for (key, group) in group
            .sorted_by(|a, b| a.x.cmp(&b.x))
            .group_by(|v| map[v.x as usize])
            .into_iter()
        {
            for (offset, point) in group.enumerate() {
                point.x = key + offset as isize;
            }
        }
    }
}

fn shift_up(points: &mut [Point], maps: &[Vec<isize>]) {
    for (x, group) in &points
        .iter_mut()
        .sorted_by(|a, b| a.x.cmp(&b.x))
        .group_by(|r| r.x)
    {
        let map = &maps[x as usize];
        for (key, group) in group
            .sorted_by(|a, b| a.y.cmp(&b.y))
            .group_by(|v| map[v.y as usize])
            .into_iter()
        {
            for (offset, point) in group.enumerate() {
                point.y = key + offset as isize;
            }
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

    let maps = build_shiftmap_up(input.as_slice());

    let mut points = input
        .into_iter()
        .filter(|r| r.rock == RockType::Round)
        .map(|r| r.point)
        .collect::<Vec<_>>();

    shift_up(points.as_mut_slice(), &maps);

    calculate_load(points.as_slice(), height)
}

fn process2(s: &str) -> isize {
    let (_, input) = parse(LocatedSpan::new(s)).unwrap();
    let width = input.iter().map(|r| r.point.x).max().unwrap();
    let height = input.iter().map(|r| r.point.y).max().unwrap();

    let up_maps = build_shiftmap_up(input.as_slice());
    let down_maps = build_shiftmap_down(input.as_slice());
    let right_maps = build_shiftmap_right(input.as_slice());
    let left_maps = build_shiftmap_left(input.as_slice());

    let mut points = input
        .iter()
        .filter(|&r| r.rock == RockType::Round)
        .cloned()
        .map(|r| r.point)
        .collect::<Vec<_>>();

    let blocks = input
        .into_iter()
        .filter(|r| r.rock == RockType::Cube)
        .map(|r| r.point)
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let mut it_remaining = None;

    for i in 0..1000000000 {
        if i % 1000 == 0 {
            println!("i {}", i)
        }
        shift_up(points.as_mut_slice(), &up_maps);
        shift_left(points.as_mut_slice(), &left_maps);
        shift_down(points.as_mut_slice(), &down_maps);
        shift_right(points.as_mut_slice(), &right_maps);

        if let Some(rem) = it_remaining {
            it_remaining = Some(rem - 1);
            if rem - 2 == 0 {
                break;
            }
            println!("done at i {}", i);
        } else {
            let mut c = points.to_vec();
            c.sort_by_key(|a| a.xy(height));
            if let Some(idx) = cache.get(&c) {
                println!("cache found {} {}", i, idx);
                let loop_len = i - idx;
                println!("loop size {}", loop_len);
                let loop_remaining = 1000000000 - i;
                println!("{} {}", loop_remaining, loop_remaining % loop_len);

                it_remaining = Some(loop_remaining % loop_len);
            } else {
                cache.insert(c, i);
            }
        }
    }

    calculate_load(points.as_slice(), height)
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
