#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn parse(s: &str) -> Vec<&str> {
    s.split(',').collect()
}

fn hash(s: &str) -> usize {
    s.chars()
        .filter(|c| *c != '\n')
        .fold(0, |cur, c| ((cur + c as usize) * 17) % 256)
}

fn process(s: &str) -> usize {
    let input = parse(s);
    input.into_iter().dbg().map(hash).sum()
}

fn process2(s: &str) -> u32 {
    let input = parse(s);
    println!("{:?}", input);

    todo!()
}

static TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 1320)
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
