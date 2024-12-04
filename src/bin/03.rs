use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0u32;
    for capture in pattern.captures_iter(input) {
        let (_, [xstr, ystr]) = capture.extract();
        let x = xstr.parse::<u32>().unwrap();
        let y = ystr.parse::<u32>().unwrap();
        result += x * y;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let enable = Regex::new(r"do\(\)").unwrap();
    let disable = Regex::new(r"don't\(\)").unwrap();
    let mut enables = enable.find_iter(input).peekable();
    let mut disables = disable.find_iter(input).peekable();
    let mut enabled = true;
    let mut result = 0u32;
    for capture in pattern.captures_iter(input) {
        let i = capture.get(0).unwrap().start();
        loop {
            match (
                enables.peek().filter(|e| e.start() < i),
                disables.peek().filter(|d| d.start() < i),
            ) {
                (Some(e), Some(d)) => {
                    if e.start() < d.start() {
                        enabled = true;
                        enables.next();
                    } else {
                        enabled = false;
                        disables.next();
                    }
                }
                (Some(_), None) => {
                    enabled = true;
                    enables.next();
                }
                (None, Some(_)) => {
                    enabled = false;
                    disables.next();
                }
                (None, None) => break,
            }
        }

        if enabled {
            let (_, [xstr, ystr]) = capture.extract();
            let x = xstr.parse::<u32>().unwrap();
            let y = ystr.parse::<u32>().unwrap();
            result += x * y;
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
