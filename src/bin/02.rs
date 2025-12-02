use std::collections::HashSet;
advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .split(",")
        .map(|s| {
            let (start, end) = s.split_once("-").unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            (start, end)
        })
        .collect()
}

#[inline(always)]
fn generate_twice_repeating_in_range(start: usize, end: usize) -> usize {
    let mut sum = 0;
    let max_digits = end.ilog10() + 1;
    let min_digits = start.ilog10() + 1;
    let half_min_digits = (start.ilog10() + 1) / 2;
    for k in half_min_digits..=(max_digits / 2) {
        let pow_d = 10usize.pow(k);
        let max_x = pow_d - 1;
        let min_x = if k == half_min_digits {
            start / 10usize.pow(min_digits - k)
        } else {
            pow_d / 10
        };
        for x in min_x..=max_x {
            let id = x * pow_d + x;
            if id > end {
                break;
            } else if id >= start {
                sum += id;
            }
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<usize> {
    let ranges = parse_input(input);
    let sum = ranges
        .into_iter()
        .map(|(start, end)| generate_twice_repeating_in_range(start, end))
        .sum::<usize>();
    Some(sum)
}

fn generate_n_repeating_in_range(start: usize, end: usize) -> usize {
    // To avoid adding duplicate pairs like 1 & 1 & 1 & 1 = 1111 and 11 & 11 == 1111
    let mut seen: HashSet<usize> = HashSet::new();
    let mut sum = 0;
    // Precompute powers
    let mut pow10 = [1usize; 20];
    for i in 1..pow10.len() {
        pow10[i] = pow10[i - 1] * 10;
    }
    // Get length bounds
    // End
    let max_digits = end.ilog10() + 1;
    // Start
    let min_digits = start.ilog10() + 1;
    // Construct interim bounds to focus on same number of digits
    // e.g. for start = 95 and end = 20065
    // [start=95, 99, 999, 9999, end=20065]
    let mut bounds = Vec::with_capacity(2 + (max_digits - min_digits) as usize);
    bounds.push(start);
    bounds.extend((0..(max_digits - min_digits)).scan(start, |state, x| {
        *state = ((*state as f64) / 10.0f64.powf((min_digits + x) as f64)).ceil()
            as usize
            * 10usize.pow(min_digits) // e.g. 1000 -> deduct 1
            - 1;
        Some(*state)
    }));
    bounds.push(end);
    let mut first = true;
    for w in bounds.windows(2) {
        let (mut start, end) = (w[0], w[1]);
        // Adjust start boundaries of e.g. 999 -> 1000 after the first iteration.
        // Such that start and end have same number of digits.
        if !first {
            start += 1;
        }
        let n = start.ilog10() + 1;
        // Loop over sizes of repeating numbers
        for k in 1..=(n / 2) {
            if n % k != 0 {
                continue;
            }
            let pow_d = pow10[k as usize];
            let max_x = pow_d - 1;
            // In case of first iteration, start from k most significant digits of start.
            let min_x = if first {
                start / pow10[(n - k) as usize]
            } else {
                pow_d / 10
            };
            // Nr of repeated additions of this subset of digits: x;
            let repeats = n / k;
            for x in min_x..=max_x {
                let mut id = 0;
                for _ in 0..repeats {
                    id = id * pow_d + x;
                }
                if id > end {
                    break;
                } else if id >= start && !seen.contains(&id) {
                    sum += id;
                    seen.insert(id);
                }
            }
        }
        first = false;
    }
    sum
}

pub fn part_two(input: &str) -> Option<usize> {
    let ranges = parse_input(input);
    let sum = ranges
        .into_iter()
        .map(|(start, end)| generate_n_repeating_in_range(start, end))
        .sum::<usize>();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
