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
#[derive(Debug, Clone)]
struct RatingsRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}
impl RatingsRange {
    fn get_permutations(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
    fn get_value(&self, c: char) -> Range<usize> {
        match c {
            'x' => self.x.clone(),
            'm' => self.m.clone(),
            'a' => self.a.clone(),
            's' => self.s.clone(),
            _ => panic!("invalid char {c}"),
        }
    }
    fn contains(&self, c: char, num: usize) -> bool {
        let range = self.get_value(c);
        range.contains(&num)
    }
    fn update_value(&self, c: char, new_range: Range<usize>) -> Self {
        match c {
            'x' => Self {
                x: new_range,
                ..self.clone()
            },
            'm' => Self {
                m: new_range,
                ..self.clone()
            },
            'a' => Self {
                a: new_range,
                ..self.clone()
            },
            's' => Self {
                s: new_range,
                ..self.clone()
            },
            _ => panic!("invalid char {c}"),
        }
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
    range: RatingsRange,
    workflows: &Vec<Workflow>,
    workflow_result: &WorkflowResult,
) -> usize {
    let mut result = 0;

    let Some(selected_workflow) = workflows
        .iter()
        .find(|&a| &WorkflowResult::Workflow(a.label.clone()) == workflow_result)
    else {
        panic!("label not found");
    };

    let mut current_range = range.clone();

    for rule in &selected_workflow.workflow_rule {
        let (permutations, next_range): (usize, Option<RatingsRange>) = match rule {
            WorkflowRule {
                result: WorkflowResult::Accept(true),
                rule: WorkflowCmp::Fallthrough,
            } => (current_range.get_permutations(), None),
            WorkflowRule {
                result: WorkflowResult::Accept(false),
                rule: WorkflowCmp::Fallthrough,
            } => (0, None),

            WorkflowRule {
                result: WorkflowResult::Workflow(_),
                rule: WorkflowCmp::Fallthrough,
            } => (
                check_catagory_range(current_range, workflows, &rule.result),
                None,
            ),

            WorkflowRule {
                result: _,
                rule: WorkflowCmp::Threshold(threshold, catagory, greater),
            } => {
                if !current_range.contains(*catagory, *threshold) {
                    continue;
                }
                let caragory_range = current_range.get_value(*catagory);

                let max_range = caragory_range.clone().max().unwrap();
                let min_range = caragory_range.clone().min().unwrap();

                let (accepted_range, regected_range) = match greater {
                    // only used Range to match arms
                    true => (min_range..*threshold, *threshold..(max_range + 1)),
                    false => ((threshold + 1)..(max_range + 1), min_range..(threshold + 1)),
                };

                // process the accepted range
                let accepted_result: usize = match rule.result {
                    WorkflowResult::Accept(true) => {
                        current_range
                            .update_value(*catagory, accepted_range)
                            .get_permutations()
                    }
                    WorkflowResult::Accept(false) => {
                        0
                    }
                    WorkflowResult::Workflow(_) => check_catagory_range(
                        current_range.update_value(*catagory, accepted_range),
                        workflows,
                        &rule.result,
                    ),
                };
                (
                    accepted_result,
                    Some(current_range.update_value(*catagory, regected_range)),
                )
            }
        };
        result += permutations;

        current_range = match next_range {
            None => break,
            Some(a) => a,
        };
    }

    result
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
    let range: Range<usize> = 1..(4000 + 1);

    let ratigns_range = RatingsRange {
        x: range.clone(),
        m: range.clone(),
        a: range.clone(),
        s: range.clone(),
    };

    let initial = WorkflowResult::Workflow("in".to_string());

    let part_2_answer = check_catagory_range(ratigns_range, workflows, &initial);

    part_2_answer
}

fn main() {
    let input = include_str!("../input.txt");

    let (workflows_str, ratings_str) = input.split_once("\r\n\r\n").unwrap();

    let workflows = parse_workflows(workflows_str);

    let ratings = parse_ratings_input(ratings_str);

    let part_1_score = part_1(&ratings, &workflows);
    println!("part 1 answer: {part_1_score}");

    let part_2_score = part_2(&workflows);
    println!("part 2 answer: {part_2_score}");
}
