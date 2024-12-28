use std::time::Instant;

#[derive(Debug, Clone)]
struct OrderRule {
    rule: [u32; 2],
}
impl OrderRule {
    fn from_line(line: &str) -> Self {
        let Some((first, second)) = line.split_once("|") else {
            panic!("Cannot process rule");
        };
        let Some(first) = first.parse().ok() else {
            panic!("Cannot parse first order");
        };
        let Some(second) = second.parse().ok() else {
            panic!("Cannot parse second order");
        };
        Self {
            rule: [first, second],
        }
    }
}

#[derive(Debug, Clone)]
struct PageOrder {
    order: Vec<u32>,
}
impl PageOrder {
    fn from_line(line: &str) -> Self {
        let order = line.split(",").map(|str| str.parse().unwrap()).collect();
        Self { order }
    }

    fn get_rule_idx(&self, order_rule: &OrderRule) -> Option<(usize, usize)> {
        let page_order = &self.order;
        let first = order_rule.rule[0];
        let Some(first_idx) = page_order.iter().position(|x| x == &first) else {
            return None;
        };

        let second = order_rule.rule[1];
        let Some(second_idx) = page_order.iter().position(|x| x == &second) else {
            return None;
        };

        Some((first_idx, second_idx))
    }

    fn check_rule(&self, order_rule: &OrderRule) -> bool {
        let Some((first_idx, second_idx)) = self.get_rule_idx(order_rule) else {
            return true;
        };

        return first_idx < second_idx;
    }

    fn check_order(&self, order_rules: &Vec<OrderRule>) -> bool {
        for order_rule in order_rules {
            if !self.check_rule(order_rule) {
                return false;
            }
        }
        return true;
    }
    fn fix_order(&mut self, order_rules: &Vec<OrderRule>) {
        while !self.check_order(order_rules) {
            for order_rule in order_rules {
                if !self.check_rule(order_rule) {
                    let Some((first_idx, second_idx)) = self.get_rule_idx(order_rule) else {
                        continue;
                    };
                    // print!("{:?}\t-->\t", self);
                    self.order.swap(first_idx, second_idx);
                    // println!("{:?}", self);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct PrintJob {
    order_rules: Vec<OrderRule>,
    page_orders: Vec<PageOrder>,
}

impl PrintJob {
    fn from_input(input: &str) -> Self {
        let Some((rules_str, page_orders_str)) = input.split_once("\r\n\r\n") else {
            panic!("Cannot seperate rules from page order");
        };

        let order_rules = rules_str
            .lines()
            .map(|line| OrderRule::from_line(line))
            .collect();
        let page_orders = page_orders_str
            .lines()
            .map(|line| PageOrder::from_line(line))
            .collect();

        Self {
            order_rules,
            page_orders,
        }
    }
}

fn part_1(input: &str) -> u32 {
    let print_job = PrintJob::from_input(input);

    let mut part_1_answer = 0;

    for page_order in &print_job.page_orders {
        if !page_order.check_order(&print_job.order_rules) {
            continue;
        }

        let add_idx = page_order.order.len().div_ceil(2) - 1;
        part_1_answer += page_order.order[add_idx];
    }
    part_1_answer
}

fn part_2(input: &str) -> u32 {
    let print_job = PrintJob::from_input(input);

    let mut part_2_answer = 0;

    for page_order in &print_job.page_orders {
        if page_order.check_order(&print_job.order_rules) {
            continue;
        }
        let mut page_order = page_order.clone();

        page_order.fix_order(&print_job.order_rules);

        let add_idx = page_order.order.len().div_ceil(2) - 1;
        part_2_answer += page_order.order[add_idx];
    }
    part_2_answer
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
