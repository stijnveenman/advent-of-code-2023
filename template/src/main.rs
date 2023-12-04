#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn process(s: &str) -> u32 {
    todo!()
}

fn process2(s: &str) -> u32 {
    todo!()
}

static TEST_INPUT: &str = "";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
