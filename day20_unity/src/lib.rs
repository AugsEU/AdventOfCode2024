mod part1;
mod part2;

use std::fs;

fn read_all_lines(path : String) -> String
{
    let contents : String = fs::read_to_string(path).expect("Should have been able to read the file");
    return contents;
}

pub fn run_part2() -> i64
{
    let file_path : String = String::from("./input.txt");
    let file_contents = read_all_lines(file_path);

    let part2 = part2::run(&file_contents);
    return part2;
}

pub fn run_part1() -> i64
{
    let file_path : String = String::from("./input.txt");
    let file_contents = read_all_lines(file_path);

    let part1 = part1::run(&file_contents);
    return part1;
}