pub mod day01;
pub mod day02;
pub mod day03;

fn main() {
    let result_day01_part_i = day01::part_ii();
    println!("Day 01 Part I {}", result_day01_part_i);
    let result_day02_part_i = day02::part_i();
    println!("Day 02 Part I {}", result_day02_part_i);
    let result_day02_part_ii = day02::part_ii();
    println!("Day 02 Part II {}", result_day02_part_ii);
    let result_day03_part_i = day03::part_i();
    println!("Day 03 Part I {}", result_day03_part_i);
    let result_day03_part_ii = day03::part_ii();
    println!("Day 03 Part II {}", result_day03_part_ii);
}
