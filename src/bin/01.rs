advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for char in line.chars() {
            match char {
                '0'..='9' => {
                    if first == 0 {
                        first = char.to_digit(10).unwrap();
                        last = first;
                    } else {
                        last = char.to_digit(10).unwrap();
                    }
                }
                _ => {}
            }
        }
        sum += first * 10 + last;
    }

    println!("{}", sum);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {

    fn find_word<'a>(line: &str, key: &str, value: &'a str) -> Vec<(usize, &'a str)>  
    {
        let mut positions: Vec<(usize, &str)> = Vec::new();
        if line.contains(key) {
            let mut start = 0;
            while let Some(pos) = line[start..].find(key) {
                let pos = pos + start;
                positions.push((pos, value));
                start = pos + 1;
            }
        }

        positions
    }

    
    fn find_numbers(line: &str, word_to_digit: &Vec<(&str, &str)>) -> String {
        let mut positions: Vec<(usize, &str)> = Vec::new();
        for (key, value) in word_to_digit {
            positions.extend(find_word(line, key, value)); 
        }

        positions.sort_by(|a, b| a.0.cmp(&b.0));

        let mut new_line: String = line.to_string();
        let mut inc = 0; 
        for (pos, value) in positions {
            let pos = pos + inc;
            new_line.insert(pos, value.chars().next().unwrap());
            inc += 1;
        }

        new_line
    }

    let mut word_to_digit: Vec<(&str, &str)> = Vec::new();
    word_to_digit.push(("one", "1"));
    word_to_digit.push(("two", "2"));
    word_to_digit.push(("three", "3"));
    word_to_digit.push(("four", "4"));
    word_to_digit.push(("five", "5"));
    word_to_digit.push(("six", "6"));
    word_to_digit.push(("seven", "7"));
    word_to_digit.push(("eight", "8"));
    word_to_digit.push(("nine", "9"));

    let mut sum: u32 = 0;
    for line in input.lines() {
        let line = find_numbers(line, &word_to_digit);
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for char in line.chars() {
            match char {
                '0'..='9' => {
                    if first == 0 {
                        first = char.to_digit(10).unwrap();
                        last = first;
                    } else {
                        last = char.to_digit(10).unwrap();
                    }
                }
                _ => {}
            }
        }
        sum += first * 10 + last;
    }

    println!("{}", sum);

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY,1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,2));
        assert_eq!(result, Some(281));
    }
}
