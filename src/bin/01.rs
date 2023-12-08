advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
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

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    use regex::Regex;
    use std::collections::HashMap;
    
    fn replace_line(line: &str, word_to_digit: &HashMap<&str, &str>, re: &Regex) -> String {
        let mut new_line = String::new();
        for key in word_to_digit.keys() {
            new_line = re.replace_all(line, |caps: &regex::Captures| {
                format!("{}{}", word_to_digit[key], &caps[0])
            }).to_string();
        }

        new_line
    }

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

    let pattern = format!("({})", word_to_digit.keys().map(|x| *x).collect::<Vec<&str>>().join("|"));
    let re: Regex = Regex::new(&pattern).unwrap();

    let mut sum: u32 = 0;
    for line in input.lines() {
        let line = replace_line(line, &word_to_digit, &re);
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

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
