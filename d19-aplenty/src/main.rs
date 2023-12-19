#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::collections::HashMap;

#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
static TEST_PART1_RESULT: isize = 19114;
static TEST_PART2_RESULT: isize = 420;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

#[derive(Debug)]
struct WorkflowStep {
    item: char,
    action: char,
    value: isize,
    next: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    steps: Vec<WorkflowStep>,
    otherwise: String,
}

type Item = HashMap<char, isize>;

fn parse(s: &str) -> (HashMap<String, Workflow>, Vec<Item>) {
    let (workflows, items) = s.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|l| {
            let (name, steps_s) = l.split_once('{').unwrap();

            let steps = steps_s
                .split(',')
                .filter(|s| s.contains(':'))
                .map(|s| WorkflowStep {
                    item: s.chars().next().unwrap(),
                    action: s.chars().nth(1).unwrap(),
                    value: s[2..].split_once(':').unwrap().0.parse().unwrap(),
                    next: s.split_once(':').unwrap().1.to_string(),
                })
                .collect();
            let otherwise = steps_s
                .split(',')
                .last()
                .unwrap()
                .to_string()
                .replace('}', "");

            Workflow {
                steps,
                otherwise,
                name: name.to_string(),
            }
        })
        .map(|w| (w.name.to_string(), w))
        .collect();

    let items = items
        .lines()
        .map(|l| {
            let l = &l[1..l.len() - 1];
            l.split(',')
                .map(|i| i.split_once('=').unwrap())
                .map(|i| (i.0.chars().next().unwrap(), i.1.parse().unwrap()))
                .collect()
        })
        .collect();

    (workflows, items)
}

fn run_workflows(workflows: &HashMap<String, Workflow>, item: &Item) -> bool {
    let mut current = workflows.get("in").unwrap();

    loop {
        println!("{:?}", current);
        for step in current.steps.iter() {
            let val = *item.get(&step.item).unwrap();

            let matching = match step.action {
                '<' => val < step.value,
                '>' => val > step.value,
                a => panic!("unknown action {}", a),
            };

            if !matching {
                continue;
            }

            match step.next.as_str() {
                "A" => return true,
                "R" => return false,

                next => current = workflows.get(next).unwrap(),
            }
        }

        match current.otherwise.as_str() {
            "A" => return true,
            "R" => return false,

            next => current = workflows.get(next).unwrap(),
        }
    }
}

fn process(s: &str) -> isize {
    let (workflows, items) = parse(s);

    items
        .iter()
        .filter(|i| run_workflows(&workflows, i))
        .map(|i| i.values().sum::<isize>())
        .sum()
}

fn process2(s: &str) -> isize {
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
