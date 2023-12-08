#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

struct LoopingIterator<T> {
    i: T,
    start: T,
}

impl<T> Iterator for LoopingIterator<T>
where
    T: Iterator + Clone,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.i.next();

        if result.is_some() {
            return result;
        }

        self.i = self.start.clone();
        self.i.next()
    }
}

trait CreateLoopingIterator<T> {
    fn looping(&self) -> LoopingIterator<T>;
}

impl<T> CreateLoopingIterator<T> for T
where
    T: Iterator + Clone,
{
    fn looping(&self) -> LoopingIterator<T> {
        LoopingIterator {
            i: self.clone(),
            start: self.clone(),
        }
    }
}

#[allow(clippy::explicit_counter_loop)]
fn process(s: &str) -> u32 {
    let steps = s.lines().next().unwrap().trim();

    let map = s
        .lines()
        .skip(2)
        .map(|l| {
            let (from, to) = l.split_once('=').unwrap();
            let to = to.trim();
            let to = &to[1..to.len() - 1];
            let (left, right) = to.split_once(',').unwrap();

            (from.trim(), (left.trim(), right.trim()))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    println!("{:?}", map);

    let mut count = 0;
    let mut pos = s.lines().nth(2).unwrap().split_once(' ').unwrap().0;
    let iter = steps.chars().looping();
    for step in iter {
        let next = map.get(&pos).unwrap();

        pos = match step {
            'L' => next.0,
            'R' => next.1,
            _ => panic!("invalid step"),
        };

        count += 1;

        if pos == "ZZZ" {
            break;
        }
    }

    count
}

fn process2(s: &str) -> u32 {
    todo!()
}

static TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 2)
}

#[test]
fn test_part1_1() {
    let result = process(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
    );

    assert_eq!(dbg!(result), 6)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
