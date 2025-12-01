advent_of_code::solution!(1);

const BOUND: i32 = 100;
const START: i32 = 50;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                return None;
            };
            let (dir, val) = l.split_at(1);
            let val = val.parse::<i32>().ok()?;
            match dir {
                "L" => Some(-val),
                "R" => Some(val),
                _ => None,
            }
        })
        .collect()
}

/// Calculate next value wrapping around bound 0..=99;
#[inline(always)]
fn wrap(current: i32, dir: i32) -> i32 {
    (current + dir).rem_euclid(BOUND)
}

fn solve_part1(ins: &[i32]) -> u64 {
    ins.iter()
        .scan(START, |prev, dir| {
            *prev = wrap(*prev, *dir);
            Some(*prev == 0)
        })
        .filter(|&passed_zero| passed_zero)
        .count() as u64
}

/// Calculate next value wrapping around bound 0..=99 and return number of times we passed 0 in the
/// process.
#[inline(always)]
fn wrap_count_passing_zero(current: i32, dir: i32) -> (i32, i32) {
    let next = (current + dir).rem_euclid(BOUND);
    if dir >= 0 {
        // Right case
        (next, (current + dir) / BOUND)
    } else {
        // Left case
        // Correction needed for first passage through zero as this is not captured by whole
        // division. If current was already == 0, then this was already counted in the previous
        // cycle.
        let correction = ((current != 0) && (current + dir) <= 0) as i32;
        (next, ((current + dir) / BOUND).abs() + correction)
    }
}

fn solve_part2(ins: &[i32]) -> u64 {
    ins.iter()
        .scan(START, |prev, dir| {
            let (prev_tmp, passing) = wrap_count_passing_zero(*prev, *dir);
            *prev = prev_tmp;
            Some(passing)
        })
        .sum::<i32>() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let ins = parse_input(input);
    Some(solve_part1(&ins))
}

pub fn part_two(input: &str) -> Option<u64> {
    let ins = parse_input(input);
    Some(solve_part2(&ins))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
