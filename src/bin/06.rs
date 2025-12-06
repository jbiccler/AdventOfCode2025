advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
struct Instruction {
    data: Vec<u128>,
    op: Operation,
}
impl Instruction {
    fn default(data: Vec<u128>) -> Self {
        Self {
            data,
            op: Operation::Add,
        }
    }
    fn calculate_value(&self) -> u128 {
        match self.op {
            Operation::Add => self.data.iter().sum::<u128>(),
            Operation::Mul => self.data.iter().product::<u128>(),
        }
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<usize>, usize) {
    let n_lines = input.lines().count();
    let max_line_length = input.lines().map(|l| l.len()).max().unwrap();
    // Get indices where new column starts == where operator is in the last row
    let col_idx = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == '*' || c == '+' { Some(i) } else { None })
        .chain(vec![max_line_length; 1])
        .collect::<Vec<usize>>();
    // Collect input into a vector or strings per row
    let input = input.lines().collect::<Vec<&str>>();
    (input, col_idx, n_lines)
}

pub fn part_one(input: &str) -> Option<u128> {
    let (input, col_idx, n_lines) = parse_input(input);
    let mut ins = Instruction::default(vec![]);
    let mut total = 0;
    for w in col_idx.windows(2) {
        let (start, end) = (w[0], w[1]);
        // New column so reset ins.
        ins.clear();
        // Set the operator based on the last row of input
        match &input[n_lines - 1][start..=start] {
            "+" => ins.op = Operation::Add,
            "*" => ins.op = Operation::Mul,
            _ => {}
        }
        // Collect digits
        for &row in input[0..n_lines - 1].iter() {
            let stop = end.min(row.len());
            let c = row[start..stop].trim().parse::<u128>().unwrap();
            ins.data.push(c);
        }
        total += ins.calculate_value();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u128> {
    let (input, col_idx, n_lines) = parse_input(input);
    let mut ins = Instruction::default(vec![]);
    let mut total = 0;
    for w in col_idx.windows(2) {
        let (start, end) = (w[0], w[1]);
        // New column so reset ins.
        ins.clear();
        // Set the operator based on the last row of input
        match &input[n_lines - 1][start..=start] {
            "+" => ins.op = Operation::Add,
            "*" => ins.op = Operation::Mul,
            _ => {}
        }
        // Collect digits
        for c in start..end {
            let mut num = 0;
            for row in input[0..n_lines - 1].iter() {
                if c < row.len() {
                    let val = &row[c..=c];
                    if val != " " {
                        num *= 10;
                        num += val.parse::<u128>().unwrap();
                    }
                }
            }
            if num > 0 {
                ins.data.push(num);
            }
        }
        total += ins.calculate_value();
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
