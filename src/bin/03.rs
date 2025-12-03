advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect()
}

#[inline(always)]
fn get_joltage(bank: &[u8], num_digits: usize) -> u64 {
    let n = bank.len();
    // Track max value and index per selected digit
    let mut max_val = vec![0; num_digits];
    let mut max_idx = vec![0; num_digits];
    // Track overall result
    let mut res = 0;
    for i in 0..num_digits {
        // Setup window to look at, start looking at window size = n - num_digits + 1 left most digits
        // (look at one more digit than we are allowed to skip)
        // And decrease window size as we select digits
        let start = if i > 0 { max_idx[i - 1] + 1 } else { 0 };
        let end = n - num_digits + 1 + i;
        for j in start..end {
            if bank[j] > max_val[i] {
                max_idx[i] = j;
                max_val[i] = bank[j];
            }
        }
        res *= 10;
        res += max_val[i] as u64;
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = parse_input(input);
    Some(banks.iter().map(|b| get_joltage(b, 2)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = parse_input(input);
    Some(banks.into_iter().map(|b| get_joltage(&b, 12)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let a: u64 = 3121910778619;
        assert_eq!(result, Some(a));
    }
}
