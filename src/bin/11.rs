use std::collections::HashMap;
advent_of_code::solution!(11);

type Graph = Vec<Vec<usize>>;

fn parse_input(input: &str) -> (Graph, HashMap<String, usize>) {
    let mut id_map = HashMap::new();
    let mut names = vec![];

    // Helper
    let mut set_id = |name: &str| -> usize {
        if let Some(&id) = id_map.get(name) {
            return id;
        }
        let id = names.len();
        names.push(name.to_string());
        id_map.insert(name.to_string(), id);
        id
    };
    // First pass to populate the nodes
    input.lines().for_each(|l| {
        let line = l.trim();
        if !line.is_empty()
            && let Some((left, right)) = line.split_once(": ")
        {
            set_id(left);
            right.split_whitespace().for_each(|r| {
                set_id(r);
            })
        }
    });

    let mut graph: Graph = vec![vec![]; names.len()];

    // Second pass to populate graph
    input.lines().for_each(|l| {
        let line = l.trim();
        if !line.is_empty()
            && let Some((left, right)) = line.split_once(": ")
        {
            let x = id_map[left];
            right.split_whitespace().for_each(|r| {
                let y = id_map[r];
                graph[x].push(y);
            })
        }
    });

    (graph, id_map)
}

fn count_paths(
    graph: &Graph,
    start: usize,
    end: usize,
    skip: Option<usize>,
    memo: &mut Vec<Option<u64>>,
) -> u64 {
    // Base case
    if start == end {
        return 1;
    }

    // DAG memoization
    if let Some(cached) = memo[start] {
        return cached;
    }

    let mut total = 0;

    for &v in &graph[start] {
        if Some(v) == skip {
            continue;
        }
        total += count_paths(graph, v, end, skip, memo)
    }
    memo[start] = Some(total);
    total
}

fn all_paths_containing_two_nodes(
    graph: &Graph,
    start: usize,
    end: usize,
    must_visit1: usize,
    must_visit2: usize,
) -> u64 {
    // start -> must_visit1 -> must_visit2 -> end
    let a = count_paths(
        graph,
        start,
        must_visit1,
        Some(must_visit2),
        &mut vec![None; graph.len()],
    );
    let b = count_paths(
        graph,
        must_visit1,
        must_visit2,
        None,
        &mut vec![None; graph.len()],
    );
    let c = count_paths(
        graph,
        must_visit2,
        end,
        Some(must_visit1),
        &mut vec![None; graph.len()],
    );

    // start -> must_visit2 -> must_visit1 -> end
    let d = count_paths(
        graph,
        start,
        must_visit2,
        Some(must_visit1),
        &mut vec![None; graph.len()],
    );
    let e = count_paths(
        graph,
        must_visit2,
        must_visit1,
        None,
        &mut vec![None; graph.len()],
    );
    let f = count_paths(
        graph,
        must_visit1,
        end,
        Some(must_visit2),
        &mut vec![None; graph.len()],
    );

    a * b * c + d * e * f
}

pub fn part_one(input: &str) -> Option<u64> {
    let (graph, id_map) = parse_input(input);
    let start = id_map["you"];
    let end = id_map["out"];
    Some(count_paths(
        &graph,
        start,
        end,
        None,
        &mut vec![None; graph.len()],
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (graph, id_map) = parse_input(input);
    let start = id_map["svr"];
    let end = id_map["out"];
    let fft = id_map["fft"];
    let dac = id_map["dac"];
    Some(all_paths_containing_two_nodes(&graph, start, end, fft, dac))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
