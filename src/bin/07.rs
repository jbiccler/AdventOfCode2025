advent_of_code::solution!(7);

fn parse_input(input: &str) -> (usize, Vec<Vec<char>>) {
    let start = input.split_once("\n").unwrap().0.find("S").unwrap();
    let grid = input
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect::<Vec<Vec<char>>>();
    (start, grid)
}

#[inline(always)]
fn count_splits(start: usize, grid: &[Vec<char>]) -> (usize, u64) {
    let n = grid[0].len();
    let mut split_count = 0;
    let mut splits = vec![0u64; n];
    splits[start] = 1;

    grid.iter().skip(1).for_each(|row| {
        row.iter().enumerate().for_each(|(i, &x)| {
            if x == '^' && splits[i] > 0 {
                split_count += 1;
                splits[i - 1] += splits[i];
                splits[i + 1] += splits[i];
                splits[i] = 0;
            }
        })
    });
    (split_count, splits.iter().sum())
}

pub fn part_one(input: &str) -> Option<usize> {
    let (start, grid) = parse_input(input);
    let (count, _) = count_splits(start, &grid);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (start, grid) = parse_input(input);
    let (_, n_timelines) = count_splits(start, &grid);
    Some(n_timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
