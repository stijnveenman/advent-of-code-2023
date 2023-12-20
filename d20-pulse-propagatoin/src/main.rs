#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
static TEST_PART1_RESULT: usize = 11687500;
static TEST_PART2_RESULT: usize = 420;

#[derive(Debug)]
enum Module {
    FlipFlop(Vec<String>, bool),
    Conjunction(Vec<String>, HashMap<String, bool>),
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
            Module::FlipFlop(list, state) => match pulse.pulse {
                true => VecDeque::new(),
                false => {
                    *state = !*state;

                    list.iter()
                        .map(|to| Pulse {
                            pulse: *state,
                            from: pulse.to.to_string(),
                            to: to.to_string(),
                        })
                        .collect()
                }
            },
            Module::Conjunction(list, state) => {
                //this probably doesn't know its inputs yet and will fail on real input
                state.insert(pulse.from, pulse.pulse);
                let p = !state.values().all(|v| *v);

                list.iter()
                    .map(|to| Pulse {
                        pulse: p,
                        from: pulse.to.to_string(),
                        to: to.to_string(),
                    })
                    .collect()
            }
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
}

fn parse(s: &str) -> HashMap<&str, Module> {
    s.lines()
        .map(|l| {
            let (module, target) = l.split_once("->").unwrap();
            let targets = target.split(',').map(|s| s.trim().to_string()).collect();

            match module.trim() {
                "broadcaster" => (module.trim(), Module::Broadcast(targets)),
                l if l.starts_with('%') => (module[1..].trim(), Module::FlipFlop(targets, false)),
                l if l.starts_with('&') => (
                    module[1..].trim(),
                    Module::Conjunction(targets, HashMap::new()),
                ),
                _ => panic!("unhandled {}", module),
            }
        })
        .collect()
}

fn reset(state: &mut HashMap<&str, Module>) {
    for (_, i) in state.iter_mut() {
        match i {
            Module::FlipFlop(_, state) => {
                *state = false;
            }
            Module::Conjunction(_, state) => {
                state.iter_mut().for_each(|(_, s)| *s = false);
            }
            Module::Broadcast(_) => (),
        }
    }
}

fn run(state: &mut HashMap<&str, Module>, count: usize) -> usize {
    let mut high_pulses = 0;
    let mut low_pulses = 0;

    for i in 0..count {
        let mut pulse_queue = VecDeque::new();

        pulse_queue.push_back(Pulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: false,
        });

        while let Some(pulse) = pulse_queue.pop_front() {
            println!("{}", pulse);
            match pulse.pulse {
                true => high_pulses += 1,
                false => low_pulses += 1,
            }
            if let Some(target) = state.get_mut(pulse.to.as_str()) {
                pulse_queue.append(&mut target.send_pulse(pulse));
            };
        }

        println!();
    }

    println!("high: {} low {}", high_pulses, low_pulses);
    low_pulses * high_pulses
}

fn find(state: &mut HashMap<&str, Module>, name: &str) -> usize {
    let mut pulses = 0;

    loop {
        let mut pulse_queue = VecDeque::new();

        pulse_queue.push_back(Pulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: false,
        });
        pulses += 1;

        while let Some(pulse) = pulse_queue.pop_front() {
            if pulse.to == name {
                println!("{}, {}", pulses, pulse)
            }
            if pulse.to == name && !pulse.pulse {
                return pulses;
            }

            if let Some(target) = state.get_mut(pulse.to.as_str()) {
                let mut next = target.send_pulse(pulse);
                pulse_queue.append(&mut next);
            };
        }
    }
}

fn process(s: &str) -> usize {
    let mut state = parse(s);

    run(&mut state, 1000);
    reset(&mut state);

    run(&mut state, 1000)
}

fn process2(s: &str) -> usize {
    let state = parse(s);

    let mut state = parse(s);

    //todo fix this properly
    run(&mut state, 1000);
    reset(&mut state);

    find(&mut state, "rx")
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
