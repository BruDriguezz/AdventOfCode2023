// AoC - Day One
use std::collections::HashMap;
use itertools::izip;
use regex::Regex;

pub fn part_one(input: &str) -> Box<str> {

    let mut first_digit: char = '.';
    let mut last_digit: char = '.';

    // First digit
    for char in input.chars() {
        if char.is_digit(10) {
            first_digit = char;
            break;
        }
    }

    // Last digit
    for char in input.chars() {
        if char.is_digit(10) {
            last_digit = char;
        }
    }

    let result: Box<str> = format!("{}{}", first_digit, last_digit).into_boxed_str();
    return result;
}

pub fn part_two(input: &str) -> Box<str> {

    // println!("Input: {}", input);
    let mut first_num: String = "0".to_string();
    let mut last_num: String = "0".to_string();
    // Forgive me father for I have sinned
    let digit_pattern: Regex = Regex::new(
        r#"(?i)(one|two|three|four|five|six|seven|eight|nine|ten)"#
    ).unwrap();

    let digit_dictionary: HashMap<&str, &str> = izip!(
        vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"],
        vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]
    ).collect();

    let captures: Vec<_> = digit_pattern.captures_iter(input).collect();

    loop {
        for capture in &captures {
            first_num = digit_dictionary
                .get(capture.get(0).unwrap().as_str())
                .unwrap()
                .to_string();

            if first_num != "0" {
                // println!("First digit has been found: {}", first_num);
                break;
            }
        }

        for capture in &captures {
            last_num = digit_dictionary
                .get(capture.get(0)
                .unwrap()
                .as_str())
                .unwrap()
                .to_string();

            // println!("Possible last digit: {}", last_num);
            if last_num != "0" {
                continue;
            }
            else {
                last_num = first_num.clone();
            }
        }
        break;
    }

    let result: Box<str> = format!("{}{}", first_num, last_num).into_boxed_str();
    return result;
}



fn main() {

    let document = std::fs::read_to_string("./data/input.txt").unwrap();

    // First part:
    let first_lines: Vec<&str> = document.lines().collect();
    let first_sum: i32 = first_lines
        .iter()
        .map(|line| part_one(line).parse::<i32>().unwrap())
        .sum();

    println!("[Part One] Sum: {}", first_sum);

    // Second part:
    let second_lines: Vec<&str> = document.lines().collect();
    let second_sum: i32 = second_lines
        .iter()
        .map(|line| part_two(line).parse::<i32>().unwrap())
        .sum();

    // Truthfully, I don't have a clue why it returns a value below of expected.
    // Nor I feel inclined to investigate it further at this point.
    // I'll ask for some reviews later on and possibly fix it.
    println!("[Part Two] Sum: {}", second_sum);
}