advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .trim()
            .split("\n\n")
            .last()
            .unwrap()
            .lines()
            .filter(|l| {
                let (wh, rest) = l.split_once(": ").unwrap();
                let (w, h) = wh.split_once("x").unwrap();
                let (w, h) = (w.parse::<u64>().unwrap(), h.parse::<u64>().unwrap());
                let n = rest
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .sum();
                w * h / 9 >= n
            })
            .count(),
    )
}

#[allow(unused)]
pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
