use std::str::FromStr;

use fxhash::FxHashSet;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

pub fn compute_answer(input: &String) -> i32
{
    let (available_blocks, desired_patterns) = input.split_once("\n\n").unwrap();

    let available_blocks : Vec<String> = available_blocks.split(", ").map(|s| String::from_str(s).unwrap()).collect();
    let desired_patterns: Vec<String> = desired_patterns.split('\n').map(|s| String::from_str(s).unwrap()).collect();

    let mut num_possible = 0;
    for desired_pattern in desired_patterns.iter()
    {
        if pattern_possible(desired_pattern, &available_blocks)
        {
            num_possible += 1;
        }
    }

    return num_possible;
}

fn pattern_possible(desired: &String, available_blocks : &Vec<String>) -> bool
{
    return search_for_pattern(String::default(), desired, available_blocks);
}

fn search_for_pattern(curr: String, desired: &String, available_blocks : &Vec<String>) -> bool
{
    if curr == *desired
    {
        return true;
    }

    for block in available_blocks.iter()
    {
        let block = block.clone();
        let new_string = curr.clone() + &block;

        if desired.starts_with(&new_string)
        {
            let found = search_for_pattern(new_string, desired, available_blocks);
            if found
            {
                return true;
            }
        }
    }

    return false;
}

