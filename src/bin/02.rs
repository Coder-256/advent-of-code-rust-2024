use advent_of_code::template::parse::splitn;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        splitn::<u32>(input)
            .map(|report| {
                if report.len() < 2 {
                    1
                } else {
                    let increase = report[0] < report[1];
                    if report
                        .iter()
                        .zip(report[1..].iter())
                        .all(|(&a, &b)| (1..=3).contains(&a.abs_diff(b)) && (a < b) == increase)
                    {
                        1
                    } else {
                        0
                    }
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        splitn::<u32>(input)
            .map(|report| {
                if report.len() < 3 {
                    1
                } else {
                    let mut sub = vec![];
                    if (0..report.len()).any(|bad_idx| {
                        sub.clear();
                        sub.extend_from_slice(&report[..bad_idx]);
                        sub.extend_from_slice(&report[(bad_idx + 1)..]);
                        let increase = sub[0] < sub[1];
                        sub.iter()
                            .zip(sub[1..].iter())
                            .all(|(&a, &b)| (1..=3).contains(&a.abs_diff(b)) && (a < b) == increase)
                    }) {
                        1
                    } else {
                        0
                    }
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
