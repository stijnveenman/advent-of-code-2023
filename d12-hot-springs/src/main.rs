#![allow(dead_code)]
#![allow(unused_variables)]
use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{one_of, space1, u32},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser, Slice,
};
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{:?}", process(input))
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

        if cur.get(0..count)?.iter().all(|i| *i != Point::Off)
            && cur.get(count).map(|n| *n != Point::On).unwrap_or(true)
        {
            return Some((cur, l.1));
        }

        if *cur.get(0)? == Point::On {
            return None;
        }
        cur = cur.slice(1..);
    }
}
#[derive(Debug)]
struct Puzzle<'a> {
    spaces_to_fill: u32,
    line: &'a str,
    batches: Vec<u32>,
}
fn parse_line(input: &str) -> IResult<&str, Puzzle> {
    let (input, (line, batches)) =
        separated_pair(is_a("?.#"), space1, separated_list1(tag(","), u32))(input)?;

    let spaces_to_fill = line.chars().filter(|c| c == &'?').count() as u32;

    Ok((
        input,
        Puzzle {
            spaces_to_fill,
            line,
            batches,
        },
    ))
}

impl<'a> Puzzle<'a> {
    fn generate_permutations(&self) -> Vec<String> {
        let options: Vec<String> = repeat_n([".", "#"].into_iter(), self.spaces_to_fill as usize)
            .multi_cartesian_product()
            .map(|v| v.join(""))
            .collect();

        options
    }
    fn check_option(&self, option: &str) -> bool {
        let mut option_iter = option.chars();
        let filled_option = self
            .line
            .chars()
            .map(|c| match c {
                '?' => option_iter
                    .next()
                    .expect("should have a length similar to needed gaps"),
                value => value,
            })
            .collect::<String>();
        let counts = filled_option
            .chars()
            .group_by(|c| c == &'#')
            .into_iter()
            .filter_map(|(is_hashes, group)| is_hashes.then_some(group.into_iter().count() as u32))
            .collect::<Vec<u32>>();
        self.batches[..] == counts[..]
    }
    fn possible_solution_count(&self) -> usize {
        let options = self.generate_permutations();
        let count = options
            .iter()
            .filter(|option| self.check_option(option))
            .count();
        count
    }
}
fn process_line(input: &str) -> usize {
    let (_input, puzzle) = parse_line(input).expect("should parse a valid line");

    puzzle.possible_solution_count()
}

fn count_options(l: LineSlice) -> usize {
    let mut s = 0;
    let mut cur = l;
    while !cur.0.is_empty() {
        //println!("-> {:?}", cur);
        let Some(next_o) = next_option(cur) else {
            //println!("<-- no option found");
            break;
        };
        //println!("<- {:?}", next_o);

        if next_o.1.len() == 1 {
            //println!("found full match\n");
            let next_len = next_o.1.first().unwrap();
            // TODO not sure
            if !next_o
                .0
                .get(*next_len..)
                .unwrap()
                .iter()
                .any(|p| *p == Point::On)
            {
                s += 1;
            };
        } else {
            let next_len = next_o.1.first().unwrap();
            let Some(next_sl) = next_o.0.get(next_len + 1..) else {
                return s;
            };
            let next = (next_sl, next_o.1.slice(1..));

            //println!(" ->> {:?}", next);
            s += count_options(next);
        }

        if *next_o.0.get(0).unwrap() == Point::On {
            return s;
        }

        cur = (next_o.0.slice(1..), next_o.1);
    }

    s
}

fn compare(s: &str) {
    let result = s
        .lines()
        .map(|l| {
            let (_, input) = parse(l).unwrap();
            let mine = count_options(input.first().unwrap().as_ref());
            let theirs = process_line(l);

            (l, theirs, mine)
        })
        .filter(|l| l.1 != l.2)
        .inspect(|l| println!("{} expected: {} got {}", l.0, l.1, l.2))
        .fold((0, 0), |a, b| (a.0 + b.1, a.1 + b.2));

    println!(
        "final result expected {} got {} diff {}",
        result.0,
        result.1,
        result.0.abs_diff(result.1)
    );
}

fn process(s: &str) -> usize {
    let (_, input) = parse(s).unwrap();
    input.iter().map(|l| count_options(l.as_ref())).sum()
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
fn test_part1_4() {
    let result = process("?#?#?#?#?#?#?#? 1,3,1,6");

    assert_eq!(dbg!(result), 1)
}

#[test]
fn test_part1_5() {
    let result = process("?????#??.????? 7,4");

    assert_eq!(dbg!(result), 4)
}

#[test]
fn test_part1_6() {
    let result = process("?###???????? 3,2,1");

    assert_eq!(dbg!(result), 10)
}

#[test]
fn test_part1_7() {
    let result = process("#.?#??#???##? 1,2,7");

    assert_eq!(dbg!(result), 3)
}

#[test]
fn test_part1_8() {
    let result = process("??#.????#???.#?# 1,5,1,1,1");

    assert_eq!(dbg!(result), 3)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
