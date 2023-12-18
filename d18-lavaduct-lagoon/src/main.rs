#![allow(dead_code)]
#![allow(unused_variables)]
mod util;

use aoc_toolbox::{char_grid::CharGrid, point::Point, shoelace::shoelace};
#[allow(unused_imports)]
use util::*;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process2(input))
}

fn parse(s: &str) -> Vec<LineItem> {
    s.lines()
        .map(|l| {
            let mut i = l.split(' ');
            LineItem {
                dir: i.next().unwrap().chars().next().unwrap(),
                count: i.next().unwrap().parse().unwrap(),
                hex: i.next().unwrap().replace(['(', ')'], ""),
            }
        })
        .collect()
}

fn parse2(s: &str) -> Vec<LineItem> {
    s.lines()
        .map(|l| {
            let mut i = l.split(' ');
            LineItem {
                dir: i.next().unwrap().chars().next().unwrap(),
                count: i.next().unwrap().parse().unwrap(),
                hex: i.next().unwrap().replace(['(', ')'], ""),
            }
        })
        .map(|item| {
            let dir = match &item.hex[item.hex.len() - 1..] {
                "0" => 'R',
                "1" => 'D',
                "2" => 'L',
                "3" => 'U',
                _ => panic!("dir"),
            };

            let hex = isize::from_str_radix(&item.hex[1..item.hex.len() - 1], 16).unwrap();

            LineItem {
                dir,
                count: hex,
                hex: item.hex,
            }
        })
        .collect()
}

#[derive(Debug)]
struct LineItem {
    dir: char,
    count: isize,
    hex: String,
}

impl LineItem {
    fn steps(&self, p: &Point) -> (Vec<Point>, Point) {
        match self.dir {
            'R' => {
                let v = (p.x..=p.x + self.count)
                    .map(|x| Point::new(x, p.y))
                    .collect();
                let n = Point::new(p.x + self.count, p.y);
                (v, n)
            }
            'L' => {
                let v = (p.x - self.count..=p.x)
                    .map(|x| Point::new(x, p.y))
                    .collect();
                let n = Point::new(p.x - self.count, p.y);
                (v, n)
            }
            'D' => {
                let v = (p.y..=p.y + self.count)
                    .map(|y| Point::new(p.x, y))
                    .collect();
                let n = Point::new(p.x, p.y + self.count);
                (v, n)
            }
            'U' => {
                let v = (p.y - self.count..=p.y)
                    .map(|y| Point::new(p.x, y))
                    .collect();
                let n = Point::new(p.x, p.y - self.count);
                (v, n)
            }

            _ => panic!("shouldnt reach"),
        }
    }

    fn next(&self, p: &Point) -> (Point, char) {
        match self.dir {
            'R' => (Point::new(p.x + self.count, p.y), self.dir),
            'L' => (Point::new(p.x - self.count, p.y), self.dir),
            'U' => (Point::new(p.x, p.y - self.count), self.dir),
            'D' => (Point::new(p.x, p.y + self.count), self.dir),
            _ => panic!(),
        }
    }
}

fn find_inside<T>(grid: &CharGrid<T>) -> Option<Point> {
    let upper = grid.upper();

    for y in 0..=upper.y {
        let mut crossings = 0;
        for x in 0..upper.x {
            match grid.get_xy(x, y) {
                Some(_) => {
                    crossings += 1;
                    if crossings > 1 {
                        break;
                    }
                }
                None => {
                    if crossings == 1 {
                        return Some(Point::new(x, y));
                    }
                }
            }
        }
    }

    None
}

fn process(s: &str) -> isize {
    let input = parse(s);

    let mut current = Point::new(0, 0);
    let mut points = vec![];

    let len = input.iter().map(|x| x.count).sum::<isize>();
    println!("{:?}", len);
    for item in input {
        let (next, dir) = item.next(&current);

        points.push(current);

        current = next;
    }

    println!("{:?}", points);

    shoelace(&points) + (len / 2) + 1
}

fn process2(s: &str) -> isize {
    let input = parse2(s);
    let mut current = Point::new(0, 0);
    let mut points = vec![];

    let len = input.iter().map(|x| x.count).sum::<isize>();
    println!("{:?}", len);
    for item in input {
        let (next, dir) = item.next(&current);

        points.push(current);

        current = next;
    }

    println!("{:?}", points);

    shoelace(&points) + (len / 2) + 1
}

static TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[test]
fn test_part1() {
    let result = process(TEST_INPUT);

    assert_eq!(dbg!(result), 62)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), 952408144115)
}
