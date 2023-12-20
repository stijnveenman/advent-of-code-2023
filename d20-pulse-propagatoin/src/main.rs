#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

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

struct Pulse {
    pulse: bool,
    from: String,
    to: String,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -{}-> {}",
            self.from,
            match self.pulse {
                true => "high",
                false => "low",
            },
            self.to
        )
    }
}

impl Module {
    fn send_pulse(&mut self, pulse: Pulse) -> VecDeque<Pulse> {
        match self {
            Module::Broadcast(list) => list
                .iter()
                .map(|to| Pulse {
                    pulse: pulse.pulse,
                    from: pulse.to.to_string(),
                    to: to.to_string(),
                })
                .collect(),
            Module::FlipFlop(to, state) => match pulse.pulse {
                true => VecDeque::new(),
                false => {
                    *state = !*state;

                    let mut v = VecDeque::new();
                    v.push_back(Pulse {
                        from: pulse.to,
                        to: to.to_string(),
                        pulse: *state,
                    });
                    v
                }
            },
            Module::Conjunction(to, state) => {
                //this probably doesn't know its inputs yet and will fail on real input
                state.insert(pulse.from, pulse.pulse);
                let p = !state.values().all(|v| *v);

                let mut v = VecDeque::new();
                v.push_back(Pulse {
                    from: pulse.to,
                    to: to.to_string(),
                    pulse: p,
                });
                v
            }
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn parse(s: &str) -> HashMap<&str, Module> {
    s.lines()
        .map(|l| {
            let (module, target) = l.split_once("->").unwrap();

            match module.trim() {
                "broadcaster" => (
                    module.trim(),
                    Module::Broadcast(target.split(',').map(|s| s.trim().to_string()).collect()),
                ),
                l if l.starts_with('%') => (
                    module[1..].trim(),
                    Module::FlipFlop(target.trim().to_string(), false),
                ),
                l if l.starts_with('&') => (
                    module[1..].trim(),
                    Module::Conjunction(target.trim().to_string(), HashMap::new()),
                ),
                _ => panic!("unhandled {}", module),
            }
        })
        .collect()
}

fn process(s: &str) -> usize {
    let mut state = parse(s);
    let mut pulse_queue = VecDeque::new();

    pulse_queue.push_back(Pulse {
        from: "button".to_string(),
        to: "broadcaster".to_string(),
        pulse: false,
    });

    while let Some(pulse) = pulse_queue.pop_front() {
        println!("{}", pulse);
        let target = state.get_mut(pulse.to.as_str()).unwrap();
        pulse_queue.append(&mut target.send_pulse(pulse));
    }

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
