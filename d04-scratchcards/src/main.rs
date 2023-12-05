#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
}

#[derive(Debug)]
struct ScratchCard {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromStr for ScratchCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card, numbers) = s.split_once(':').unwrap();
        let (_, card) = card.split_once(' ').unwrap();

        let (winnig, numbers) = numbers.split_once('|').unwrap();

        let winning = winnig
            .trim()
            .replace("  ", " ")
            .split(' ')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let numbers = numbers
            .trim()
            .replace("  ", " ")
            .split(' ')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        Ok(ScratchCard {
            id: card.trim().parse().unwrap(),
            winning,
            numbers,
        })
    }
}

impl ScratchCard {
    fn get_matches(&self) -> u32 {
        let mut winning = HashSet::new();
        self.winning.iter().for_each(|n| {
            winning.insert(n);
        });

        self.numbers.iter().filter(|n| winning.contains(n)).count() as u32
    }

    fn get_worth(&self) -> u32 {
        let count = self.get_matches();
        if count == 0 {
            return 0;
        }
        2u32.pow(count - 1)
    }
}

fn process(s: &str) -> u32 {
    let cards = s
        .lines()
        .map(|l| l.parse::<ScratchCard>().unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", cards);

    cards.iter().map(|c| c.get_worth()).sum()
}

fn process2(s: &str) -> u32 {
    let cards = s
        .lines()
        .map(|l| l.parse::<ScratchCard>().unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", cards);

    let mut hm = HashMap::new();

    for card in cards.iter() {
        let count = *hm.get(&card.id).unwrap_or(&1);

        let matches = card.get_matches();
        for i in card.id + 1..=card.id + matches {
            hm.insert(i, *hm.get(&i).unwrap_or(&1) + count);
        }
    }

    cards.iter().map(|c| hm.get(&c.id).unwrap_or(&1)).sum()
}

static TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 13)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 30)
}
