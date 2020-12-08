use std::io::BufRead;
use std::io::BufReader;
use std::{collections::HashSet, fs::File};

fn read_input() -> Vec<String> {
    let file = File::open("./input/6.input").unwrap();
    let line_reader = BufReader::new(file);
    line_reader.lines().map(|f| f.unwrap()).collect::<Vec<_>>()
}

fn common(
    init: impl Fn(&[String]) -> HashSet<char>,
    folder: impl Fn(HashSet<char>, &str) -> HashSet<char>,
) -> usize {
    let input = read_input();
    input
        .as_slice()
        .split(|s| s.trim().is_empty())
        .map(|lines| {
            lines
                .iter()
                .fold(
                    init(lines),
                    |set: HashSet<char>, a: &String| -> HashSet<char> { folder(set, a) },
                )
                .len()
        })
        .fold(0, |init, l| init + l)
}

pub fn part_i() -> usize {
    fn folder(a: HashSet<char>, b: &str) -> HashSet<char> {
        a.union(&b.chars().collect::<HashSet<char>>())
            .cloned()
            .collect::<HashSet<char>>()
    }

    fn init(_a: &[String]) -> HashSet<char> {
        HashSet::new()
    }
    common(init, folder)
}

pub fn part_ii() -> usize {
    fn folder(a: HashSet<char>, b: &str) -> HashSet<char> {
        a.intersection(&b.chars().collect::<HashSet<char>>())
            .cloned()
            .collect::<HashSet<char>>()
    }

    fn init(a: &[String]) -> HashSet<char> {
        a.iter().next().unwrap().chars().collect::<HashSet<char>>()
    }

    common(init, folder)
}
