use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    println!("{}", process(input))
}

fn replace_line(s: &str) -> String {
    let mut s = s.to_string();

    s = s.replace("one", "1");
    s = s.replace("two", "2");
    s = s.replace("three", "3");
    s = s.replace("four", "4");
    s = s.replace("five", "5");
    s = s.replace("six", "6");
    s = s.replace("seven", "7");
    s = s.replace("eight", "8");
    s = s.replace("nine", "9");

    s
}

fn process_line(s: &str) -> u32 {
    print!("{} ", s);
    let first = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine).*")
        .unwrap()
        .captures(s)
        .unwrap()
        .get(1)
        .unwrap();
    let first = replace_line(first.as_str());

    let last = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)")
        .unwrap()
        .captures(s)
        .unwrap()
        .get(1)
        .unwrap();
    let last = replace_line(last.as_str());

    let number = format!("{}{}", first, last);

    println!("{}", number);
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

#[test]
fn test_input2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    let result = process(input);

    assert_eq!(dbg!(result), 281)
}
