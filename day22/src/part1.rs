use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

const NUM_ITER : u64 = 2000;

pub fn compute_answer(input: &String) -> u64
{
    return input.lines().filter_map(|s| s.parse::<u64>().ok()).map(|n| compute_nth_random_number(n, NUM_ITER)).sum();
}

fn compute_nth_random_number(seed: u64, iter: u64) -> u64
{
    let mut result = seed;
    for _ in 0..iter
    {
        result = next_random_number(result);
    }

    return result;
}

fn next_random_number(seed: u64) -> u64
{
    let step1 = prune(mix(seed, seed * 64));
    let step2 = prune(mix(step1, step1 / 32));
    let step3 = prune(mix(step2, step2 * 2048));
    return step3;
}

fn mix(n1: u64, n2: u64) -> u64
{
    return n1 ^ n2;
}

fn prune(n: u64) -> u64
{
    return n % 16777216;
}