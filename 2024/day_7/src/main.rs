use std::{slice::Iter, time::Instant};

enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
            Self::Concat => {
                let len: u32 = format!("{rhs}").len() as u32;
                lhs * 10_u64.pow(len) + rhs
            }
        }
    }
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}
impl Equation {
    fn from_line(line: &str) -> Self {
        let (test_value_str, operands_str) = line.split_once(": ").unwrap();

        let Some(test_value) = test_value_str.parse().ok() else {
            panic!("Operand cannot be parsed");
        };

        let operands = operands_str
            .split(" ")
            .map(|operand_str| operand_str.parse().unwrap())
            .collect();

        Self {
            test_value,
            operands,
        }
    }

    fn check_equation(&self, operators: &Vec<Operator>) -> bool {
        let mut operands = self.operands.iter();

        let curr = *operands.next().unwrap();

        check_equation(self.test_value, curr, operands, operators)
    }
}

fn check_equation(
    expected: u64,
    curr: u64,
    mut operands: Iter<'_, u64>,
    operators: &Vec<Operator>,
) -> bool {
    let Some(next) = operands.next() else {
        return expected == curr;
    };

    for operator in operators {
        if check_equation(
            expected,
            operator.apply(curr, *next),
            operands.clone(),
            operators,
        ) {
            return true;
        }
    }

    return false;
}

struct Calibrations {
    equations: Vec<Equation>,
    operators: Vec<Operator>,
}

impl Calibrations {
    fn from_input(input: &str, operators: Vec<Operator>) -> Self {
        let equations = input
            .lines()
            .map(|line| Equation::from_line(line))
            .collect();
        Self {
            equations,
            operators,
        }
    }
}

fn part_1(input: &str) -> u64 {
    let operators = vec![Operator::Add, Operator::Mul];
    let calibrations = Calibrations::from_input(input, operators);

    calibrations
        .equations
        .iter()
        .filter(|eq| eq.check_equation(&calibrations.operators))
        .fold(0, |acc, eq| acc + eq.test_value)
}

fn part_2(input: &str) -> u64 {
    let operators = vec![Operator::Add, Operator::Mul, Operator::Concat];
    let calibrations = Calibrations::from_input(input, operators);

    calibrations
        .equations
        .iter()
        .filter(|eq| eq.check_equation(&calibrations.operators))
        .fold(0, |acc, eq| acc + eq.test_value)
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(&input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(&input);
    let duration = start.elapsed();
    println!("Part 2 answer: {}, time: {:?}", part_2_answer, duration);
}

#[cfg(test)]
mod tests {
    use crate::Operator;

    #[test]
    fn test1() {
        let result = Operator::Concat.apply(10, 10);
        assert_eq!(result, 1010);
    }

    #[test]
    fn test2() {
        let result = Operator::Concat.apply(91, 19);
        assert_eq!(result, 9119);
    }
    #[test]
    fn test3() {
        let result = Operator::Concat.apply(100, 123456789);
        assert_eq!(result, 100123456789);
    }
    #[test]
    fn test4() {
        let result = Operator::Concat.apply(77, 88);
        assert_eq!(result, 7788);
    }
}
