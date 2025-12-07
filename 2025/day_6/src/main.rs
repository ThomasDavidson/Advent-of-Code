use library::input::{Day, InputType};

struct Worksheet {
    problems: Vec<Problem>,
}
impl Worksheet {
    fn parse(input: &str) -> Self {
        let input_parts: Vec<Vec<&str>> = input
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect();

        let mut problems: Vec<Problem> = Vec::new();

        let width = input_parts[0].len();
        let height = input_parts.len();

        for x in 0..width {
            let column: Vec<&str> = (0..height).map(|y| input_parts[y][x]).collect();

            let problem = Problem::parse_str(&column);
            problems.push(problem)
        }

        Self { problems }
    }
    fn parse_part_2(input: &str) -> Self {
        let char_lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let width = char_lines[0].len();
        let height = char_lines.len();

        let mut problems: Vec<Problem> = Vec::new();
        let mut problem_str: Vec<String> = Vec::new();
        let mut operator = ' ';

        for x in 0..width {
            let slice: Vec<char> = char_lines[0..height].iter().map(|line| line[x]).collect();

            if slice.iter().all(|c| *c == ' ') {
                problem_str.push(operator.to_string());

                let problem = Problem::parse_string(&problem_str);
                problems.push(problem);
                problem_str = Vec::new();
                operator = ' ';
                continue;
            }

            let operand: String = slice[..slice.len()]
                .iter()
                .filter(|c| c.is_numeric())
                .collect();
            problem_str.push(operand);

            if slice[slice.len() - 1] != ' ' {
                operator = slice[slice.len() - 1];
            }
        }

        problem_str.push(operator.to_string());

        let problem = Problem::parse_string(&problem_str);
        problems.push(problem);

        Self { problems }
    }
}

#[derive(Debug)]
struct Problem {
    operator: Operator,
    operands: Vec<u64>,
}
impl Problem {
    fn parse_str(input_column: &[&str]) -> Self {
        let len = input_column.len();

        let operands = input_column[0..len - 1]
            .iter()
            .map(|&s| s.parse().unwrap())
            .collect();

        let operator = Operator::parse(input_column[len - 1]);

        Self { operands, operator }
    }
    fn parse_string(input_column: &[String]) -> Self {
        let len = input_column.len();

        let operands = input_column[0..len - 1]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        let operator = Operator::parse(&input_column[len - 1]);

        Self { operands, operator }
    }

    fn complete_problem(&self) -> u64 {
        self.operator.compute(&self.operands)
    }
}

#[derive(Debug)]
enum Operator {
    Multiply,
    Add,
}
impl Operator {
    fn parse(string: &str) -> Self {
        match string {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            a => panic!("Unknown operator: {a}"),
        }
    }
    fn compute(&self, ops: &[u64]) -> u64 {
        match self {
            Self::Add => ops.iter().fold(0, |acc, x| acc + x),
            Self::Multiply => ops.iter().fold(1, |acc, x| acc * x),
        }
    }
}

struct Day6;
const DAY: Day6 = Day6;
impl Day<u64> for Day6 {
    fn part_1(&self, input: &str) -> u64 {
        let worksheet = Worksheet::parse(input);

        worksheet
            .problems
            .iter()
            .map(Problem::complete_problem)
            .sum()
    }
    fn part_2(&self, input: &str) -> u64 {
        let worksheet = Worksheet::parse_part_2(input);

        worksheet
            .problems
            .iter()
            .map(Problem::complete_problem)
            .sum()
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
