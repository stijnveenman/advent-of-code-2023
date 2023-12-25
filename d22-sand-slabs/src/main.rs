#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
static TEST_PART1_RESULT: usize = 5;
static TEST_PART2_RESULT: usize = 7;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Vec3 {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Vec3 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');

        Ok(Self::new(
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        ))
    }
}

impl Vec3 {
    fn new(x: usize, y: usize, z: usize) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn with_z(&self, z: usize) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Brick {
    start: Vec3,
    end: Vec3,
}

impl Brick {
    fn new(start: Vec3, end: Vec3) -> Brick {
        Brick { start, end }
    }

    fn min_z(&self) -> usize {
        self.start.z.min(self.end.z)
    }

    fn max_z(&self) -> usize {
        self.start.z.max(self.end.z)
    }

    fn has_z(&self, z: usize) -> bool {
        (self.start.z..=self.end.z).contains(&z)
    }

    fn points(&self) -> Vec<Vec3> {
        (self.start.x..=self.end.x)
            .flat_map(|x| (self.start.y..=self.end.y).map(move |y| Vec3::new(x, y, 0)))
            .collect_vec()
    }

    fn collides_with(&self, other: &Brick) -> bool {
        let mine = self.points();
        let others = other.points();

        mine.iter().any(|p| others.contains(p))
    }

    fn with_min_z(&self, z: usize) -> Brick {
        let dz = self.end.z - self.start.z;

        Brick::new(self.start.with_z(z), self.end.with_z(z + dz))
    }
}

fn parse(s: &str) -> Vec<Brick> {
    s.lines()
        .map(|l| {
            let (start, end) = l.split_once('~').unwrap();
            Brick::new(start.parse().unwrap(), end.parse().unwrap())
        })
        .collect_vec()
}

fn settle(mut bricks: Vec<Brick>) -> Vec<Brick> {
    bricks.sort_by_key(|b| b.min_z());
    let mut settled = vec![];

    for brick in bricks {
        if brick.min_z() == 1 {
            settled.push(brick);
            continue;
        }

        let mut to_z = 1;
        for z in (1..brick.min_z()).rev() {
            if settled
                .iter()
                .any(|other| other.has_z(z) && brick.collides_with(other))
            {
                to_z = z + 1;
                break;
            }
        }

        //settled.iter().for_each(|s| println!("set: {:?}", s));
        //println!("{:?} found to_z {}", brick, to_z);
        //println!("placing {:?}\n", brick.with_min_z(to_z));

        settled.push(brick.with_min_z(to_z));
    }

    settled
}

// hahsmap per brick what bricks support it
fn find_supporting_bricks(bricks: &[Brick]) -> HashMap<&Brick, HashSet<&Brick>> {
    let mut hm = HashMap::new();

    for brick in bricks {
        let at_z = brick.min_z() - 1;

        if !hm.contains_key(brick) {
            hm.insert(brick, HashSet::new());
        }
        let hs = hm.get_mut(brick).unwrap();

        for other in bricks {
            if other == brick || !other.has_z(at_z) {
                continue;
            }

            //check collision
            if !brick.collides_with(other) {
                continue;
            }

            //if collusion, add it to a HashSet
            hs.insert(other);
        }
    }

    hm
}

fn process(s: &str) -> usize {
    let input = parse(s);
    let bricks = settle(input);

    let supporting = find_supporting_bricks(&bricks);

    bricks
        .iter()
        .filter(|brick| {
            supporting
                .iter()
                .all(|other| !other.1.contains(brick) || other.1.len() > 1)
        })
        .count()
}

fn recursive_supporting<'a>(
    supporting: &'a HashMap<&'a Brick, HashSet<&'a Brick>>,
    brick: &'a Brick,
) -> HashSet<&'a Brick> {
    let mut removed = HashSet::new();
    removed.insert(brick);

    loop {
        let freefalling = supporting
            .iter()
            .filter(|b| !b.1.is_empty())
            .filter(|b| b.1.iter().all(|brick| removed.contains(brick)))
            .filter(|b| !removed.contains(b.0))
            .collect_vec();

        if freefalling.is_empty() {
            break;
        }

        for n in freefalling {
            removed.insert(n.0);
        }
    }

    removed
}

fn process2(s: &str) -> usize {
    let input = parse(s);
    let bricks = settle(input);

    let supporting = find_supporting_bricks(&bricks);

    let f = bricks.last().unwrap();

    bricks
        .iter()
        .map(|brick| recursive_supporting(&supporting, brick).len() - 1)
        .sum()
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
