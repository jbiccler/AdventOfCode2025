advent_of_code::solution!(1);

const BOUND: i32 = 100;
const START: i32 = 50;

pub fn part_one(input: &str) -> Option<i32> {
    let mut count = 0;
    let mut current = START;
    for l in input.lines() {
        let l = l.trim();
        if l.is_empty() {
            continue;
        };
        let (dir, val) = l.split_at(1);
        let val = val.parse::<i32>().unwrap();
        let d = match dir {
            "L" => -1,
            "R" => 1,
            _ => panic!(),
        };
        current = (current + val * d).rem_euclid(BOUND);
        if current == 0 {
            count += 1;
        }
    }
    Some(count)
}

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

pub fn part_two(input: &str) -> Option<i32> {
    let mut count = 0;
    let mut current = START;
    for l in input.lines() {
        let l = l.trim();
        if l.is_empty() {
            continue;
        };
        let (dir, val) = l.split_at(1);
        let val = val.parse::<i32>().unwrap();
        let d = match dir {
            "L" => -1,
            "R" => 1,
            _ => 0,
        };
        let (next, passing) = wrap_count_passing_zero(current, val * d);
        current = next;
        count += passing;
    }
    Some(count)
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
