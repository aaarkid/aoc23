use regex::Regex;
use std::collections::HashMap;
use utils::read_file;

fn replace_line(line: &str, word_to_digit: &HashMap<&str, &str>) -> String {
    let mut new_line = String::new();
    
fn main() {
    let mut word_to_digit: HashMap<&str, &str> = HashMap::new();
    word_to_digit.insert("one", "1");
    word_to_digit.insert("two", "2");
    word_to_digit.insert("three", "3");
    word_to_digit.insert("four", "4");
    word_to_digit.insert("five", "5");
    word_to_digit.insert("six", "6");
    word_to_digit.insert("seven", "7");
    word_to_digit.insert("eight", "8");
    word_to_digit.insert("nine", "9");

    let input = read_file("day1");
    let mut sum: u32 = 0;
    for line in input.lines() {

        let mut first: i8 = -1;
        let mut last: u8 = 0;
        for char in line.chars() {
            match char {
                '0'..='9' => {
                    if first == -1 {
                        first = char.to_digit(10).unwrap() as i8;
                        last = first as u8;
                    } else {
                        last = char.to_digit(10).unwrap() as u8;
                    }
                }
                _ => {}
            }
        }
        println!("{}{}", first, last);
        sum += (first as u8 * 10 + last) as u32;
    }

    println!("Sum: {}", sum);
}