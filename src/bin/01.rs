use std::collections::HashMap;

use advent_of_code::template::parse::split2;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for (x1, x2) in split2::<u32, u32>(input) {
        list1.push(x1);
        list2.push(x2);
    }

    list1.sort();
    list2.sort();

    Some(
        list1
            .iter()
            .zip(list2.iter())
            .map(|(&a, &b)| a.abs_diff(b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1 = vec![];
    let mut count2 = HashMap::new();

    for (x1, x2) in split2::<u32, u32>(input) {
        list1.push(x1);
        *count2.entry(x2).or_insert(0) += 1;
    }

    Some(
        list1
            .iter()
            .filter_map(|x| count2.get(x).map(|c| x * c))
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
