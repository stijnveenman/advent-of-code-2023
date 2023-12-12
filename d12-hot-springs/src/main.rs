#![allow(dead_code)]
#![allow(unused_variables)]
use nom::{
    character::complete::{one_of, u32},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser, Slice,
};
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug, PartialEq, Eq)]
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

fn next_option(l: LineSlice) -> Option<LineSlice> {
    let count = *l.1.first()?;
    let mut cur = l.0;

    loop {
        while *cur.get(0)? == Point::Off {
            cur = cur.slice(1..);
        }

        if cur.get(0..count)?.iter().all(|i| *i != Point::Off) {
            return Some((cur, l.1));
        }

        cur = cur.slice(1..);
    }
}

fn count_options(l: LineSlice) -> usize {
    let mut s = 0;
    let mut cur = l;
    while !cur.0.is_empty() {
        let Some(next_o) = next_option(cur) else {
            //println!("no option found");
            break;
        };
        //println!("{:?}", next_o);

        if next_o.1.len() == 1 {
            //println!("returning full match\n");
            return 1;
        }

        let next_len = next_o.1.first().unwrap();
        let Some(next_sl) = next_o.0.get(next_len + 1..) else {
            return s;
        };
        let next = (next_sl, next_o.1.slice(1..));

        //println!(" {:?}", next);
        s += count_options(next);

        cur = (next_o.0.slice(1..), next_o.1);
    }

    s
}

fn process(s: &str) -> usize {
    let (_, input) = parse(s).unwrap();
    input.iter().map(|l| count_options(l.as_ref())).dbg().sum()
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
fn test_part1_2() {
    let result = process("???.### 1,1,3");

    assert_eq!(dbg!(result), 1)
}

#[test]
fn test_part1_3() {
    let result = process(".??..??...?##. 1,1,3");

    assert_eq!(dbg!(result), 4)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
