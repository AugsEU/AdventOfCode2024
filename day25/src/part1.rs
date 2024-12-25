use std::thread::LocalKey;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

pub fn compute_answer(input: &str) -> i32
{
    let mut lock_grids: Vec<CharGrid> = Vec::new();
    let mut key_grids: Vec<CharGrid> = Vec::new();

    for char_grid in input.split("\n\n")
    {
        let char_grid = CharGrid::from(char_grid);
        if char_grid.at(0, 0).unwrap() == '#'
        {
            lock_grids.push(char_grid);
        }
        else
        {
            key_grids.push(char_grid);
        }
    }

    let mut total_matches = 0;
    for lock_grid in lock_grids.iter()
    {
        for key_grid in key_grids.iter()
        {
            assert!(lock_grid.m_height == key_grid.m_height && lock_grid.m_width == key_grid.m_width);

            if key_fits_in_lock(lock_grid, key_grid)
            {
                total_matches += 1;
            }
        }
    }

    return total_matches;
}


fn key_fits_in_lock(lock_grid: &CharGrid, key_grid: &CharGrid) -> bool
{
    assert!(lock_grid.m_height == key_grid.m_height && lock_grid.m_width == key_grid.m_width);

    for x in 0..lock_grid.m_width
    {
        for y in 0..lock_grid.m_height
        {
            let lock_char = lock_grid.at(x, y).unwrap();
            let key_char = key_grid.at(x, y).unwrap();

            if lock_char == '#' && key_char == '#'
            {
                return false;
            }
        }
    }

    return true;
}