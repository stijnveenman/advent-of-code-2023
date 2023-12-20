#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::HashMap;

#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
static TEST_PART1_RESULT: usize = 420;
static TEST_PART2_RESULT: usize = 420;

#[derive(Debug)]
enum Module {
    FlipFlop(String, bool),
    Conjunction(String, HashMap<String, bool>),
    Broadcast(Vec<String>),
}

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn parse(s: &str) -> HashMap<String, Module> {
    s.lines()
        .map(|l| {
            let (module, target) = l.split_once("->").unwrap();

            match module.trim() {
                "broadcaster" => (
                    module.to_string(),
                    Module::Broadcast(target.split(',').map(|s| s.trim().to_string()).collect()),
                ),
                l if l.starts_with('%') => (
                    module[1..].to_string(),
                    Module::FlipFlop(target.trim().to_string(), false),
                ),
                l if l.starts_with('&') => (
                    module[1..].to_string(),
                    Module::Conjunction(target.trim().to_string(), HashMap::new()),
                ),
                _ => panic!("unhandled {}", module),
            }
        })
        .dbg()
        .collect()
}

fn process(s: &str) -> usize {
    let input = parse(s);
    println!("{:?}", input);

    todo!()
}

fn process2(s: &str) -> usize {
    let input = parse(s);
    println!("{:?}", input);

    todo!()
}

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), TEST_PART1_RESULT)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), TEST_PART2_RESULT)
}
