use std::cmp;
use std::cmp::max;
use std::collections::HashSet;
use std::i32;
use std::i64;
use std::usize;
use std::vec;

use glam::IVec2;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

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

    let numpad_moves : Vec<Vec<RobotCmd>> = get_numpad_moves(line, true);

    let mut min_moves = i64::MAX;
    for numpad_move_seq in numpad_moves.iter()
    {
        let mut sum_moves = 0;
        let mut arm_pos = RobotCmd::Press;
        for numpad_move in numpad_move_seq.iter()
        {
            let search_result = get_keypad_moves(*numpad_move, arm_pos, 0, max_depth);
            sum_moves += search_result.0;
            arm_pos = search_result.1;
        }

        min_moves = cmp::min(sum_moves, min_moves);
    }
    println!("  min: {}", min_moves);
    
    return min_moves * line_value;
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
fn get_keypad_moves(target: RobotCmd, arm_pos: RobotCmd, depth: i32, max_depth: i32) -> (i64, RobotCmd)
{
    let curr_pos = get_keypad_key_pos(arm_pos);
    let dest_pos = get_keypad_key_pos(target);
    let delta = curr_pos - dest_pos;

    let can_do_horiz_first = !(curr_pos.y == 0 && dest_pos == IVec2::new(0, 1));
    let can_do_vert_first = !(curr_pos != IVec2::new(0, 1) && dest_pos.y == 0);

    let two_directions = delta.x != 0 && delta.y != 0 && can_do_horiz_first && can_do_vert_first;

    let do_horiz_first = can_do_horiz_first;
    let moves_to_target = gen_robot_commands(curr_pos, dest_pos, do_horiz_first);

    let mut min_moves = i64::MAX;
    let mut final_arm_pos = RobotCmd::Press;
    if depth == max_depth
    {
        min_moves = moves_to_target.len() as i64;
        final_arm_pos = target;
    }
    else
    {
        let mut moves_sum = 0;
        let mut arm_pos = RobotCmd::Press;
        for robot_move in moves_to_target.iter()
        {
            let search_result = get_keypad_moves(*robot_move, final_arm_pos, depth + 1, max_depth);
            arm_pos = search_result.1;
            moves_sum += search_result.0;
        }
    
        if two_directions
        {
            let mut alt_moves_sum = 0;
            let mut alt_arm_pos = RobotCmd::Press;
            let moves_to_target = gen_robot_commands(curr_pos, dest_pos, !do_horiz_first);
            for robot_move in moves_to_target.iter()
            {
                let search_result = get_keypad_moves(*robot_move, alt_arm_pos, depth + 1, max_depth);
                alt_arm_pos = search_result.1;
                alt_moves_sum += search_result.0;
            }

            if alt_moves_sum <= moves_sum
            {
                moves_sum = alt_moves_sum;
                arm_pos = alt_arm_pos;
            }
        }

        min_moves = moves_sum;
        final_arm_pos = arm_pos;
    }

    return (min_moves, final_arm_pos);
}

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


// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
fn get_numpad_moves(line: &str, find_all: bool) -> Vec<Vec<RobotCmd>>
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
        let two_directions = find_all && (delta.x.abs() != 0 && delta.y.abs() != 0) && can_do_vert_first && can_do_horiz_first;

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

fn gen_robot_commands(start: IVec2, end: IVec2, horiz_first: bool) -> Vec<RobotCmd>
{
    let mut result = Vec::new();
    push_robot_commands(start, end, horiz_first, &mut result);

    return result;
}

fn push_robot_commands(start: IVec2, end: IVec2, horiz_first: bool, list: &mut Vec<RobotCmd>)
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