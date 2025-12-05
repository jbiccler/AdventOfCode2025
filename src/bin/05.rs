use anyhow::{Context, Result, anyhow};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> Result<(Vec<(u64, u64)>, Vec<u64>)> {
    let (rngs, ings) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Couldnt parse line break"))?;
    let rngs = rngs
        .lines()
        .map(|l| {
            let (a, b) = l
                .trim()
                .split_once("-")
                .ok_or_else(|| anyhow!("Couldn't split ranges on -"))?;
            Ok((
                a.parse::<u64>()
                    .context("Failed to parse start val of range")?,
                b.parse::<u64>()
                    .context("Failed to parse right val of range")?,
            ))
        })
        .collect::<Result<Vec<(u64, u64)>>>()?;
    let ings = ings
        .lines()
        .map(|l| {
            Ok(l.trim()
                .parse::<u64>()
                .context("Failed to parse ingredient")?)
        })
        .collect::<Result<Vec<u64>>>()?;
    Ok((rngs, ings))
}

fn merge_ranges(rngs: &mut [(u64, u64)]) -> Vec<(u64, u64)> {
    // Sort by start value
    rngs.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (a, b) in rngs {
        if let Some((_, last_b)) = merged.last_mut() {
            // last_b + 1 as the ranges are inclusive
            if *a <= *last_b + 1 {
                // Merge
                let new_end = *b.max(last_b);
                *last_b = new_end;
            } else {
                // Disjoint
                merged.push((*a, *b));
            }
        } else {
            // First iteration
            merged.push((*a, *b));
        }
    }
    merged
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut rngs, ings) = parse_input(input).ok()?;
    let merged = merge_ranges(&mut rngs);
    Some(
        ings.iter()
            .map(|&i| {
                // Parition point returns the fist where index of the second partition (start where predicate = false) assuming merged is sorted based on the predicate.
                // +- binary search
                let p = merged.partition_point(|(_, b)| i > *b);
                merged.get(p).map_or(0, |(a, _)| u64::from(i >= *a))
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut rngs, _) = parse_input(input).ok()?;
    let merged = merge_ranges(&mut rngs);
    let count = merged.iter().fold(0, |acc, (a, b)| acc + b - a + 1);
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
        assert_eq!(result, Some(14));
    }
}
