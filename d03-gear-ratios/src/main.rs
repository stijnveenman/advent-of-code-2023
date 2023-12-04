#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn is_not_maching(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit() || c == '.')
}

fn process(s: &str) -> u32 {
    let mut iter = s.lines().peekable();

    let mut prev = None;
    let r = Regex::new(r"\d+").unwrap();
    let mut v = vec![];

    while let Some(current) = iter.next() {
        let next = iter.peek();

        for m in r.captures_iter(current) {
            let m = m.get(0).unwrap();

            let start = m.start().saturating_sub(1);
            let end = current.len().min(m.end() + 1);

            if is_not_maching(&current[start..end])
                && next
                    .map(|next| is_not_maching(&next[start..end]))
                    .unwrap_or(true)
                && prev
                    .map(|prev: &str| is_not_maching(&prev[start..end]))
                    .unwrap_or(true)
            {
                continue;
            }

            v.push(m.as_str().to_string());
        }

        prev = Some(current);
    }

    println!("{:?}", v);
    v.iter().map(|i| i.parse::<u32>().unwrap()).sum()
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

    assert_eq!(dbg!(result), 4361)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 0)
}
