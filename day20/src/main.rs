#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod tests;
mod part1;
mod part2;
mod aoc_utils;
mod char_grid;
mod int_grid;
mod direction;

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

    // let part1 = part1::compute_answer(&file_contents, 100);
    // println!("Part 1: {}", part1);
    
    // Do part 1 using part2's logic as it is faster.
    let part1 = part2::compute_answer(&file_contents, 100, 2);
    println!("Part 1: {}", part1);

    let part2 = part2::compute_answer(&file_contents, 100, 20);
    println!("Part 2: {}", part2);
}