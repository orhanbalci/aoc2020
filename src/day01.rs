use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part_i() -> u32 {
    let mut sorted = read_input();
    sorted.sort();
    sorted
        .iter()
        .enumerate()
        .map(|(index, &val)| {
            sorted
                .iter()
                .skip(index)
                .find(|&&d| d + val == 2020)
                .map(|&d| d * val)
        })
        .find(|k| k.is_some())
        .unwrap()
        .unwrap()
}

pub fn part_ii() -> u32 {
    let mut input = read_input();
    input.sort();
    input
        .iter()
        .enumerate()
        .map(|(index, &val)| {
            input
                .iter()
                .enumerate()
                .skip(index)
                .map(|(index2, &val2)| {
                    input
                        .iter()
                        .enumerate()
                        .skip(index2)
                        .find(|(_index3, &val3)| val + val2 + val3 == 2020)
                        .map(|(_index3, &val3)| val * val2 * val3)
                })
                .collect::<Vec<Option<u32>>>()
                .into_iter()
                .find(|k| k.is_some())
        })
        .find(|k| k.is_some())
        .unwrap()
        .unwrap()
        .unwrap()
}

fn read_input() -> Vec<u32> {
    let file = File::open("./input/1.input").unwrap();
    let line_reader = BufReader::new(file);
    line_reader
        .lines()
        .into_iter()
        .map(|f| f.map(|x| x.parse::<u32>().unwrap()).unwrap())
        .collect::<Vec<_>>()
}
