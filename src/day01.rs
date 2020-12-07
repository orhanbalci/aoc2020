use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part_i() -> u32 {
    let mut sorted = read_input();
    sorted.sort();
    sorted
        .iter()
        .enumerate()
        .find_map(|(index, &val)| {
            sorted
                .iter()
                .skip(index)
                .find(|&&d| d + val == 2020)
                .map(|&d| d * val)
        })
        .unwrap()
}

pub fn part_ii() -> u32 {
    let mut input = read_input();
    input.sort_unstable();
    input
        .iter()
        .enumerate()
        .find_map(|(index, &val)| {
            input
                .iter()
                .enumerate()
                .skip(index)
                .filter(|(_index, &val2)| val2 + val < 2020)
                .find_map(|(index2, &val2)| {
                    input
                        .iter()
                        .enumerate()
                        .skip(index2)
                        .find(|(_index3, &val3)| val + val2 + val3 == 2020)
                        .map(|(_index3, &val3)| val * val2 * val3)
                })
        })
        .unwrap()
}

fn read_input() -> Vec<u32> {
    let file = File::open("./input/1.input").unwrap();
    let line_reader = BufReader::new(file);
    line_reader
        .lines()
        .map(|f| f.map(|t| t.parse::<u32>().unwrap()).unwrap())
        .collect::<Vec<_>>()
}
