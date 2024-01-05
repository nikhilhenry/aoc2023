use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use itertools::Itertools;

type Part = HashMap<char, usize>;

#[derive(Debug)]
enum Condition {
    Greater,
    Lower,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '>' => Condition::Greater,
            '<' => Condition::Lower,
            _ => panic!("Invalid char"),
        }
    }
}

#[derive(Debug, Clone)]
enum Status {
    Workflow(String),
    Accepted,
    Rejected,
}

impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "A" => Status::Accepted,
            "R" => Status::Rejected,
            _ => Status::Workflow(s.to_string()),
        })
    }
}

#[derive(Debug)]
struct Rule {
    field: char,
    condition: Condition,
    num: usize,
    status: Status,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (s, status) = s.split_once(":").ok_or(anyhow!("invalid string"))?;
        let status = status.parse()?;
        let mut rule = s.to_string();
        let field = rule.remove(0);
        let condition = rule.remove(0).into();
        let num = rule.parse()?;

        Ok(Self {
            field,
            status,
            condition,
            num,
        })
    }
}

impl Rule {
    fn process(&self, part: &Part) -> Option<Status> {
        match &self.condition {
            Condition::Greater => {
                if part.get(&self.field).unwrap() > &self.num {
                    Some(self.status.clone())
                } else {
                    None
                }
            }
            Condition::Lower => {
                if part.get(&self.field).unwrap() < &self.num {
                    Some(self.status.clone())
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    resort: Status,
    name: String,
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (name, s) = s.split_once("{").ok_or(anyhow!("invalid string"))?;
        let mut s = s.to_string();
        s.pop();
        let resort = s.clone().split(",").last().unwrap().parse()?;

        let rules = s.split(",").filter_map(|r| r.parse().ok()).collect_vec();

        Ok(Workflow {
            rules,
            resort,
            name: name.to_string(),
        })
    }
}

impl Workflow {
    fn process(&self, part: &Part) -> Status {
        let mut status = self.rules.iter().filter_map(|r| r.process(part));
        status.next().unwrap_or_else(|| self.resort.clone())
    }
}

fn main() -> Result<()> {
    let (workflows, parts) = include_str!("../../data/day19.input")
        .split_once("\n\n")
        .unwrap();
    let workflows: HashMap<String, Workflow> = workflows
        .split("\n")
        .filter_map(|w| w.parse::<Workflow>().ok())
        .map(|w| (w.name.clone(), w))
        .collect();

    let parts: Vec<Part> = parts
        .split("\n")
        .filter_map(|p| {
            let mut p = p.to_string();
            if p.is_empty() {
                return None;
            }
            p.remove(0);
            p.pop()?;
            Some(
                p.split(",")
                    .filter_map(|s| {
                        let (field, num) = s.split_once("=").unwrap();
                        let field = field.chars().next().unwrap();
                        let num = num.parse::<usize>().unwrap();
                        Some((field, num))
                    })
                    .collect(),
            )
        })
        .collect();

    let accepted = parts
        .iter()
        .filter_map(|part| {
            let mut workflow = String::from("in");
            while let Status::Workflow(name) = workflows.get(&workflow).unwrap().process(part) {
                workflow = name;
            }
            let status = workflows.get(&workflow).unwrap().process(part);
            match status {
                Status::Workflow(_) => panic!("shouldn't be here"),
                Status::Accepted => Some(part),
                Status::Rejected => None,
            }
        })
        .collect_vec();

    // compute the sum of each
    let sum: usize = accepted.iter().map(|p| p.values().sum::<usize>()).sum();
    println!("Part 1:{sum}");

    Ok(())
}
