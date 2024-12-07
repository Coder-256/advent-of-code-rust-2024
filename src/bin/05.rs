use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
};

use advent_of_code::template::parse::lines;

advent_of_code::solution!(5);

fn parse_edges<'a>(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> HashMap<u32, Vec<u32>> {
    let mut order = HashMap::<u32, Vec<u32>>::new();
    while lines.peek().unwrap().contains('|') {
        let mut parts = lines.next().unwrap().split('|');
        let a = parts.next().unwrap().parse::<u32>().unwrap();
        let b = parts.next().unwrap().parse::<u32>().unwrap();
        order.entry(a).or_default().push(b);
    }
    order
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = lines(input).peekable();
    let order = parse_edges(&mut lines);
    let mut seen = Vec::<u32>::new();
    let mut res = 0;
    for line in lines {
        seen.clear();
        let mut nums = line.split(',').map(|s| s.parse::<u32>().unwrap());
        let mut highest = nums.clone();
        let mut parity = false;
        let ok = nums.all(|num| {
            parity = !parity;
            if !parity {
                highest.next();
            }
            let res = order
                .get(&num)
                .map_or(true, |forwards| forwards.iter().all(|f| !seen.contains(f)));
            seen.push(num);
            res
        });
        if !ok {
            continue;
        }
        res += highest.next().unwrap();
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = lines(input).peekable();
    let order = parse_edges(&mut lines);

    let mut nums = Vec::<u32>::new();
    let mut relevant_edges = Vec::<(u32, u32)>::new();
    let mut indeg = vec![0; nums.len()];
    let mut todo = Vec::<u32>::new();
    let mut res = 0;
    for line in lines {
        nums.clear();
        let mut ok = true;
        for num in line.split(',').map(|s| s.parse::<u32>().unwrap()) {
            if !order
                .get(&num)
                .map_or(true, |forwards| forwards.iter().all(|f| !nums.contains(f)))
            {
                ok = false;
            }
            nums.push(num);
        }
        if ok {
            continue;
        }
        // topsort
        indeg.clear();
        indeg.resize(nums.len(), 0);
        for &a in nums.iter() {
            for &b in order.get(&a).map(|v| v.iter()).into_iter().flatten() {
                if let Some((i, _)) = nums.iter().copied().enumerate().find(|&(_, x)| x == b) {
                    relevant_edges.push((a, b));
                    indeg[i] += 1;
                }
            }
        }
        todo.clear();
        todo.extend(
            nums.iter()
                .copied()
                .zip(indeg.iter().copied())
                .filter_map(|(x, d)| if d == 0 { Some(x) } else { None }),
        );
        for _ in 0..(nums.len() / 2) {
            let a = todo.pop().unwrap();
            for &b in order.get(&a).map(|v| v.iter()).into_iter().flatten() {
                for (&x, deg) in nums.iter().zip(indeg.iter_mut()) {
                    if x == b {
                        *deg -= 1;
                        if *deg == 0 {
                            todo.push(b);
                        }
                    }
                }
            }
        }
        res += todo.last().unwrap();
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
