use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::{collections::HashMap, io::BufRead};

#[derive(Clone, Debug)]
pub struct Bag {
    pub color: String,
    pub amount: u32,
}

fn read_input() -> Vec<String> {
    let file = File::open("./input/7.input").unwrap();
    let line_reader = BufReader::new(file);
    line_reader.lines().map(|f| f.unwrap()).collect::<Vec<_>>()
}

pub fn part_i() -> usize {
    let mut bags: HashMap<String, Vec<Bag>> = HashMap::new();
    let input = read_input();
    bags = input.iter().fold(bags, move |mut init, i| {
        let bb = parse_bags(i);
        let (x, xs) = bb.split_first().unwrap();
        init.insert(x.color.clone(), xs.iter().cloned().collect());
        init
    });
    bags.iter()
        .map(|(k, _v)| search(k, &bags))
        .filter(|&a| a)
        .count()
}

pub fn part_ii() -> u32 {
    let mut bags: HashMap<String, Vec<Bag>> = HashMap::new();
    let input = read_input();
    bags = input.iter().fold(bags, move |mut init, i| {
        let bb = parse_bags(i);
        let (x, xs) = bb.split_first().unwrap();
        init.insert(x.color.clone(), xs.iter().cloned().collect());
        init
    });
    count("shiny gold bag", &bags)
}

pub fn search(a: &str, bags: &HashMap<String, Vec<Bag>>) -> bool {
    //dbg!(a);
    if bags.get(a).unwrap().is_empty() {
        false
    } else {
        match bags
            .get(a)
            .unwrap()
            .iter()
            .find(|&b| b.color.contains("shiny gold"))
        {
            Some(_b) => true,
            None => bags
                .get(a)
                .unwrap()
                .iter()
                .map(|b| search(&b.color, bags))
                .fold(false, |init, i| init || i),
        }
    }
}

pub fn count(a: &str, bags: &HashMap<String, Vec<Bag>>) -> u32 {
    if bags.get(a).unwrap().is_empty() {
        dbg!(a, 0);
        0
    } else {
        let res = bags
            .get(a)
            .unwrap()
            .iter()
            .map(|b| b.amount * (count(&b.color, bags) + 1))
            .fold(0, |init, f| init + f);
        dbg!(a, res);
        res
    }
}

fn parse_bags(inp: &str) -> Vec<Bag> {
    let mut result = Vec::new();
    let first = inp
        .split("contain")
        .next()
        .map(|s| s.trim().trim_end_matches("s"))
        .unwrap();
    result.push(Bag {
        color: first.to_owned(),
        amount: 0,
    });
    let right = inp
        .split("contain")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|b| {
            if b.contains("no other bags") {
                None
            } else {
                let (amount, color) = b.trim().split_at(1);
                Some(Bag {
                    amount: u32::from_str(amount).unwrap_or(0),
                    color: color
                        .trim()
                        .trim_end_matches(".")
                        .trim_end_matches("s")
                        .to_owned(),
                })
            }
        })
        .collect::<Vec<Option<Bag>>>();
    result.append(
        &mut right
            .iter()
            .filter_map(|s| s.as_ref())
            .cloned()
            .collect::<Vec<Bag>>(),
    );
    //println!("{:?}", result);
    result
}
