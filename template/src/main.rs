#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn process(s: &str) -> u32 {
    420
}

fn process2(s: &str) -> u32 {
    420
}

static TEST_INPUT: &str = "";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 420)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 420)
}
