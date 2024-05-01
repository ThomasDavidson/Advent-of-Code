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
        let mut split = line.split(['{', '}', ',']);
        let label = split.next().unwrap();
        let mut workflow = Workflow {
            label: label.to_string(),
            workflow_rule: Vec::new(),
        };

        // println!("Label: {label}");
        for rule in split {
            // println!("{rule}");
            let rule_split = rule.split(":").collect::<Vec<&str>>();
            if rule.is_empty() {
                continue;
            }

            // println!("{:?}", rule_split);
            match rule_split.len() {
                // fallthrough
                1 => workflow.workflow_rule.push(WorkflowRule {
                    result: WorkflowResult::parse(rule_split[0]),
                    rule: WorkflowCmp::Fallthrough,
                }),
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

#[derive(Debug)]
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
}

fn parse_ratings_input(ratings_str: &str) -> Vec<Ratings> {
    ratings_str
        .lines()
        .map(|line| Ratings::parse(line))
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let mut part_1_score = 0;

    let (workflows_str, ratings_str) = input.split_once("\r\n\r\n").unwrap();

    let workflows = parse_workflows(workflows_str);

    let ratings = parse_ratings_input(ratings_str);

    for rating in ratings {
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
            WorkflowResult::Accept(true) => part_1_score += rating.x + rating.m + rating.a + rating.s,
            _ => (),
        }
    }

    println!("Result: {part_1_score}");
}
