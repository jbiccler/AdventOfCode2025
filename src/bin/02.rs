use rayon::prelude::*;
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
fn _twice_repeating(id: usize) -> bool {
    // Basic way of doing it by converting to string but slower
    let s = id.to_string();
    let n = s.len();
    let n_half = n / 2;
    let a = &s.as_bytes()[..n_half];
    let b = &s.as_bytes()[n_half..];
    a == b
}

#[inline(always)]
fn twice_repeating_num(id: usize) -> bool {
    let n = id.ilog10() + 1;
    let n_half = n / 2;
    let pow = 10usize.pow(n_half);
    id % pow == id / pow
}

#[inline(always)]
fn even_digits(n: usize) -> bool {
    if n == 0 {
        return false;
    }
    (n.ilog10() + 1) % 2 == 0
}

pub fn part_one(input: &str) -> Option<usize> {
    let ranges = parse_input(input);
    let sum = ranges
        .into_par_iter()
        .map(|(start, end)| {
            // Only need to check numbers with an even number of digits
            (start..=end)
                .filter(|&id| even_digits(id) && twice_repeating_num(id))
                .sum::<usize>()
        })
        .sum::<usize>();
    Some(sum)
}

#[inline(always)]
fn _n_repeating(id: usize) -> bool {
    // Basic way of doing it by converting to string but slower
    let s = id.to_string();
    let n = s.len();
    let n_half = n / 2;
    for k in 1..=n_half {
        if n % k == 0 {
            let first_str = s[..k].as_bytes();
            if s[k..].as_bytes().chunks(k).all(|x| x == first_str) {
                return true;
            }
        }
    }
    false
}

#[inline(always)]
fn n_repeating_num(id: usize) -> bool {
    let n = id.ilog10() + 1;
    // Precomupte powers
    let mut pow10 = [1usize; 20];
    for i in 1..pow10.len() {
        pow10[i] = pow10[i - 1] * 10;
    }
    for k in 1..=n / 2 {
        if n % k == 0 {
            // First part to match
            let pow = pow10[(n - k) as usize];
            let pattern = id / pow;
            // Start
            let mut check = id;
            let mut valid = true;
            for _ in 0..(n / k - 1) {
                let div = pow10[k as usize];
                let part = check % div;
                if part != pattern {
                    valid = false;
                    break;
                }
                check /= div;
            }
            if valid {
                return true;
            }
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<usize> {
    let ranges = parse_input(input);
    let sum = ranges
        .into_par_iter()
        .map(|(start, end)| {
            // Only need to check numbers with an even number of digits
            (start..=end)
                .filter(|&id| n_repeating_num(id))
                .sum::<usize>()
        })
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
