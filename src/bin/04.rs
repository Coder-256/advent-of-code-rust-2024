use advent_of_code::template::parse::lines;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = lines(input).map(|l| l.as_bytes()).filter(|l| !l.is_empty()).collect();
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut res = 0;
    for dx in [-1i32, 0, 1] {
        for dy in [-1i32, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            for x0 in 0..m {
                for y0 in 0..n {
                    if !(0..m).contains(&(x0 + 3 * dx)) || !(0..n).contains(&(y0 + 3 * dy)) {
                        continue;
                    }
                    if (
                        grid[x0 as usize][y0 as usize],
                        grid[(x0 + dx) as usize][(y0 + dy) as usize],
                        grid[(x0 + 2 * dx) as usize][(y0 + 2 * dy) as usize],
                        grid[(x0 + 3 * dx) as usize][(y0 + 3 * dy) as usize],
                    ) != (b'X', b'M', b'A', b'S')
                    {
                        continue;
                    }
                    res += 1;
                }
            }
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = lines(input).map(|l| l.as_bytes()).filter(|l| !l.is_empty()).collect();
    let m = grid.len();
    let n = grid[0].len();

    let mut res = 0;

    for x0 in 0..(m-2) {
        for y0 in 0..(n-2) {
            match (grid[x0][y0], grid[x0][y0+2], grid[x0+1][y0+1], grid[x0+2][y0], grid[x0+2][y0+2]) {
                (b'M', b'S', b'A', b'M', b'S') |
                (b'S', b'S', b'A', b'M', b'M') |
                (b'M', b'M', b'A', b'S', b'S') |
                (b'S', b'M', b'A', b'S', b'M') => res += 1,
                _ => ()
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
