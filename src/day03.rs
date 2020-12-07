use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_input() -> Vec<String> {
    let file = File::open("./input/3.input").unwrap();
    let line_reader = BufReader::new(file);
    line_reader.lines().map(|f| f.unwrap()).collect::<Vec<_>>()
}

pub fn part_i() -> usize {
    common(1, 3)
}

pub fn part_ii() -> usize {
    common(1, 1) * common(1, 3) * common(1, 5) * common(1, 7) * common(2, 1)
}

fn common(row_policy: usize, column_policy: usize) -> usize {
    let input = read_input();
    input
        .as_slice()
        .chunks(row_policy)
        .enumerate()
        .skip(1)
        .map(|(index, f)| {
            f[0].chars()
                .nth((column_policy * index) % input[0].len())
                .unwrap()
        })
        .filter(|&s| s == '#')
        .count()
}
