use itertools::Itertools;
advent_of_code::solution!(8);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
struct Circuit {
    data: Vec<Junction>,
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

fn shortest_connections(dist: &mut Vec<Pair>, n: usize) -> Vec<Circuit> {
    let mut circuits: Vec<Circuit> = vec![];
    let mut count = 0;
    let mut added;
    let mut a_idx = None;
    let mut b_idx = None;
    while let Some(p) = dist.pop() {
        added = false;
        for (i, c) in circuits.iter().enumerate() {
            if c.data.contains(&p.a) {
                a_idx = Some(i);
            }
            if c.data.contains(&p.b) {
                b_idx = Some(i);
            }
        }
        if let Some(a) = a_idx {
            if b_idx.is_none() {
                circuits[a].data.push(p.b)
            } else if a_idx == b_idx {
                // Do nothing
            } else {
                // Combine both networks
                let mut b_cpy = circuits[b_idx.unwrap()].data.clone();
                circuits[a].data.append(&mut b_cpy);
                circuits.remove(b_idx.unwrap());
            }
            added = true;
        } else if let Some(b) = b_idx {
            if a_idx.is_none() {
                circuits[b].data.push(p.a)
            } else if a_idx == b_idx {
                // Do nothing
            } else {
                // Combine both networks
                let mut a_cpy = circuits[a_idx.unwrap()].data.clone();
                circuits[b].data.append(&mut a_cpy);
                circuits.remove(a_idx.unwrap());
            }
            added = true;
        }
        if !added {
            // None of the circuits contained a or b -> create new circuit
            circuits.push(Circuit {
                data: vec![p.a, p.b],
            });
        }
        a_idx = None;
        b_idx = None;
        count += 1;
        if count >= n {
            break;
        }
    }
    circuits.sort_unstable_by(|a, b| b.data.len().cmp(&a.data.len()));
    circuits
}

pub fn part_one(input: &str) -> Option<usize> {
    let junc = parse_input(input);
    let mut dist = distance_vec(junc);
    let circuits = shortest_connections(&mut dist, 1000);
    Some(circuits.iter().take(3).map(|c| c.data.len()).product())
}

fn shortest_connections_single_circuit(dist: &mut Vec<Pair>, n: usize) -> Option<u64> {
    let mut circuits: Vec<Circuit> = vec![];
    let mut added;
    let mut a_idx = None;
    let mut b_idx = None;
    while let Some(p) = dist.pop() {
        added = false;
        for (i, c) in circuits.iter().enumerate() {
            if c.data.contains(&p.a) {
                a_idx = Some(i);
            }
            if c.data.contains(&p.b) {
                b_idx = Some(i);
            }
        }
        if let Some(a) = a_idx {
            if b_idx.is_none() {
                circuits[a].data.push(p.b)
            } else if a_idx == b_idx {
                // Do nothing
            } else {
                // Combine both networks
                let mut b_cpy = circuits[b_idx.unwrap()].data.clone();
                circuits[a].data.append(&mut b_cpy);
                circuits.remove(b_idx.unwrap());
            }
            added = true;
        } else if let Some(b) = b_idx {
            if a_idx.is_none() {
                circuits[b].data.push(p.a)
            } else if a_idx == b_idx {
                // Do nothing
            } else {
                // Combine both networks
                let mut a_cpy = circuits[a_idx.unwrap()].data.clone();
                circuits[b].data.append(&mut a_cpy);
                circuits.remove(a_idx.unwrap());
            }
            added = true;
        }
        if !added {
            // None of the circuits contained a or b -> create new circuit
            circuits.push(Circuit {
                data: vec![p.a, p.b],
            });
        }
        a_idx = None;
        b_idx = None;
        if circuits.len() == 1 && circuits[0].data.len() == n {
            return Some(p.a.x * p.b.x);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let junc = parse_input(input);
    let n = junc.len();
    let mut dist = distance_vec(junc);
    shortest_connections_single_circuit(&mut dist, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let junc = parse_input(input);
        let mut dist = distance_vec(junc);
        let circuits = shortest_connections(&mut dist, 10);
        let result = Some(circuits.iter().take(3).map(|c| c.data.len()).product());
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
