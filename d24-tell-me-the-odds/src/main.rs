#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use itertools::Itertools;
#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
static TEST_PART1_RESULT: usize = 2;
static TEST_PART2_RESULT: usize = 420;

#[derive(PartialEq, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Line {
    pos: Vec3,
    vel: Vec3,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn from_str(s: &str) -> Vec3 {
        let mut iter = s.split(',');
        Vec3 {
            x: iter
                .next()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap()
                .try_into()
                .unwrap(),
            y: iter
                .next()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap()
                .try_into()
                .unwrap(),
            z: iter
                .next()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap()
                .try_into()
                .unwrap(),
        }
    }
}

impl Line {
    fn new(pos: Vec3, vel: Vec3) -> Line {
        Line { pos, vel }
    }

    fn dx(&self) -> f64 {
        let x: f64 = self.vel.x;
        let y: f64 = self.vel.y;

        y / x
    }

    fn y0(&self) -> f64 {
        self.pos.y - (self.pos.x * self.dx())
    }

    fn at_x(&self, x: f64) -> f64 {
        self.y0() + (self.dx() * x)
    }
}

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn intersect(a: &Line, b: &Line) -> Vec3 {
    println!("a {}", a.at_x(14.333));
    println!("b {}", b.at_x(14.333));

    // a_y0 + a_dx * x = b_y0 + b_dx * x
    // a_dx * x - b_dx * x = b_y0 - a_y0
    // x = (b_y0 - a_y0) / (a_dx - b_dx)

    let x = (b.y0() - a.y0()) / (a.dx() - b.dx());
    let y = a.at_x(x);

    println!("{:?}", (x, y));
    Vec3 {
        x: (x * 10.0).round() / 10.0,
        y: (y * 10.0).round() / 10.0,
        z: 0.0,
    }
}

fn parse(s: &str) -> Vec<Line> {
    s.lines()
        .map(|l| {
            let mut iter = l.split('@');
            Line::new(
                Vec3::from_str(iter.next().unwrap()),
                Vec3::from_str(iter.next().unwrap()),
            )
        })
        .collect_vec()
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

#[test]
fn p_1() {
    let a = Line::new(Vec3::new(19.0, 13.0, 30.0), Vec3::new(-2.0, 1.0, -2.0));
    let b = Line::new(Vec3::new(18.0, 19.0, 22.0), Vec3::new(-1.0, -1.0, -2.0));

    assert_eq!(intersect(&a, &b), Vec3::new(14.3, 15.3, 0.0))
}
