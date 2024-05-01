use std::ops::Range;

#[derive(Debug, Clone, Eq, PartialEq)]
enum WorkflowResult {
    Accept(bool),
    Workflow(String),
}
impl WorkflowResult {
    fn parse(str: &str) -> Self {
        match str {
            "A" => WorkflowResult::Accept(true),
            "R" => WorkflowResult::Accept(false),
            _ => WorkflowResult::Workflow(str.to_string()),
        }
    }
}

#[derive(Debug)]
enum WorkflowCmp {
    Fallthrough,
    Threshold(usize, char, bool),
}
impl WorkflowCmp {
    // pass in a string that has the pattern catagory then < or > then number
    // ie a<2006
    fn parse(str: &str) -> Self {
        let greater: bool = str.contains("<");

        let (catagory_str, threshold_str) = match greater {
            true => str.split_once("<").unwrap(),
            false => str.split_once(">").unwrap(),
        };

        let threshold = threshold_str.parse().unwrap();
        let catagory = catagory_str.chars().nth(0).unwrap();

        WorkflowCmp::Threshold(threshold, catagory, greater)
    }
}

#[derive(Debug)]
struct WorkflowRule {
    result: WorkflowResult,
    rule: WorkflowCmp,
}
#[derive(Debug)]
struct Workflow {
    label: String,
    workflow_rule: Vec<WorkflowRule>,
}

fn parse_workflows(workflows_str: &str) -> Vec<Workflow> {
    let mut workflows: Vec<Workflow> = Vec::new();

    for line in workflows_str.lines() {
        let mut split = line.split(['{', '}', ',']).filter(|a| !a.is_empty());
        // label is allways the first split item
        let label = split.next().unwrap();
        let mut workflow = Workflow {
            label: label.to_string(),
            workflow_rule: Vec::new(),
        };

        for rule in split {
            let rule_split = rule.split(":").collect::<Vec<&str>>();

            match rule_split.len() {
                // Fallthrough
                1 => workflow.workflow_rule.push(WorkflowRule {
                    result: WorkflowResult::parse(rule_split[0]),
                    rule: WorkflowCmp::Fallthrough,
                }),
                // Threshold
                2 => workflow.workflow_rule.push(WorkflowRule {
                    result: WorkflowResult::parse(rule_split[1]),
                    rule: WorkflowCmp::parse(rule_split[0]),
                }),
                _ => (),
            }
        }
        workflows.push(workflow);
    }
    workflows
}

#[derive(Debug, Clone)]
struct Ratings {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl Ratings {
    fn parse(str: &str) -> Ratings {
        let str: Vec<&str> = str
            .split(['{', '}', ','])
            .filter(|&s| s.contains("="))
            .collect();

        let mut ratings = Ratings {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for (i, rating) in str.iter().enumerate() {
            let rating: usize = rating[2..].parse().unwrap();
            match i {
                0 => ratings.x = rating,
                1 => ratings.m = rating,
                2 => ratings.a = rating,
                3 => ratings.s = rating,
                a => panic!("parse not expected {a} {rating}"),
            }
        }
        ratings
    }
    fn get_value(&self, c: char) -> usize {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("invalid char {c}"),
        }
    }
    fn get_sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn parse_ratings_input(ratings_str: &str) -> Vec<Ratings> {
    ratings_str
        .lines()
        .map(|line| Ratings::parse(line))
        .collect()
}

fn check_machine_part(rating: &Ratings, workflows: &Vec<Workflow>) -> bool {
    let mut workflow_result: WorkflowResult = WorkflowResult::Workflow("in".to_string());

    while !(workflow_result == WorkflowResult::Accept(true)
        || workflow_result == WorkflowResult::Accept(false))
    {
        let Some(selected_workflow) = workflows
            .iter()
            .find(|&a| WorkflowResult::Workflow(a.label.clone()) == workflow_result)
        else {
            panic!("label not found");
        };

        for rule in &selected_workflow.workflow_rule {
            let result = match rule.rule {
                WorkflowCmp::Fallthrough => true,
                WorkflowCmp::Threshold(threshold, catagory, greater) => match greater {
                    true => rating.get_value(catagory) < threshold,
                    false => rating.get_value(catagory) > threshold,
                },
            };
            match (result, rule.result.clone()) {
                (true, _) => {
                    workflow_result = rule.result.clone();
                    break;
                }
                (false, _) => (),
            }
        }
    }
    match workflow_result {
        WorkflowResult::Accept(true) => return true,
        _ => return false,
    }
}

fn check_catagory_range(
    range: Range<usize>,
    range_catagory: char,
    workflows: &Vec<Workflow>,
    workflow_result: &WorkflowResult,
) -> Vec<usize> {
    let mut accepted_scores: Vec<usize> = Vec::new();

    let Some(selected_workflow) = workflows
        .iter()
        .find(|&a| &WorkflowResult::Workflow(a.label.clone()) == workflow_result)
    else {
        panic!("label not found");
    };

    let mut current_range = range.clone();

    for rule in &selected_workflow.workflow_rule {
        let (mut result_range, next_range): (Vec<usize>, Range<usize>) = match rule.rule {
            WorkflowCmp::Fallthrough => (current_range.clone().collect(), 0..0),
            WorkflowCmp::Threshold(threshold, catagory, greater) => {
                if catagory != range_catagory && !current_range.contains(&threshold) {
                    continue;
                }
                // println!("{:?}", current_range);

                let max_range = current_range.clone().max().unwrap();
                let min_range = current_range.clone().min().unwrap();

                let (accepted_range, regected_range) = match greater {
                    // only used Range to match arms
                    true => ((threshold + 1)..(max_range + 1), min_range..(threshold + 1)),
                    false => (min_range..threshold, threshold..(max_range + 1)),
                };
                let accepted_vec = match rule.result {
                    // todo add other range to accept condition
                    WorkflowResult::Accept(true) => {
                        println!("Accept");
                        accepted_range.collect()
                    }
                    WorkflowResult::Accept(false) => {
                        println!("Reject");
                        accepted_range.collect()
                    }
                    WorkflowResult::Workflow(_) => check_catagory_range(
                        accepted_range,
                        range_catagory,
                        workflows,
                        &rule.result,
                    ),
                };
                (accepted_vec, regected_range)
            }
        };
        println!("len {:?} {:?}", result_range.len(), rule);
        accepted_scores.append(&mut result_range);
        // match (&rule.result, &rule.rule) {
        //     (WorkflowResult::Accept(true), WorkflowCmp::Fallthrough) => {
        //         accepted_scores.append(&mut result_range);
        //     }
        //     (_, _) => (),
        // }
        current_range = next_range;
    }

    println!("{:?} => {}", range, accepted_scores.len());
    accepted_scores
}

fn part_1(ratings: &Vec<Ratings>, workflows: &Vec<Workflow>) -> usize {
    ratings
        .iter()
        .map(|rating| match check_machine_part(rating, workflows) {
            false => 0,
            true => rating.get_sum(),
        })
        .fold(0, |acc, res| acc + res)
}

fn part_2(workflows: &Vec<Workflow>) -> usize {
    let range = 1..(4000 + 1);

    let initial = WorkflowResult::Workflow("in".to_string());

    let accepted_x = check_catagory_range(range.clone(), 'x', workflows, &initial).len();
    let accepted_m = check_catagory_range(range.clone(), 'm', workflows, &initial).len();
    let accepted_a = check_catagory_range(range.clone(), 'a', workflows, &initial).len();
    let accepted_s = check_catagory_range(range.clone(), 's', workflows, &initial).len();

    println!("accepted_x {accepted_x} accepted_m {accepted_m} accepted_a {accepted_a} accepted_s {accepted_s}");

    vec![accepted_x, accepted_m, accepted_a, accepted_s]
        .iter()
        .fold(0, |acc, i| acc + i)
}

fn main() {
    let input = include_str!("../example.txt");

    let (workflows_str, ratings_str) = input.split_once("\r\n\r\n").unwrap();

    let workflows = parse_workflows(workflows_str);

    // for workflow in &workflows {
    //     println!("{:#?}", workflow);
    // }

    let ratings = parse_ratings_input(ratings_str);

    let part_1_score = part_1(&ratings, &workflows);
    println!("Result: {part_1_score}");

    let part_2_score = part_2(&workflows);
    println!("Result: {part_2_score}");
}
