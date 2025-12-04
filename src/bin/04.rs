advent_of_code::solution!(4);

const MAX_COUNT: u8 = 4;
const DIRS: [(i32, i32); 8] = [
    (-1, 0),  // Left
    (1, 0),   // Right
    (0, -1),  // Up
    (0, 1),   // Down
    (-1, -1), // Up-Left
    (1, -1),  // Up-Right
    (-1, 1),  // Down-Left
    (1, 1),   // Down-Right
];

fn parse_input(input: &str) -> (Vec<bool>, usize) {
    let n = input.split_once("\n").unwrap().0.len();
    (
        input
            .lines()
            .flat_map(|l| l.chars().map(|c| c == '@').collect::<Vec<bool>>())
            .collect(),
        n,
    )
}

#[inline(always)]
fn idx(x: i32, y: i32, n: i32) -> i32 {
    y * n + x
}

#[inline(always)]
fn reverse_idx(idx: usize, n: usize) -> (usize, usize) {
    (idx % n, idx / n)
}

#[inline(always)]
fn neighbours(grid: &[bool], x: usize, y: usize, n: i32) -> u8 {
    let mut count = 0;
    for (dx, dy) in DIRS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < n && ny >= 0 && ny < n {
            count += grid[idx(nx, ny, n) as usize] as u8;
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, n) = parse_input(input);
    let n_i32 = n as i32;
    let mut total = 0;
    for (i, c) in grid.iter().enumerate() {
        if *c {
            let (x, y) = reverse_idx(i, n);
            let count = neighbours(&grid, x, y, n_i32);
            if count < MAX_COUNT {
                total += 1;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut grid, n) = parse_input(input);
    let n_i32 = n as i32;
    let mut total = 0;
    let mut prev_total = u64::MAX;
    while prev_total != total {
        prev_total = total;
        // Simply iterating over 0..grid.len() is faster than
        // tracking indexes of @ values and only iterating over those
        // for this size of input
        for i in 0..grid.len() {
            if grid[i] {
                let (x, y) = reverse_idx(i, n);
                let count = neighbours(&grid, x, y, n_i32);
                if count < MAX_COUNT {
                    grid[i] = false;
                    total += 1;
                }
            }
        }
    }
    Some(total)
}

/// Deprecated as tracking indexes of @ values is slower than simply iterating
/// over 0..grid.len(), kept for reference
pub fn _part_two(input: &str) -> Option<u64> {
    let (mut grid, n) = parse_input(input);
    let n_i32 = n as i32;
    let mut total = 0;
    let mut removed = true;
    let mut idxs: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if *c { Some(i) } else { None })
        .collect();
    let mut idxs_to_remove = vec![];
    while removed && !idxs.is_empty() {
        for (j, &i) in idxs.iter().enumerate() {
            let (x, y) = reverse_idx(i, n);
            let count = neighbours(&grid, x, y, n_i32);
            if count < 4 {
                idxs_to_remove.push(j);
                grid[i] = false;
                total += 1;
            }
        }
        if !idxs_to_remove.is_empty() {
            for &i in idxs_to_remove.iter().rev() {
                idxs.remove(i);
            }
            idxs_to_remove.clear();
        } else {
            removed = false;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
