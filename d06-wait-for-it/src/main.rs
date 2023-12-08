#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
}

trait Numbers {
    fn numbers(self) -> Vec<u64>;
}

impl Numbers for &str {
    fn numbers(self) -> Vec<u64> {
        let r = Regex::new(r"\d+").unwrap();
        r.captures_iter(self)
            .map(|m| m.get(0).unwrap().as_str().parse().unwrap())
            .collect::<Vec<_>>()
    }
}

fn race_points(race: (u64, u64)) -> u64 {
    let (time, distance) = race;

    (1..time)
        .map(|hold| {
            let speed = hold;
            let remaining = time - hold;
            speed * remaining
        })
        .filter(|d| *d > distance)
        .count() as u64
}

fn process(s: &str) -> u64 {
    let times = s.lines().next().unwrap().numbers();
    let distances = s.lines().last().unwrap().numbers();

    times
        .into_iter()
        .zip(distances)
        .map(race_points)
        .product::<u64>()
}

fn process2(s: &str) -> u64 {
    let time = s
        .lines()
        .next()
        .unwrap()
        .replace(' ', "")
        .numbers()
        .into_iter()
        .next()
        .unwrap();
    let distance = s
        .lines()
        .last()
        .unwrap()
        .replace(' ', "")
        .numbers()
        .into_iter()
        .next()
        .unwrap();

    println!("{:?}", (time, distance));

    race_points((time, distance))
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

    assert_eq!(dbg!(result), 71503)
}
