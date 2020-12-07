use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct PassPolicy {
    pub character: char,
    pub at_least: usize,
    pub at_most: usize,
    pub pass: String,
}

impl PassPolicy {
    pub fn valid_first_policy(&self) -> bool {
        let count = self.pass.chars().filter(|&s| s == self.character).count();
        count >= self.at_least && count <= self.at_most
    }

    pub fn valid_second_policy(&self) -> bool {
        let a = self.pass.chars().nth(self.at_least - 1).unwrap();
        let b = self.pass.chars().nth(self.at_most - 1).unwrap();
        (a == self.character && b != self.character) || (a != self.character && b == self.character)
    }
}

fn extract_pass_policy(inp: String) -> Option<PassPolicy> {
    let pass_policy_regex = Regex::new(r"(\d+).(\d+)\s(.):\s(.+)").unwrap();
    let captures = pass_policy_regex.captures(&inp).unwrap();
    if captures.len() == 5 {
        Some(PassPolicy {
            at_least: captures[1].parse::<usize>().unwrap(),
            at_most: captures[2].parse::<usize>().unwrap(),
            character: captures[3].chars().next().unwrap(),
            pass: captures[4].to_string(),
        })
    } else {
        None
    }
}

fn read_input() -> Vec<PassPolicy> {
    let file = File::open("./input/2.input").unwrap();
    let line_reader = BufReader::new(file);
    line_reader
        .lines()
        .into_iter()
        .map(|f| extract_pass_policy(f.unwrap()).unwrap())
        .collect::<Vec<_>>()
}

pub fn part_i() -> usize {
    let passwords = read_input();
    passwords.iter().filter(|&p| p.valid_first_policy()).count()
}

pub fn part_ii() -> usize {
    let passwords = read_input();
    passwords
        .iter()
        .filter(|&p| p.valid_second_policy())
        .count()
}
