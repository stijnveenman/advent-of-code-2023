#![allow(dead_code)]
#![allow(unused_variables)]

use nom::{
    character::complete::{i32, newline, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn parse(s: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, separated_list1(space1, i32))(s)
}

fn difference(i: &[i32]) -> Vec<i32> {
    i.iter().zip(i.iter().skip(1)).map(|(a, b)| b - a).collect()
}

fn predict(i: Vec<i32>) -> i32 {
    let mut diffs = vec![i];

    loop {
        let p = difference(diffs.last().unwrap().as_slice());
        if p.iter().all(|n| *n == 0) {
            break;
        }
        diffs.push(p);
    }

    diffs
        .into_iter()
        .rev()
        .fold(0, |current, v| current + v.last().unwrap())
}

fn process(s: &str) -> i32 {
    let (_, input) = parse(s).unwrap();

    input.into_iter().map(predict).sum()
}

fn process2(s: &str) -> i32 {
    todo!()
}

static TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 114)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
