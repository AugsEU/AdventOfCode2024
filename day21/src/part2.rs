use std::cmp;
use std::cmp::max;
use std::collections::HashMap;
use std::i32;
use std::i64;
use std::usize;
use std::vec;

use glam::IVec2;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

type RobotSeq = Vec<RobotCmd>;
type SearchCache = HashMap<(RobotSeq, i32), i64>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RobotCmd
{
    Up,
    Down,
    Left,
    Right,
    Press,
}

pub fn compute_answer(input: &String, max_depth: i32) -> i64
{
    return input.lines().map(|s| compute_line(s, max_depth)).sum();
}

fn parse_numeric(input: &str) -> i64
{
    let numeric_part: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    return numeric_part.parse::<i64>().unwrap();
}

pub fn compute_line(line: &str, max_depth: i32) -> i64
{
    let line_value = parse_numeric(line);
    println!("{} = {}", line, line_value);

    let numpad_seqs : Vec<RobotSeq> = get_all_numpad_seq(line);

    let mut min_moves = i64::MAX;
    for numpad_seq in numpad_seqs.iter()
    {
        let mut cache = SearchCache::new();
        let mut sum_moves = 0;
        let seq_split = split_seq_by_a_press(&numpad_seq);
        for sub_seq in seq_split.iter()
        {
            sum_moves += num_keypad_moves_from_a(sub_seq, 0, max_depth, &mut cache);
        }

        min_moves = cmp::min(sum_moves, min_moves);
    }
    println!("  min: {}", min_moves);
    
    return min_moves * line_value;
}


// Assuming all above robots are at A, how many moves will it take to generate the target keypresses?
fn num_keypad_moves_from_a(target: &RobotSeq, depth: i32, max_depth: i32, cache: &mut SearchCache) -> i64
{
    let possible_sequences = get_all_keypad_seq(target);

    if depth == max_depth
    {
        return possible_sequences[0].len() as i64;
    }

    let cache_key = (target.clone(), depth);

    if let Some(cached_result) = cache.get(&cache_key)
    {
        return *cached_result;
    }

    let mut min_moves = i64::MAX;

    for seq in possible_sequences.iter()
    {
        let seq_split = split_seq_by_a_press(seq);

        let mut total_move_cost = 0;
        for sub_seq in seq_split
        {
            total_move_cost += num_keypad_moves_from_a(&sub_seq, depth + 1, max_depth, cache);
        }

        min_moves = cmp::min(total_move_cost, min_moves);
    }

    cache.insert(cache_key, min_moves);
    return min_moves;
}

fn get_all_keypad_seq(cmds: &Vec<RobotCmd>) -> Vec<Vec<RobotCmd>>
{
    let mut result: Vec<Vec<RobotCmd>> = Vec::new();
    result.push(Vec::new());

    let mut curr_pos = IVec2::new(2, 0);

    for c in cmds.iter()
    {
        let dest_pos = get_keypad_key_pos(*c);
        let delta = dest_pos - curr_pos;

        let can_do_horiz_first = !(curr_pos.y == 0 && dest_pos == IVec2::new(0, 1));
        let can_do_vert_first = !(curr_pos == IVec2::new(0, 1) && dest_pos.y == 0);
        let two_directions = delta.x.abs() != 0 && delta.y.abs() != 0 && can_do_vert_first && can_do_horiz_first;

        let do_horiz = can_do_horiz_first;

        if two_directions
        {
            let mut new_vectors = result.clone();
            for v in new_vectors.iter_mut()
            {
                push_robot_commands(curr_pos, dest_pos, !do_horiz, v);
            }

            for v in result.iter_mut()
            {
                push_robot_commands(curr_pos, dest_pos, do_horiz, v);
            }

            result.append(&mut new_vectors);
        }
        else
        {
            for v in result.iter_mut()
            {
                push_robot_commands(curr_pos, dest_pos, do_horiz, v);
            }
        }

        for v in result.iter_mut()
        {
            v.push(RobotCmd::Press);
        }

        curr_pos = dest_pos;
    }

    return result;
}


fn get_all_numpad_seq(line: &str) -> Vec<RobotSeq>
{
    let mut result = Vec::new();
    result.push(Vec::new());

    let mut curr_pos = IVec2::new(2, 3);

    for c in line.chars()
    {
        let dest_pos = get_numpad_key_pos(c);
        let delta = dest_pos - curr_pos;

        let can_do_horiz_first = !(curr_pos.y == 3 && dest_pos.x == 0);
        let can_do_vert_first = !(curr_pos.x == 0 && dest_pos.y == 3);
        let two_directions = (delta.x.abs() != 0 && delta.y.abs() != 0) && can_do_vert_first && can_do_horiz_first;

        let do_horiz = can_do_horiz_first;

        if two_directions
        {
            let mut new_vectors = result.clone();
            for v in new_vectors.iter_mut()
            {
                push_robot_commands(curr_pos, dest_pos, !do_horiz, v);
            }

            for v in result.iter_mut()
            {
                push_robot_commands(curr_pos, dest_pos, do_horiz, v);
            }

            result.append(&mut new_vectors);
        }
        else
        {
            for v in result.iter_mut()
            {
                push_robot_commands(curr_pos, dest_pos, do_horiz, v);
            }
        }

        for v in result.iter_mut()
        {
            v.push(RobotCmd::Press);
        }

        curr_pos = dest_pos;
    }

    return result;
}

// Generate a sequence
fn push_robot_commands(start: IVec2, end: IVec2, horiz_first: bool, list: &mut RobotSeq)
{
    if horiz_first
    {
        let horz_move = if end.x < start.x { RobotCmd::Left } else { RobotCmd::Right } ;
        for _ in 0..(end.x-start.x).abs()
        {
            list.push(horz_move);
        }

        let vert_move = if end.y < start.y { RobotCmd::Up } else { RobotCmd::Down } ;
        for _ in 0..(end.y-start.y).abs()
        {
            list.push(vert_move);
        }
    }
    else
    {
        let vert_move = if end.y < start.y { RobotCmd::Up } else { RobotCmd::Down } ;
        for _ in 0..(end.y-start.y).abs()
        {
            list.push(vert_move);
        }

        let horz_move = if end.x < start.x { RobotCmd::Left } else { RobotCmd::Right } ;
        for _ in 0..(end.x-start.x).abs()
        {
            list.push(horz_move);
        }
    }
}


///////////////////////////////////////////////
/// Utils
///////////////////////////////////////////////

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
fn get_numpad_key_pos(key_char: char) -> IVec2
{
    match key_char
    {
        '7' => { return IVec2::new(0,0); }
        '8' => { return IVec2::new(1,0); }
        '9' => { return IVec2::new(2,0); }
        '4' => { return IVec2::new(0,1); }
        '5' => { return IVec2::new(1,1); }
        '6' => { return IVec2::new(2,1); }
        '1' => { return IVec2::new(0,2); }
        '2' => { return IVec2::new(1,2); }
        '3' => { return IVec2::new(2,2); }
        '0' => { return IVec2::new(1,3); }
        'A' => { return IVec2::new(2,3); }
        _ => { panic!("Invalid numpad key. {}", key_char)}
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
fn get_keypad_key_pos(key_cmd: RobotCmd) -> IVec2
{
    match key_cmd
    {
        RobotCmd::Up =>     { return IVec2::new(1,0); }
        RobotCmd::Press =>  { return IVec2::new(2,0); }
        RobotCmd::Left =>   { return IVec2::new(0,1); }
        RobotCmd::Down =>   { return IVec2::new(1,1); }
        RobotCmd::Right =>  { return IVec2::new(2,1); }
    }
}

fn split_seq_by_a_press(seq: &RobotSeq) -> Vec<RobotSeq>
{
    let mut result = Vec::new();

    result.push(Vec::new());

    for &cmd in seq.iter()
    {
        result.last_mut().unwrap().push(cmd);

        if cmd == RobotCmd::Press
        {
            result.push(Vec::new());
        }
    }

    return result;
}

fn get_cmd_list_debug_str(cmds: &Vec<RobotCmd>) -> String
{
    cmds.into_iter()
        .map(|cmd| match cmd {
            RobotCmd::Up => '^',
            RobotCmd::Down => 'v',
            RobotCmd::Left => '<',
            RobotCmd::Right => '>',
            RobotCmd::Press => 'A',
        })
        .collect()
}