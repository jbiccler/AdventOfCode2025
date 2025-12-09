use itertools::Itertools;
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

#[derive(Debug, Clone)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    #[inline]
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    #[inline]
    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            // a and b are in the same component
            return false;
        }
        // Make a represent the one with the largest size
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        // Set component of b equal to a and update sizes
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }

    #[inline]
    fn component_size(&mut self, x: usize) -> usize {
        let r = self.find(x);
        self.size[r]
    }
}

#[derive(Debug, Copy, Clone)]
struct Pair {
    a: usize,
    b: usize,
    dist: f64,
}

fn distance_vec(junctions: &[Junction]) -> Vec<Pair> {
    let mut dist = (0..junctions.len())
        .tuple_combinations()
        .map(|(i, j)| Pair {
            a: i,
            b: j,
            dist: junctions[i].distance(&junctions[j]),
        })
        .collect::<Vec<Pair>>();
    dist.sort_unstable_by(|a, b| {
        a.dist
            .partial_cmp(&b.dist)
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
    junctions: &[Junction],
    dist: &mut [Pair],
    n_connections: usize,
) -> Option<u64> {
    let n_junctions = junctions.len();
    let mut dsu = Dsu::new(n_junctions);
    let mut merges = 0;

    for p in dist.iter() {
        merges += 1;
        if dsu.union(p.a, p.b) {
            // Part 2: all junctions are joined in one component
            if dsu.component_size(p.a) == n_junctions {
                return Some(junctions[p.a].x * junctions[p.b].x);
            }
            // Part 1
            if merges >= n_connections {
                break;
            }
        }
    }
    // Part 1
    // Get sizes of the roots
    let mut sizes = vec![];
    for i in 0..n_junctions {
        let root = dsu.find(i);
        if root == i {
            sizes.push(dsu.size[root] as u64);
        }
    }
    // Sort in descending order and take product of top 3
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    Some(sizes.iter().take(3).product::<u64>())
}

pub fn part_one(input: &str) -> Option<u64> {
    let junc = parse_input(input);
    let mut dist = distance_vec(&junc);
    shortest_connections(&junc, &mut dist, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let junc = parse_input(input);
    let mut dist = distance_vec(&junc);
    shortest_connections(&junc, &mut dist, usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let junc = parse_input(input);
        let mut dist = distance_vec(&junc);
        let result = shortest_connections(&junc, &mut dist, 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
