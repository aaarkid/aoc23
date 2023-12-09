advent_of_code::solution!(2);
use regex::Regex;

#[derive(PartialEq, PartialOrd, Debug)]
struct GrabBag {
    red: u32,
    green: u32,
    blue: u32,
}

const MAX_GRAB: GrabBag = GrabBag {
    red: 12,
    green: 13,
    blue: 14,
};

pub fn part_one(input: &str) -> Option<u32> {
    let id_regex: Regex = Regex::new(r"Game (\d+):(.*)").unwrap();
    let grabs_regex: Regex = Regex::new(r"([\w\s,]*);").unwrap();
    let color_regex: Regex = Regex::new(r"\s(\d*)\s(\w*),").unwrap();

    let mut sum: u32 = 0;
    'outer: for line in input.lines() {
        let line = line.to_string() + ";";
        let caps = id_regex.captures(&line).unwrap();
        let id: u32 = caps[1].parse::<u32>().unwrap();
        let grabs = caps[2].to_string();
        for cap in grabs_regex.captures_iter(&grabs) {
            let grab = cap[1].to_string();
            let grab = grab + ",";
            let mut grab_bag = GrabBag {
                red: 0,
                green: 0,
                blue: 0,
            };
            for cap in color_regex.captures_iter(&grab) {
                let count = cap[1].parse::<u32>().unwrap();
                let color = cap[2].to_string();
                match color.as_str() {
                    "red" => grab_bag.red = count,
                    "green" => grab_bag.green = count,
                    "blue" => grab_bag.blue = count,
                    _ => panic!("Unknown color: {}", color),
                }
            }

            if grab_bag.red > MAX_GRAB.red
                || grab_bag.green > MAX_GRAB.green
                || grab_bag.blue > MAX_GRAB.blue
            {
                continue 'outer;
            }
        }
        sum += id;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let id_regex: Regex = Regex::new(r"Game (\d+):(.*)").unwrap();
    let grabs_regex: Regex = Regex::new(r"([\w\s,]*);").unwrap();
    let color_regex: Regex = Regex::new(r"\s(\d*)\s(\w*),").unwrap();

    let mut sum: u32 = 0;
    for line in input.lines() {
        let line = line.to_string() + ";";
        let caps = id_regex.captures(&line).unwrap();
        let grabs = caps[2].to_string();
        let mut bag: GrabBag = GrabBag {
            red: 0,
            green: 0,
            blue: 0,
        };
        for cap in grabs_regex.captures_iter(&grabs) {
            let grab = cap[1].to_string();
            let grab = grab + ",";
            for cap in color_regex.captures_iter(&grab) {
                let count = cap[1].parse::<u32>().unwrap();
                let color = cap[2].to_string();
                match color.as_str() {
                    "red" => {
                        if bag.red > count {
                            continue;
                        } else {
                            bag.red = count;
                        }
                    }
                    "green" => {
                        if bag.green > count {
                            continue;
                        } else {
                            bag.green = count;
                        }
                    }
                    "blue" => {
                        if bag.blue > count {
                            continue;
                        } else {
                            bag.blue = count;
                        }
                    }
                    _ => panic!("Unknown color: {}", color),
                }
            }
        }

        sum += bag.blue * bag.green * bag.red;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2286));
    }
}
