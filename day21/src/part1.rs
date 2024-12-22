use std::cmp;
use std::i32;
use std::vec;

use glam::IVec2;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

const MIDDLE_ROBOTS : i64 = 2;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RobotCmd
{
    Up,
    Down,
    Left,
    Right,
    Press,
}

pub fn compute_answer(input: &String) -> i64
{
    return input.lines().map(|s| compute_line(s)).sum();
}

fn parse_numeric(input: &str) -> i64
{
    let numeric_part: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
    return numeric_part.parse::<i64>().unwrap();
}

pub fn compute_line(line: &str) -> i64
{
    const FIND_ALL_VALUES : bool = true;
    
    let line_value = parse_numeric(line);
    println!("{} = {}", line, line_value);

    let mut possible_computes : Vec<Vec<RobotCmd>> = get_numpad_moves(line, FIND_ALL_VALUES);

    for i in 0..MIDDLE_ROBOTS
    {
        let mut new_computes = Vec::new();

        for v in possible_computes.iter()
        {
            new_computes.append(&mut get_keypad_moves(v, FIND_ALL_VALUES));
        }

        possible_computes = new_computes;
    }

    let mut min_value = i64::MAX;
    for v in possible_computes.iter()
    {
        let complexity = line_value * (v.len() as i64);
        min_value = cmp::min(min_value, complexity);
    }
    
    return min_value;
}


//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
fn get_keypad_moves(cmds: &Vec<RobotCmd>, find_all: bool) -> Vec<Vec<RobotCmd>>
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
        let two_directions = find_all &&(delta.x.abs() != 0 && delta.y.abs() != 0) && can_do_vert_first && can_do_horiz_first;

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