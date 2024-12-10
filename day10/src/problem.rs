use std::collections::HashSet;
use crate::int_grid::IntGrid;

type Point = (i32, i32);

// Problem
pub fn total_trail_score(input: &String) -> i32
{
    let grid = IntGrid::from(&input);
    let mut total_scores = 0;

    for x in 0..grid.m_width
    {
        for y in 0..grid.m_height
        {
            let value_at = grid.at(x, y).unwrap();
            if value_at == 0
            {
                let mut unique_nines: HashSet<Point> = HashSet::new();
                get_unique_trail_head_nines((x, y), &grid, &mut unique_nines);

                total_scores += unique_nines.len() as i32;
            }
        }
    }

    return total_scores;
}

pub fn total_trail_score_part2(input: &String) -> i32
{
    let grid = IntGrid::from(&input);
    let mut total_scores = 0;

    for x in 0..grid.m_width
    {
        for y in 0..grid.m_height
        {
            let value_at = grid.at(x, y).unwrap();
            if value_at == 0
            {
                let mut all_nines: Vec<Point> = Vec::new();
                get_all_trail_head_nines((x, y), &grid, &mut all_nines);

                total_scores += all_nines.len() as i32;
            }
        }
    }

    return total_scores;
}

// Pathfinding
fn get_unique_trail_head_nines(pos: Point, grid: &IntGrid, mut out_set: &mut HashSet<Point>)
{
    let value_at = grid.at_vec(pos);

    if value_at.is_none()
    {
        return;
    }

    let value_at = value_at.unwrap();

    // Base case:
    if value_at == 9
    {
        out_set.insert(pos);
        return;
    }

    let up = (pos.0, pos.1-1);
    let right = (pos.0 + 1, pos.1);
    let down = (pos.0, pos.1 + 1);
    let left = (pos.0 - 1, pos.1);

    try_go_to_next_square(value_at, up, grid, &mut out_set);
    try_go_to_next_square(value_at, right, grid, &mut out_set);
    try_go_to_next_square(value_at, down, grid, &mut out_set);
    try_go_to_next_square(value_at, left, grid, &mut out_set);
}

fn try_go_to_next_square(old_value: i32, next_pos: Point, grid: &IntGrid, mut out_set: &mut HashSet<Point>)
{
    if let Some(next_value) = grid.at_vec(next_pos) 
    {
        if next_value == old_value + 1
        {
            get_unique_trail_head_nines(next_pos, &grid, &mut out_set);
        }
    }
}

// Bodge; Just copy these for part 2 instead of using fancy generics....
fn get_all_trail_head_nines(pos: Point, grid: &IntGrid, mut out_set: &mut Vec<Point>)
{
    let value_at = grid.at_vec(pos);

    if value_at.is_none()
    {
        return;
    }

    let value_at = value_at.unwrap();

    // Base case:
    if value_at == 9
    {
        out_set.push(pos);
        return;
    }

    let up = (pos.0, pos.1-1);
    let right = (pos.0 + 1, pos.1);
    let down = (pos.0, pos.1 + 1);
    let left = (pos.0 - 1, pos.1);

    try_go_to_next_square_vec(value_at, up, grid, &mut out_set);
    try_go_to_next_square_vec(value_at, right, grid, &mut out_set);
    try_go_to_next_square_vec(value_at, down, grid, &mut out_set);
    try_go_to_next_square_vec(value_at, left, grid, &mut out_set);
}

fn try_go_to_next_square_vec(old_value: i32, next_pos: Point, grid: &IntGrid, mut out_set: &mut Vec<Point>)
{
    if let Some(next_value) = grid.at_vec(next_pos) 
    {
        if next_value == old_value + 1
        {
            get_all_trail_head_nines(next_pos, &grid, &mut out_set);
        }
    }
}