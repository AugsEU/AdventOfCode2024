use std::ops::Index;
use std::str::FromStr;

use fxhash::FxHashSet;
use fxhash::FxHashMap;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

pub fn compute_answer(input: &String) -> u128
{
    let (available_blocks, desired_patterns) = input.split_once("\n\n").unwrap();

    let available_blocks : Vec<String> = available_blocks.split(", ").map(|s| String::from_str(s).unwrap()).collect();
    let desired_patterns: Vec<String> = desired_patterns.split('\n').map(|s| String::from_str(s).unwrap()).collect();

    let mut total_ways_possible = 0;
    for desired_pattern in desired_patterns.iter()
    {
        let num_ways_possible = pattern_possible_count(desired_pattern, &available_blocks);
        if num_ways_possible > 0
        {
            total_ways_possible += num_ways_possible;
        }

        //println!("{} -> {}", desired_pattern, num_ways_possible);
    }

    return total_ways_possible;
}

fn pattern_possible_count(desired: &String, available_blocks : &Vec<String>) -> u128
{
    let mut culled_blocks : Vec<&str> = Vec::new();

    for block in available_blocks
    {
        if desired.contains(block)
        {
            culled_blocks.push(block.as_str());
        }
    }

    let mut count_cache: FxHashMap<usize, u128> = FxHashMap::default();
    let num_matches = count_patterns(&desired, &culled_blocks, &mut count_cache);

    return num_matches;
}

fn count_patterns(desired_remain: &str, available_blocks : &Vec<&str>, count_cache: &mut FxHashMap<usize, u128>) -> u128
{
    assert!(desired_remain.len() != 0);

    // Search cache for answer.
    if let Some(cached_count) = count_cache.get(&desired_remain.len())
    {
        return *cached_count;
    }

    let mut total_matches = 0;
    for block in available_blocks.iter()
    {
        let block = *block;

        // Too big or not the right start
        if block.len() > desired_remain.len() || !desired_remain.starts_with(block)
        {
            continue;
        }

        // Found exact match.
        if block == desired_remain
        {
            total_matches += 1;
        }
        else // Or search deeper
        {
            let chopped_desire = &desired_remain[block.len()..];
            total_matches += count_patterns(chopped_desire, available_blocks, count_cache);   
        }
    }

    count_cache.insert(desired_remain.len(), total_matches);

    return total_matches;
}