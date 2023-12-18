#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug)]
struct LineItem {
    dir: char,
    count: usize,
    hex: String,
}

fn parse(s: &str) -> Vec<LineItem> {
    s.lines()
        .map(|l| {
            let mut i = l.split(' ');
            LineItem {
                dir: i.next().unwrap().chars().next().unwrap(),
                count: i.next().unwrap().parse().unwrap(),
                hex: i.next().unwrap().replace(['(', ')'], ""),
            }
        })
        .collect()
}

fn process(s: &str) -> usize {
    let input = parse(s);
    println!("{:?}", input);

    todo!()
}

fn process2(s: &str) -> usize {
    todo!()
}

static TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 62)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
