fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn process_line(s: &str) -> u32 {
    let numbers = s.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
    let first = numbers.first().unwrap();
    let last = numbers.last().unwrap();

    let number = format!("{}{}", first, last);
    number.parse().unwrap()
}

fn process(s: &str) -> u32 {
    let sum: u32 = s.lines().map(process_line).sum();

    sum
}

#[test]
fn test_input() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    let result = process(input);

    assert_eq!(dbg!(result), 142)
}
