use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut l1 = Vec::<u32>::new();
    let mut l2 = Vec::<u32>::new();
    for line in input
        .split('\n')
        .map(|l| l.trim_ascii())
        .filter(|l| !l.is_empty())
    {
        let mut parts = line.split_ascii_whitespace();
        l1.push(parts.next().unwrap().parse().unwrap());
        l2.push(parts.next().unwrap().parse().unwrap());
    }

    l1.sort();
    l2.sort();

    Some(l1.iter().zip(l2.iter()).map(|(&a, &b)| a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list = Vec::<u32>::new();
    let mut count = HashMap::<u32, u32>::new();

    for line in input
        .lines()
        .map(|l| l.trim_ascii())
        .filter(|l| !l.is_empty())
    {
        let mut parts = line.split_ascii_whitespace();
        let x1 = parts.next().unwrap().parse().unwrap();
        let x2 = parts.next().unwrap().parse().unwrap();
        list.push(x1);
        *count.entry(x2).or_insert(0) += 1;
    }

    Some(
        list.iter()
            .filter_map(|x| count.get(x).map(|c| x * c))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
