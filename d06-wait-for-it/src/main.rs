#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

trait Numbers {
    fn numbers(self) -> Vec<u32>;
}

impl Numbers for &str {
    fn numbers(self) -> Vec<u32> {
        let r = Regex::new(r"\d+").unwrap();
        r.captures_iter(self)
            .map(|m| m.get(0).unwrap().as_str().parse().unwrap())
            .collect::<Vec<_>>()
    }
}

fn race_points(race: (u32, u32)) -> u32 {
    let (time, distance) = race;

    (1..time)
        .map(|hold| {
            let speed = hold;
            let remaining = time - hold;
            speed * remaining
        })
        .filter(|d| *d > distance)
        .count() as u32
}

fn process(s: &str) -> u32 {
    let times = s.lines().next().unwrap().numbers();
    let distances = s.lines().last().unwrap().numbers();

    times
        .into_iter()
        .zip(distances)
        .map(race_points)
        .product::<u32>()
}

fn process2(s: &str) -> u32 {
    todo!()
}

static TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 288)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
