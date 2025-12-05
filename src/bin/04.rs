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

fn parse_input(input: &str) -> (Vec<bool>, (usize, usize)) {
    let n = input.split_once("\n").unwrap().0.len();
    let m = input.lines().filter(|l| !l.is_empty()).count();
    (
        input
            .lines()
            .flat_map(|l| l.chars().map(|c| c == '@').collect::<Vec<bool>>())
            .collect(),
        (n, m),
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
fn neighbours(grid: &[bool], x: usize, y: usize, n: i32, m: i32) -> u8 {
    let mut count = 0;
    for (dx, dy) in DIRS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < n && ny >= 0 && ny < m {
            count += grid[idx(nx, ny, n) as usize] as u8;
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, (n, m)) = parse_input(input);
    let (n_i32, m_i32) = (n as i32, m as i32);
    let mut total = 0;
    for (i, c) in grid.iter().enumerate() {
        if *c {
            let (x, y) = reverse_idx(i, n);
            let count = neighbours(&grid, x, y, n_i32, m_i32);
            if count < MAX_COUNT {
                total += 1;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, (n, m)) = parse_input(input);
    let (n_i32, m_i32) = (n as i32, m as i32);
    let mut total = 0;
    // Vector of neighbour counts
    let mut counts = grid
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if !c {
                0
            } else {
                let (x, y) = reverse_idx(i, n);
                neighbours(&grid, x, y, n_i32, m_i32)
            }
        })
        .collect::<Vec<u8>>();

    // Vector of indices of next @ values to check
    let mut queue = grid
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if !c {
                None
            } else {
                if counts[i] < MAX_COUNT { Some(i) } else { None }
            }
        })
        .collect::<Vec<usize>>();

    while let Some(i) = queue.pop() {
        total += 1;
        if counts[i] > 0 {
            // Update neighbour counts
            let (x, y) = reverse_idx(i, n);
            for (dx, dy) in DIRS {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < n_i32 && ny >= 0 && ny < m_i32 {
                    let ni = idx(nx, ny, n_i32) as usize;
                    counts[ni] = counts[ni].saturating_sub(1);
                    // Branching here on if counts[ni] > 0 is slower.
                    // Check == MAX_COUNT - 1, not <= MAX_COUNT - 1 as then you duplicate elements in queue
                    if counts[ni] == MAX_COUNT - 1 {
                        queue.push(ni);
                    }
                }
            }
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
