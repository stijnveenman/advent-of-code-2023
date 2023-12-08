#![allow(dead_code)]
#![allow(unused_variables)]

use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    cards: &'a str,
    bet: u32,
}

impl<'a> Hand<'a> {
    fn get_type(&self) -> u32 {
        let hm = self.hashmap();

        if hm.iter().any(|c| *c.1 == 5) {
            return 7;
        }

        if hm.iter().any(|c| *c.1 == 4) {
            return 6;
        }

        if hm.iter().any(|c| *c.1 == 2) && hm.iter().any(|c| *c.1 == 3) {
            return 5;
        }

        if hm.iter().any(|c| *c.1 == 3) {
            return 4;
        }

        if hm.iter().filter(|c| *c.1 == 2).count() == 2 {
            return 3;
        }

        if hm.iter().any(|c| *c.1 == 2) {
            return 2;
        }

        1
    }

    fn hashmap(&self) -> HashMap<char, u32> {
        let mut h = HashMap::new();

        self.cards.chars().for_each(|c| {
            let count = h.get(&c).unwrap_or(&0);
            h.insert(c, count + 1);
        });

        h
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn to_val(c: char) -> u32 {
    if c.is_ascii_digit() {
        return c.to_digit(10).unwrap();
    }

    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => panic!("wrong card"),
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_type = self.get_type();
        let other_type = other.get_type();
        if my_type != other_type {
            return my_type.cmp(&other_type);
        }

        for (my, other) in self.cards.chars().zip(other.cards.chars()) {
            if my == other {
                continue;
            }

            return to_val(my).cmp(&to_val(other));
        }

        std::cmp::Ordering::Equal
    }
}

fn process(s: &str) -> u32 {
    let mut hands = s
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(cards, bet)| Hand {
            cards,
            bet: bet.parse().unwrap(),
        })
        .collect::<Vec<_>>();

    hands.sort();

    hands.iter().for_each(|h| println!("{:?}", h));

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bet * ((i as u32) + 1))
        .sum()
}

fn process2(s: &str) -> u32 {
    todo!()
}

static TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 6440)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
