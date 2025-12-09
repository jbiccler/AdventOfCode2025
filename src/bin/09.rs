use itertools::Itertools;
advent_of_code::solution!(9);

#[derive(Debug, Copy, Clone)]
struct Corner {
    x: u64,
    y: u64,
}

fn parse_input(input: &str) -> Vec<Corner> {
    input
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                None
            } else {
                let (x, y) = l.split_once(",").unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                Some(Corner { x, y })
            }
        })
        .collect()
}

fn area(c1: &Corner, c2: &Corner) -> u64 {
    (c1.x.abs_diff(c2.x) + 1) * (c1.y.abs_diff(c2.y) + 1)
}

pub fn part_one(input: &str) -> Option<u64> {
    let corners = parse_input(input);
    corners
        .iter()
        .combinations(2)
        .map(|c| area(c[0], c[1]))
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let corners = parse_input(input);
    let n = corners.len();
    let mut areas = corners
        .iter()
        .combinations(2)
        .map(|c| (c[0], c[1], area(c[0], c[1])))
        .collect::<Vec<(&Corner, &Corner, u64)>>();
    areas.sort_unstable_by_key(|(_, _, a)| *a);
    let greens = (0..n)
        .map(|i| {
            let (c1, c2) = (corners[i], corners[(i + 1) % n]);
            let (min_x, max_x) = (c1.x.min(c2.x), c1.x.max(c2.x));
            let (min_y, max_y) = (c1.y.min(c2.y), c1.y.max(c2.y));
            (min_x, max_x, min_y, max_y)
        })
        .collect::<Vec<(u64, u64, u64, u64)>>();
    while let Some((c1, c2, a)) = areas.pop() {
        // If valid break
        let (min_x, max_x) = (c1.x.min(c2.x), c1.x.max(c2.x));
        let (min_y, max_y) = (c1.y.min(c2.y), c1.y.max(c2.y));
        let mut brk = false;
        for (x1, x2, y1, y2) in greens.iter() {
            if min_x < *x2 && min_y < *y2 && max_x > *x1 && max_y > *y1 {
                brk = true;
                break;
            }
        }
        if !brk {
            return Some(a);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
