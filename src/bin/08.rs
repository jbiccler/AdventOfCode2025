use itertools::Itertools;
use std::collections::HashSet;
advent_of_code::solution!(8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Junction {
    x: u64,
    y: u64,
    z: u64,
}

impl Junction {
    fn distance(&self, other: &Self) -> f64 {
        ((self.x as f64 - other.x as f64).powf(2.)
            + (self.y as f64 - other.y as f64).powf(2.)
            + (self.z as f64 - other.z as f64).powf(2.))
        .sqrt()
    }
}

#[derive(Debug, Copy, Clone)]
struct Pair {
    a: Junction,
    b: Junction,
    dist: f64,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b) && (self.b == other.a)
    }
}

fn distance_vec(junctions: Vec<Junction>) -> Vec<Pair> {
    let mut dist: Vec<Pair> = junctions
        .into_iter()
        .combinations(2)
        .map(|comb| {
            let a = comb[0];
            let b = comb[1];
            Pair {
                a,
                b,
                dist: a.distance(&b),
            }
        })
        .collect();
    dist.sort_unstable_by(|a, b| {
        b.dist
            .partial_cmp(&a.dist)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    dist
}

fn parse_input(input: &str) -> Vec<Junction> {
    input
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                None
            } else {
                let (x, rest) = l.split_once(",").unwrap();
                let (y, z) = rest.split_once(",").unwrap();
                Some(Junction {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                    z: z.parse().unwrap(),
                })
            }
        })
        .collect()
}

fn shortest_connections(
    dist: &mut Vec<Pair>,
    n_connections: usize,
    n_junctions: usize,
) -> Option<u64> {
    let mut circuits: Vec<HashSet<Junction>> = vec![];
    let mut count = 0;
    let mut added;
    let mut a_idx = None;
    let mut b_idx = None;
    while let Some(p) = dist.pop() {
        added = false;
        for (i, c) in circuits.iter().enumerate() {
            if c.iter().contains(&p.a) {
                a_idx = Some(i);
            }
            if c.iter().contains(&p.b) {
                b_idx = Some(i);
            }
        }
        if let Some(a) = a_idx {
            if b_idx.is_none() {
                // b is new
                circuits[a].insert(p.b);
            } else if a_idx != b_idx {
                // Combine both networks
                let b = b_idx.unwrap();
                let (a, b) = if a < b { (a, b) } else { (b, a) };
                let (left, right) = circuits.split_at_mut(b);
                left[a].extend(&right[0]);
                circuits.remove(b);
            }
            added = true;
        } else if let Some(b) = b_idx {
            if a_idx.is_none() {
                // a is new
                circuits[b].insert(p.a);
            } else if a_idx != b_idx {
                // Combine both networks
                let a = a_idx.unwrap();
                let (a, b) = if a < b { (a, b) } else { (b, a) };
                let (left, right) = circuits.split_at_mut(b);
                left[a].extend(&right[0]);
                circuits.remove(b);
            }
            added = true;
        }
        if !added {
            // None of the circuits contained a or b -> create new circuit
            circuits.push(HashSet::from([p.a, p.b]));
        }
        a_idx = None;
        b_idx = None;
        count += 1;
        // Part 1 limits number of connections to make
        if count >= n_connections {
            break;
        }
        // Part 2 requires you to assign all n junctions and return product of x coordinates of last 2
        if circuits.len() == 1 && circuits[0].len() == n_junctions {
            return Some(p.a.x * p.b.x);
        }
    }
    // Part 1 return
    circuits.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
    Some(circuits.iter().take(3).map(|c| c.len() as u64).product())
}

pub fn part_one(input: &str) -> Option<u64> {
    let junc = parse_input(input);
    let n = junc.len();
    let mut dist = distance_vec(junc);
    shortest_connections(&mut dist, 1000, n)
}

pub fn part_two(input: &str) -> Option<u64> {
    let junc = parse_input(input);
    let n = junc.len();
    let mut dist = distance_vec(junc);
    shortest_connections(&mut dist, usize::MAX, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let junc = parse_input(input);
        let n = junc.len();
        let mut dist = distance_vec(junc);
        let result = shortest_connections(&mut dist, 10, n);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
