#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::HashMap;

#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
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

fn process2(s: &str) -> usize {
    let input = parse(s);
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    for i in 0..=255 {
        boxes.insert(i, vec![]);
    }

    for operation in input {
        if operation.contains('-') {
            let (lens, _) = operation.split_once('-').unwrap();
            let i = hash(lens);
            if let Some(b) = boxes.get_mut(&i) {
                if let Some(idx) = b.iter().enumerate().find(|l| l.1 .0 == lens) {
                    b.remove(idx.0);
                }
            }
        } else {
            let (lens, vocal) = operation.split_once('=').unwrap();
            let i = hash(lens);
            let vocal: usize = vocal.parse().unwrap();
            if let Some(b) = boxes.get_mut(&i) {
                if let Some(idx) = b.iter().enumerate().find(|l| l.1 .0 == lens).map(|l| l.0) {
                    b[idx] = (lens, vocal);
                } else {
                    b.push((lens, vocal))
                }
            }
        }
    }

    boxes
        .iter()
        .dbg()
        .map(|(box_index, b)| {
            b.iter()
                .enumerate()
                .map(|(lens_index, (_, vocal))| {
                    println!("{:?}", (box_index, lens_index, vocal));
                    (box_index + 1) * (lens_index + 1) * vocal
                })
                .sum::<usize>()
        })
        .sum()
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

    assert_eq!(dbg!(result), 145)
}
