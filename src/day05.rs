use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_input() -> Vec<String> {
    let file = File::open("./input/5.input").unwrap();
    let line_reader = BufReader::new(file);
    line_reader.lines().map(|f| f.unwrap()).collect::<Vec<_>>()
}

pub fn get_next(directive: char, section: (u32, u32)) -> (u32, u32) {
    let (first, second) = section;
    match directive {
        'B' | 'R' => ((first + second + 1) / 2, second),
        'F' | 'L' => (first, (first + second - 1) / 2),
        _ => (first, second),
    }
}
pub fn get_seat_ids() -> Vec<u32> {
    let input = read_input();
    input
        .iter()
        .map(|l| {
            let directives = l[..].split_at(7);
            let row = directives
                .0
                .chars()
                .fold((0, 127), |init, f| get_next(f, init))
                .0;
            let column = directives
                .1
                .chars()
                .fold((0, 7), |init, d| get_next(d, init))
                .0;
            row * 8 + column
        })
        .collect()
}
pub fn part_i() -> u32 {
    get_seat_ids()
        .iter()
        .fold(0, |init, &hash| if hash > init { hash } else { init })
}

pub fn part_ii() -> u32 {
    let mut seat_ids = get_seat_ids();
    seat_ids.sort();
    seat_ids
        .iter()
        .fold(seat_ids.iter().next().unwrap() - 1, |init, &hash| {
            if hash - init == 1 {
                hash
            } else {
                init
            }
        })
        + 1
}
