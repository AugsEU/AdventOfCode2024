use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;
use crate::money_cmds;

use rayon::prelude::*;

const NUM_ITER : usize = 2000;

pub fn compute_answer(input: &String) -> i64
{
    let all_seeds: Vec<i64> = input.lines().filter_map(|s| s.parse::<i64>().ok()).collect();
    let all_sequences: Vec<Vec<i64>> = all_seeds.iter().map(|seed| compute_random_numbers(*seed, NUM_ITER)).collect();
    let all_sequences_delta: Vec<(Vec<i64>, Vec<i64>)> = all_sequences.iter().map(|seq: &Vec<i64>| (seq.clone(), compute_delta_sequence(seq))).collect();
    

    let mut most_money = 0;
    let mut best_seq = None;
    for (i, money_cmd) in money_cmds::all_cmds_iter().enumerate()
    {
        let total_profit = all_sequences_delta.par_iter()
                        .map(|(seq, delta_seq)|
                        {
                            compute_profit(seq, delta_seq, &money_cmd)
                        })
                        .sum::<i64>();

        if total_profit > most_money
        {
            most_money = total_profit;
            best_seq = Some(money_cmd);
        }
    }

    println!("Best seq {:?}", best_seq.unwrap());
    
    return most_money;
}

fn compute_profit(seq: &Vec<i64>, delta_seq: &Vec<i64>, cmd: &[i64; 4]) -> i64
{
    for (i, window) in delta_seq.windows(4).enumerate()
    {
        if let [a, b, c, d] = window
        {
            let seq_match = *a == cmd[0] && *b == cmd[1] && *c == cmd[2] && *d == cmd[3];

            if seq_match
            {
                return seq[i+4];
            }
        }
        else
        {
            panic!("wtf rust");
        }
    }

    return 0;
}

fn compute_delta_sequence(seq: &Vec<i64>) -> Vec<i64>
{
    let mut result = Vec::new();

    for window in seq.windows(2)
    {
        if let [a, b] = window
        {
            result.push(*b - *a);
        }
        else
        {
            panic!("wtf rust");
        }
    }

    return result;
}

fn compute_random_numbers(seed: i64, size: usize) -> Vec<i64>
{
    let mut result = Vec::with_capacity(size);
    result.push(seed);

    for _ in 0..size
    {
        let curr_num = *result.last().unwrap();
        result.push(next_random_number(curr_num));
    }

    for i in 0..result.len()
    {
        result[i] = result[i] % 10;
    }

    return result;
}

fn next_random_number(seed: i64) -> i64
{
    let step1 = prune(mix(seed, seed * 64));
    let step2 = prune(mix(step1, step1 / 32));
    let step3 = prune(mix(step2, step2 * 2048));
    return step3;
}

fn mix(n1: i64, n2: i64) -> i64
{
    return n1 ^ n2;
}

fn prune(n: i64) -> i64
{
    return n % 16777216;
}