#![allow(dead_code)]

mod tests;
mod problem;

use std::fs;

fn read_all_lines(path : String) -> String
{
    let contents : String = fs::read_to_string(path).expect("Should have been able to read the file");
    return contents;
}

fn main()
{
    let file_path : String = String::from("./input.txt");
    let file_contents = read_all_lines(file_path);

    let part1 = problem::get_num_stones(&file_contents, 25);
    let part2 = problem::get_num_stones_cheat_sheets(&file_contents, 75);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

