use std::{cmp, u128};

// Problem
pub fn get_num_stones(input: &String, blinks: i32) -> usize
{
    let mut rocks = parse_to_vec(input);

    for _ in 0..blinks
    {
        rocks = do_step(rocks);
    }

    return rocks.len();
}

pub fn get_num_stones_cheat_sheets(input: &String, blinks: usize) -> usize
{
    let cheat_sheet_blinks = blinks / 2;
    let cheat_sheet_max = 20;

    let cheat_sheets: Vec<Vec<usize>> = generate_cheat_sheets(cheat_sheet_blinks, cheat_sheet_max);

    let mut rocks = parse_to_vec(input);

    let mut cheated_steps : usize = 0;

    dbg!(&cheat_sheets);

    for i in 0..blinks
    {
        println!("Blink {i}...");
        rocks = do_step(rocks);

        let blinks_left = blinks - i - 1;
        if 0 < blinks_left && blinks_left <= cheat_sheet_blinks
        {
            println!("Cheating with {blinks_left}...");
            // Apply cheat sheets
            let mut i = 0;
            while i < rocks.len()
            {
                let rock = rocks[i];
                if rock < cheat_sheet_max as u128
                {
                    cheated_steps += cheat_sheets[rock as usize][blinks_left - 1];
                    rocks.swap_remove(i);
                }   
                else
                {
                    i += 1;    
                }
            }
        }
    }

    return rocks.len() + cheated_steps;
}

fn generate_cheat_sheets(blinks: usize, cheat_sheet_max: u128) -> Vec<Vec<usize>>
{
    let mut result = Vec::new();

    for i in 0..cheat_sheet_max
    {
        println!("Generating sheet {i}...");
        result.push(generate_cheat_sheet(i, blinks));
    }

    return result;
}

// For a single rock generate it's final size
fn generate_cheat_sheet(rock: u128, blinks: usize) -> Vec<usize>
{
    let mut result = Vec::new();
    let mut rocks = Vec::new();
    rocks.push(rock);

    for _ in 0..blinks
    {
        rocks = do_step(rocks);
        result.push(rocks.len());
    }

    return result;
}

fn do_step(rocks: Vec<u128>) -> Vec<u128>
{
    let mut result = Vec::new();
    result.reserve(rocks.len() * 2);

    for rock in rocks
    {
        if rock == 0
        {
            result.push(1);
        }
        else if let Some(split_rock) = split_digits(rock)
        {
            result.push(split_rock.0);
            result.push(split_rock.1);
        }
        else
        {
            result.push(rock*2024);   
        }
    }

    return result;
}

// Parse
fn parse_to_vec(input: &String) -> Vec<u128>
{
    let result = input.split(' ')
                        .filter(|s| s.parse::<u128>().is_ok())
                        .map(|s | s.parse::<u128>().unwrap())
                        .collect();

    return result;
}

// Math
fn count_digits(mut n: u128) -> i32
{
    let mut digits = 1;
    while n >= 10
    {
        n = n / 10;
        digits += 1;
    }

    return cmp::max(digits, 1);
}

fn split_digits(n: u128) -> Option<(u128, u128)>
{
    let num_digits = count_digits(n);
    if num_digits % 2 == 1
    {
        // Odd number of digits.
        return None;
    }

    let half_digits = num_digits / 2;
    let mut top = n;

    for _ in 0..half_digits
    {
        top = top / 10;
    }

    let mut to_sub = top;
    for _ in 0..half_digits
    {
        to_sub = to_sub * 10;
    }

    let bottom = n - to_sub;

    return Some((top, bottom));
}