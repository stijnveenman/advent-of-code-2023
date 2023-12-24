#![allow(dead_code)]
#![allow(unused_variables)]
mod util;
use itertools::Itertools;
#[allow(unused_imports)]
use util::*;

static TEST_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
static TEST_PART1_RESULT: usize = 2;
static TEST_PART2_RESULT: usize = 47;

#[derive(PartialEq, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Line {
    pos: Vec3,
    vel: Vec3,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn from_str(s: &str) -> Vec3 {
        let mut iter = s.split(',');
        Vec3 {
            x: iter.next().unwrap().trim().parse().unwrap(),
            y: iter.next().unwrap().trim().parse().unwrap(),
            z: iter.next().unwrap().trim().parse().unwrap(),
        }
    }
}

impl Line {
    fn new(pos: Vec3, vel: Vec3) -> Line {
        Line { pos, vel }
    }

    fn dx(&self) -> f64 {
        let x: f64 = self.vel.x;
        let y: f64 = self.vel.y;

        y / x
    }

    fn y0(&self) -> f64 {
        self.pos.y - (self.pos.x * self.dx())
    }

    fn at_x(&self, x: f64) -> f64 {
        self.y0() + (self.dx() * x)
    }

    fn is_past(&self, v: &Vec3) -> bool {
        let dx = v.x - self.pos.x;
        if dx > 0.0 && self.vel.x < 0.0 {
            return true;
        }
        if dx < 0.0 && self.vel.x > 0.0 {
            return true;
        }

        let dy = v.y - self.pos.y;
        if dy < 0.0 && self.vel.y > 0.0 {
            return true;
        }
        if dy > 0.0 && self.vel.y < 0.0 {
            return true;
        }
        false
    }
}

fn main() {
    let input = include_str!("./input.txt");

    //println!("{}", process(input, 200000000000000.0, 400000000000000.0))
    println!("{}", process2(input))
}

fn is_parralel(a: &Line, b: &Line) -> bool {
    a.vel.x / b.vel.x == a.vel.y / b.vel.y && a.vel.z / b.vel.z == a.vel.y / b.vel.y
}

fn intersect(a: &Line, b: &Line) -> Option<Vec3> {
    // a_y0 + a_dx * x = b_y0 + b_dx * x
    // a_dx * x - b_dx * x = b_y0 - a_y0
    // x = (b_y0 - a_y0) / (a_dx - b_dx)

    let x = (b.y0() - a.y0()) / (a.dx() - b.dx());

    if x.is_infinite() {
        return None;
    }

    let y = a.at_x(x);

    let v = Vec3 {
        x: (x * 10.0).round() / 10.0,
        y: (y * 10.0).round() / 10.0,
        z: 0.0,
    };

    if a.is_past(&v) || b.is_past(&v) {
        return None;
    }
    println!("{:?}:{:?} - {:?}", a.pos, b.pos, (x, y));

    Some(v)
}

fn parse(s: &str) -> Vec<Line> {
    s.lines()
        .map(|l| {
            let mut iter = l.split('@');
            Line::new(
                Vec3::from_str(iter.next().unwrap()),
                Vec3::from_str(iter.next().unwrap()),
            )
        })
        .collect_vec()
}

fn process(s: &str, min: f64, max: f64) -> usize {
    let input = parse(s);
    input
        .iter()
        .combinations(2)
        .filter_map(|v| {
            let a = v.first().unwrap();
            let b = v.last().unwrap();

            intersect(a, b)
        })
        .filter(|v| (min..max).contains(&v.x) && (min..max).contains(&v.y))
        .count()
}

//public (bool intersects, (BigInteger X, BigInteger Y) pos, BigInteger time) TryIntersectPos(Hail one, Hail two, (int x, int y) offset)
fn intersect3d(one: &Line, two: &Line, offset_x: f64, offset_y: f64) -> Option<(Vec3, f64)> {
    //var a = (Pos: (one.Pos.X, one.Pos.Y), Vel: (X: one.Vel.X + offset.x, Y: one.Vel.Y + offset.y));
    let a = Line::new(
        Vec3::new(one.pos.x, one.pos.y, 0.0),
        Vec3::new(one.vel.x + offset_x, one.vel.y + offset_y, 0.0),
    );
    let c = Line::new(
        Vec3::new(two.pos.x, two.pos.y, 0.0),
        Vec3::new(two.vel.x + offset_x, two.vel.y + offset_y, 0.0),
    );

    let d = (a.vel.x * -1.0 * c.vel.y) - (a.vel.y * -1.0 * c.vel.x);

    if d == 0.0 {
        return None;
    }

    let qx = (-1.0 * c.vel.y * (c.pos.x - a.pos.x)) - (-1.0 * c.vel.x * (c.pos.y - a.pos.y));
    let qy = (a.vel.x * (c.pos.y - a.pos.y)) - (a.vel.y * (c.pos.x - a.pos.x));

    let t = qx / d;
    let s = qy / d;

    let px = a.pos.x + t * a.vel.x;
    let py = a.pos.y + t * a.vel.y;

    Some((Vec3::new(px, py, 0.0), t))
}

fn process2(s: &str) -> usize {
    let input = parse(s);
    let r = 3000;

    let mut iter = input.iter();
    let l0 = iter.next().unwrap();
    let l1 = iter.next().unwrap();
    let l2 = iter.next().unwrap();
    let l3 = iter.next().unwrap();

    for x in -r..r {
        let x = f64::try_from(x).unwrap();
        for y in -r..r {
            let y = f64::try_from(y).unwrap();

            let Some(intersect1) = intersect3d(l1, l0, x, y) else {
                continue;
            };
            let Some(intersect2) = intersect3d(l2, l0, x, y) else {
                continue;
            };
            let Some(intersect3) = intersect3d(l3, l0, x, y) else {
                continue;
            };

            if intersect1.0 != intersect2.0 || intersect1.0 != intersect3.0 {
                continue;
            }

            println!("found {:?} trying z", (x, y));
            for z in -r..r {
                let z = f64::try_from(z).unwrap();

                let intersect_z = l1.pos.z + intersect1.1 * (l1.vel.z + z);
                let intersect_z2 = l2.pos.z + intersect2.1 * (l2.vel.z + z);
                let intersect_z3 = l3.pos.z + intersect3.1 * (l3.vel.z + z);

                if intersect_z != intersect_z2 || intersect_z != intersect_z3 {
                    continue;
                }

                return (intersect1.0.x + intersect1.0.y + intersect_z).round() as usize;
            }
        }
    }

    println!("ran out");
    todo!()
}

#[test]
fn test_part1() {
    let result = process(TEST_INPUT, 7.0, 27.0);

    assert_eq!(dbg!(result), TEST_PART1_RESULT)
}

#[test]
fn test_part2() {
    let result = process2(TEST_INPUT);

    assert_eq!(dbg!(result), TEST_PART2_RESULT)
}

#[test]
fn p_1() {
    let a = Line::new(Vec3::new(19.0, 13.0, 30.0), Vec3::new(-2.0, 1.0, -2.0));
    let b = Line::new(Vec3::new(18.0, 19.0, 22.0), Vec3::new(-1.0, -1.0, -2.0));

    assert_eq!(intersect(&a, &b).unwrap(), Vec3::new(14.3, 15.3, 0.0))
}

#[test]
fn p_2() {
    let a = Line::new(Vec3::new(19.0, 13.0, 30.0), Vec3::new(-2.0, 1.0, -2.0));
    let b = Line::new(Vec3::new(20.0, 19.0, 15.0), Vec3::new(1.0, -5.0, -3.0));

    assert_eq!(intersect(&a, &b), None)
}
