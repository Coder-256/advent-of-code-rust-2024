use advent_of_code::template::parse::lines;

advent_of_code::solution!(7);

fn dfs1(target: u64, prior: u64, eqn: &[u64]) -> bool {
    match eqn.split_first() {
        None => prior == target,
        Some((x, tail)) => dfs1(target, prior + x, tail) || dfs1(target, prior * x, tail),
    }
}

const POW10: &[u64] = &[
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
];

fn dfs2(target: u64, prior: u64, eqn: &[(u64, usize)]) -> bool {
    prior <= target
        && match eqn.split_first() {
            None => prior == target,
            Some((&(x, len), tail)) => {
                dfs2(target, prior + x, tail)
                    || dfs2(target, prior * x, tail)
                    || dfs2(target, (prior * POW10[len]) + x, tail)
            }
        }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut res = 0;
    for line in lines(input) {
        let mut splita = line.split(':');
        let target = splita.next().unwrap().parse::<u64>().unwrap();
        let rest = splita.next().unwrap();
        let eqn = rest
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let (&init, tail) = eqn.split_first().unwrap();
        if dfs1(target, init, tail) {
            res += target;
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut res = 0;
    for line in lines(input) {
        let mut splita = line.split(':');
        let target = splita.next().unwrap().parse::<u64>().unwrap();
        let rest = splita.next().unwrap();
        let eqn = rest
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| (s.parse::<u64>().unwrap(), s.len()))
            .collect::<Vec<(u64, usize)>>();
        let (&(init, _), tail) = eqn.split_first().unwrap();
        if dfs2(target, init, tail) {
            res += target;
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
