#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::{hash_map, HashMap, HashSet};

use itertools::Itertools;
#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
static TEST_PART1_RESULT: usize = 54;
static TEST_PART2_RESULT: usize = 420;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn parse(s: &str) -> Vec<(&str, Vec<&str>)> {
    s.lines()
        .map(|l| {
            let (l, r) = l.split_once(": ").unwrap();

            (l, r.split(' ').collect_vec())
        })
        .collect_vec()
}

fn build_connections(s: Vec<(&str, Vec<&str>)>) -> HashMap<String, HashSet<String>> {
    let mut m = HashMap::new();

    for l in s {
        let mut set = m.get(l.0).unwrap_or(&HashSet::new()).clone();
        for i in l.1.iter() {
            set.insert(i.to_string());
        }
        m.insert(l.0.to_string(), set);

        for i in l.1.iter() {
            let mut set = m.get(*i).unwrap_or(&HashSet::new()).clone();
            set.insert(l.0.to_string());
            m.insert(i.to_string(), set);
        }
    }

    m
}

fn loop_detect(connections: &HashMap<String, HashSet<String>>, start: &str) -> HashSet<String> {
    let mut open = vec![start];
    let mut visited = HashSet::new();

    while let Some(current) = open.pop() {
        let nbs = connections.get(current).unwrap();

        for n in nbs {
            if visited.contains(n) {
                continue;
            }

            visited.insert(n.to_string());
            open.push(n);
        }
    }
    visited
}

fn cut(connections: &mut HashMap<String, HashSet<String>>, from: &str, to: &str) {
    let hs = connections.get_mut(from).unwrap();
    hs.remove(to);

    let hs = connections.get_mut(to).unwrap();
    hs.remove(from);
}

fn process(s: &str) -> usize {
    let input = parse(s);
    let mut connections = build_connections(input);

    cut(&mut connections, "xvh", "dhn");
    cut(&mut connections, "lxt", "lsv");
    cut(&mut connections, "qmr", "ptj");

    loop_detect(&connections, "qmr").len() * loop_detect(&connections, "ptj").len()
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
