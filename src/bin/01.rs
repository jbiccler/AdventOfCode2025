advent_of_code::solution!(1);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left(i32),
    Right(i32),
}

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let line = l.trim();
            let dir = &line[0..1];
            let val = &line[1..]
                .parse::<i32>()
                .expect("Failed to parse digit in input");
            match dir {
                "L" => Direction::Left(*val),
                "R" => Direction::Right(*val),
                _ => panic!("Wrong input for direction"),
            }
        })
        .collect()
}

/// Calculate next value wrapping around bound 0..=99;
#[inline(always)]
fn wrap(current: i32, dir: &Direction) -> i32 {
    match dir {
        Direction::Left(v) => (current - v).rem_euclid(100),
        Direction::Right(v) => (current + v).rem_euclid(100),
    }
}

fn solve_part1(ins: &[Direction]) -> u64 {
    let mut count = 0;
    // Start value as per instructions is 50
    let mut prev = 50;
    for i in ins.iter() {
        prev = wrap(prev, i);
        if prev == 0 {
            count += 1;
        }
    }
    count
}

/// Calculate next value wrapping around bound 0..=99 and return number of times we passed 0 in the
/// process.
#[inline(always)]
fn wrap_count_passing_zero(current: i32, dir: &Direction) -> (i32, i32) {
    let next = match dir {
        Direction::Left(v) => (current - v).rem_euclid(100),
        Direction::Right(v) => (current + v).rem_euclid(100),
    };
    let passed_zero = match dir {
        Direction::Left(v) => {
            // Correction needed for first passage through zero as this is not captured by whole
            // division. If current was already == 0, then this was already counted in the previous
            // cycle.
            if (current != 0) && (current - v) <= 0 {
                ((current - v) / 100).abs() + 1
            } else {
                ((current - v) / 100).abs()
            }
        }
        Direction::Right(v) => (current + v) / 100,
    };
    (next, passed_zero)
}

fn solve_part2(ins: &[Direction]) -> u64 {
    let mut count = 0;
    // Start value as per instructions is 50
    let mut prev = 50;
    for i in ins.iter() {
        let (prev_tmp, passing) = wrap_count_passing_zero(prev, i);
        prev = prev_tmp;
        count += passing as u64;
    }
    count
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
