use good_lp::*;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Machine {
    target: u64,
    buttons: Vec<u64>,
    joltage: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                None
            } else {
                let (target_str, rest) = l.split_once("] ").unwrap();
                let target_str = &target_str[1..target_str.len()];
                let mut target: u64 = 0;
                for bit in 0..target_str.len() {
                    let b = &target_str[bit..=bit];
                    if b == "#" {
                        target |= 1 << bit;
                    }
                }

                let curly_idx = rest.find("{").unwrap();
                let buttons_str = &rest[..curly_idx - 1];
                let mut buttons = vec![];
                for b in buttons_str.split_whitespace() {
                    let mut button_out: u64 = 0;
                    for button in b[1..b.len() - 1].split(',') {
                        let parsed = button.parse::<u64>().unwrap();
                        button_out |= 1 << parsed;
                    }
                    buttons.push(button_out)
                }
                let mut joltage: Vec<u64> = vec![];
                for j in rest[curly_idx + 1..rest.len() - 1].split(',') {
                    let j = j.parse().unwrap();
                    joltage.push(j);
                }

                Some(Machine {
                    target,
                    buttons,
                    joltage,
                })
            }
        })
        .collect()
}

impl Machine {
    fn min_toggles_lights(&self) -> Option<u64> {
        let target = self.target;
        // Edge case
        if target == 0 {
            return Some(0);
        }
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back((0u64, 0u64)); // (state, steps)
        seen.insert(0u64);

        while let Some((state, steps)) = queue.pop_front() {
            for &btn in self.buttons.iter() {
                let next = state ^ btn;
                if next == target {
                    return Some(steps + 1);
                }
                if !seen.contains(&next) {
                    seen.insert(next);
                    queue.push_back((next, steps + 1));
                }
            }
        }
        None
    }

    /// Solve part 2 using a fast MILP solver (Highs).
    pub fn min_presses_good_lp(&self) -> Option<u64> {
        let m = self.joltage.len();
        let n = self.buttons.len();

        // Create variable per button
        let mut vars = variables!();
        let x = vars.add_vector(variable().min(0).integer(), n);
        // Minimize the sum of the presses
        let objective: Expression = x.iter().sum();

        // Build the MILP model
        let mut model = highs(vars.minimise(objective));

        // Add which joltages are increased by which button
        for i in 0..m {
            let mut expr = Expression::from(0.0);
            for (j, xj) in x.iter().enumerate() {
                if (self.buttons[j] >> i) & 1 == 1 {
                    expr += xj;
                }
            }
            model = model.with(expr.eq(self.joltage[i] as f64));
        }

        let solution = model.solve().ok()?;

        // Extract final result = sum of presses
        let mut total = 0u64;
        for &xj in x.iter() {
            let val = solution.value(xj);
            if val < -1e-9 {
                return None; // should never happen
            }
            total += val.round() as u64;
        }

        Some(total)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let sum = machines
        .par_iter()
        .map(|m| m.min_toggles_lights().unwrap())
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    machines.par_iter().map(|m| m.min_presses_good_lp()).reduce(
        || Some(0),
        |acc, x| match (acc, x) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
