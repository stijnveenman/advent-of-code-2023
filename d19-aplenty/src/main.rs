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
static TEST_PART2_RESULT: isize = 167409079868000;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
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

fn run_workflows(workflows: &HashMap<String, Workflow>, item: &Item, name: &str) -> bool {
    let current = workflows.get(name).unwrap();

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

            next => return run_workflows(workflows, item, next),
        }
    }

    match current.otherwise.as_str() {
        "A" => true,
        "R" => false,

        next => run_workflows(workflows, item, next),
    }
}

fn process(s: &str) -> isize {
    let (workflows, items) = parse(s);

    items
        .iter()
        .filter(|i| run_workflows(&workflows, i, "in"))
        .map(|i| i.values().sum::<isize>())
        .sum()
}

#[derive(Debug, Copy, Clone)]
struct Range {
    min: isize,
    max: isize,
}

impl Range {
    fn lower_then(&self, i: isize) -> Option<Range> {
        if self.min > i {
            println!("return none");
            return None;
        }
        Some(Range {
            min: self.min.min(i),
            max: self.max.min(i),
        })
    }
    fn higher_then(&self, i: isize) -> Option<Range> {
        if self.max < i {
            println!("return none");
            return None;
        }
        Some(Range {
            min: self.min.max(i),
            max: self.max.max(i),
        })
    }
}

#[derive(Debug, Clone)]
struct RangeItem {
    items: HashMap<char, Range>,
}

impl RangeItem {
    pub fn product(&self) -> isize {
        self.items.values().map(|v| v.max - v.min + 1).product()
    }
}

fn calculate_range(
    workflows: &HashMap<String, Workflow>,
    item: RangeItem,
    name: &str,
    step_index: usize,
) -> Vec<RangeItem> {
    let current = workflows.get(name).unwrap();

    let Some(step) = current.steps.get(step_index) else {
        match current.otherwise.as_str() {
            "A" => return vec![item],
            "R" => return vec![],

            next => return calculate_range(workflows, item, next, 0),
        }
    };

    let val = *item.items.get(&step.item).unwrap();

    let (matching, notmatching) = match step.action {
        '<' => (val.lower_then(step.value - 1), val.higher_then(step.value)),
        '>' => (val.higher_then(step.value + 1), val.lower_then(step.value)),
        a => panic!("unknown action {}", a),
    };

    println!("{:?} {:?}", matching, step);

    let mut v = vec![];
    if let Some(matching) = matching {
        let mut next_item = item.clone();
        next_item.items.insert(step.item, matching);

        match step.next.as_str() {
            "A" => {
                v.push(next_item);
            }
            "R" => (),

            next => {
                v.append(&mut calculate_range(workflows, next_item, next, 0));
            }
        }
    }

    if let Some(notmatching) = notmatching {
        let mut next_item = item.clone();
        next_item.items.insert(step.item, notmatching);

        v.append(&mut calculate_range(
            workflows,
            next_item,
            name,
            step_index + 1,
        ));
    }

    v
}

fn process2(s: &str) -> isize {
    let (workflows, _) = parse(s);

    let mut a = RangeItem {
        items: HashMap::new(),
    };
    a.items.insert('x', Range { min: 1, max: 4000 });
    a.items.insert('m', Range { min: 1, max: 4000 });
    a.items.insert('a', Range { min: 1, max: 4000 });
    a.items.insert('s', Range { min: 1, max: 4000 });

    let result = calculate_range(&workflows, a, "in", 0);
    println!("{:?}", result);

    result.iter().map(|i| i.product()).sum()
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
