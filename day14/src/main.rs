#![allow(dead_code)]

mod tests;
mod part1;
mod part2;
mod aoc_utils;
mod robot_room;

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

    let part1 = part1::get_robot_safety_factor(&file_contents, 101, 103);
    let part2 = part2::step_until_tree(&file_contents, 101, 103);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}