#![allow(dead_code)]
#![allow(unused_variables)]

use aoc_toolbox::char_grid::CharGrid;

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
static TEST_PART1_RESULT: usize = 102;
static TEST_PART2_RESULT: usize = 420;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn process(s: &str) -> usize {
    let grid = CharGrid::new(s, |c| c.to_digit(10));
    println!("{:?}", grid.get_xy(0, 1));

    todo!()
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
