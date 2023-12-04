#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn process(s: &str) -> u32 {
    let mut iter = s.lines().peekable();

    let mut prev = None;
    let r = Regex::new(r"\d+").unwrap();

    while let Some(current) = iter.next() {
        let next = iter.peek();

        r.captures_iter(current).for_each(|m| {
            println!("{}", m.get(0).unwrap().as_str());
        });

        prev = Some(current);
    }

    todo!()
}

fn process2(s: &str) -> u32 {
    todo!()
}

static TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

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
