#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, hash::Hash};

use aoc_toolbox::{char_grid::CharGrid, point::Point};
use itertools::Itertools;

static TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
static TEST_INPUT2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";
static TEST_PART1_RESULT: u32 = 102;
static TEST_PART2_RESULT: usize = 420;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(TEST_INPUT2))
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Node {
    point: Point,
    direction: Point,
    steps: u32,
}

impl Node {
    fn new(point: Point, direction: Point, steps: u32) -> Node {
        Node {
            point,
            direction,
            steps,
        }
    }

    fn neighbours(&self) -> Vec<Node> {
        if self.steps < 4 {
            return vec![Node::new(
                self.point + self.direction,
                self.direction,
                self.steps + 1,
            )];
        }

        let mut v = vec![
            Node::new(
                self.point + self.direction.rotate(1),
                self.direction.rotate(1),
                1,
            ),
            Node::new(
                self.point + self.direction.rotate(-1),
                self.direction.rotate(-1),
                1,
            ),
        ];

        if self.steps < 10 {
            v.push(Node::new(
                self.point + self.direction,
                self.direction,
                self.steps + 1,
            ))
        }

        v
    }
}

fn process(s: &str) -> u32 {
    let grid = CharGrid::new(s, |c| c.to_digit(10));

    let mut open_set: HashMap<Node, u32> = HashMap::new();
    let mut closed_set: HashMap<Node, u32> = HashMap::new();

    open_set.insert(Node::new(Point::new(0, 0), Point::RIGHT, 1), 0);
    open_set.insert(Node::new(Point::new(0, 0), Point::DOWN, 1), 0);
    let goal = grid.upper();

    loop {
        let Some(s) = open_set.iter().sorted_by_key(|x| x.1).next() else {
            panic!("didnt find goal");
        };

        let current = *s.0;
        let value = *s.1;
        //println!("exploring {:?} {}", current, value);

        for neighbour in current
            .neighbours()
            .into_iter()
            .filter(|p| grid.is_within(&p.point))
        {
            let Some(val) = grid.get(&neighbour.point) else {
                println!("failed to get point {:?}", neighbour);
                continue;
            };

            let neighbour_val = value + val;

            if let Some(cur_val) = closed_set.get(&neighbour) {
                if *cur_val <= neighbour_val {
                    continue;
                }
            }

            if let Some(cur_val) = open_set.get(&neighbour) {
                if *cur_val <= neighbour_val {
                    continue;
                }
            }

            open_set.insert(neighbour, value + val);
        }

        if current.point == goal && current.steps >= 4 {
            println!("found goal {} {:?}", value, current.steps);
            return value;
        }

        open_set.remove(&current);
        closed_set.insert(current, value);
    }
}

fn process2(s: &str) -> usize {
    println!("{:?}", s);

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
