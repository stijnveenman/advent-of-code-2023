#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
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
    let r = Regex::new(r"\d+").unwrap();
    let mut v = vec![];

    for (i, current) in s.lines().enumerate() {
        for m in r.captures_iter(current) {
            let m = m.get(0).unwrap();

            v.push((i, m));
        }
    }

    let r = Regex::new(r"\*").unwrap();
    let mut sum = 0;
    for (i, current) in s.lines().enumerate() {
        for m in r.captures_iter(current) {
            let m = m.get(0).unwrap();

            let numbers = v
                .iter()
                .filter(|n| {
                    let r = n.1.start()..=n.1.end();
                    let vr = i.checked_sub(1).unwrap_or(i)..=(i + 1);
                    vr.contains(&n.0) && (r.contains(&m.start()) || r.contains(&m.end()))
                })
                .collect::<Vec<_>>();

            if numbers.len() != 2 {
                continue;
            }

            sum += numbers
                .into_iter()
                .map(|n| n.1.as_str().parse::<u32>().unwrap())
                .product::<u32>();
        }
    }

    sum
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

    assert_eq!(dbg!(result), 467835)
}
