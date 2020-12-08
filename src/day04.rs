use lazy_static::lazy_static;
use regex::Regex;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::{collections::HashSet, fs::File};

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"#([a-f]|\d){6,6}").unwrap();
    static ref ECL_REGEX: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)+$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^(\d){9,9}+$").unwrap();
}

fn read_input() -> Vec<String> {
    let file = File::open("./input/4.input").unwrap();
    let line_reader = BufReader::new(file);
    line_reader.lines().map(|f| f.unwrap()).collect::<Vec<_>>()
}

pub fn part_i_filter(_kv: &[String]) -> bool {
    true
}

pub fn part_ii_filter(kv: &[String]) -> bool {
    match kv[0].as_str() {
        "byr" => byr_validate(kv[1].trim()),
        "iyr" => iyr_validate(kv[1].trim()),
        "eyr" => eyr_validate(kv[1].trim()),
        "hgt" => hgt_validate(kv[1].trim()),
        "hcl" => hcl_validate(kv[1].trim()),
        "ecl" => ecl_validate(kv[1].trim()),
        "pid" => pid_validate(kv[1].trim()),
        _ => true,
    }
}

pub fn part_i() -> usize {
    common(part_i_filter)
}

pub fn part_ii() -> usize {
    common(part_ii_filter)
}
pub fn common(filter: impl Fn(&[String]) -> bool) -> usize {
    let valid_keys: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .map(|&k| k.to_owned())
        .collect();

    let input = read_input();
    input
        .as_slice()
        .split(|s| s.trim().is_empty())
        .map(|lines| {
            valid_keys
                .difference(
                    &lines
                        .join(" ")
                        .split_whitespace()
                        .map(|kv| kv.split(':').map(|s| s.to_owned()).collect::<Vec<_>>())
                        .filter(|info| filter(info))
                        .map(|k| k.iter().next().unwrap().to_owned())
                        .collect::<HashSet<_>>(),
                )
                .filter(|&k| k != "cid")
                .count()
                == 0
        })
        .filter(|&s| s == true)
        .count()
}

pub fn year_validate(inp: &str, lower_bound: u32, upper_bound: u32) -> bool {
    lower_bound <= u32::from_str(inp).unwrap() && u32::from_str(inp).unwrap() <= upper_bound
}

pub fn byr_validate(inp: &str) -> bool {
    year_validate(inp, 1920, 2002)
}

pub fn iyr_validate(inp: &str) -> bool {
    year_validate(inp, 2010, 2020)
}

pub fn eyr_validate(inp: &str) -> bool {
    year_validate(inp, 2020, 2030)
}

pub fn hgt_validate(inp: &str) -> bool {
    let captures = HEIGHT_REGEX.captures(inp);
    match captures {
        None => false,
        Some(s) => {
            if s.len() != 3 {
                false
            } else {
                let num = &s[1];
                let unit = &s[2];
                match unit {
                    "in" => year_validate(num, 59, 76),
                    "cm" => year_validate(num, 150, 193),
                    _ => false,
                }
            }
        }
    }
}

pub fn hcl_validate(inp: &str) -> bool {
    match HCL_REGEX.captures(inp) {
        Some(s) => {
            //println!("hcl {} {}", inp, s.len());
            s.len() == 2
        }
        None => false,
    }
}

pub fn ecl_validate(inp: &str) -> bool {
    match ECL_REGEX.captures(inp) {
        Some(s) => {
            // println!("ecl {} {}", inp, s.len());
            s.len() == 2
        }
        None => false,
    }
}

pub fn pid_validate(inp: &str) -> bool {
    match PID_REGEX.captures(inp) {
        Some(s) => {
            // println!("pid {} {}", inp, s.len());
            s.len() == 2
        }
        None => false,
    }
}
