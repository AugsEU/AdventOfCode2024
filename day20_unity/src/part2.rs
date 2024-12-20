#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::cmp;

use glam::IVec2;
use rayon::prelude::*;

const GRID_SIZE : i32 = 141;


pub struct Grid
{
    m_walls: Vec<i32>
}

impl Grid
{
    pub fn from(input : &str) -> (Self, IVec2) 
    {
        let mut end = IVec2::ZERO;

        let mut walls: Vec<i32> = Vec::with_capacity((GRID_SIZE * GRID_SIZE) as usize);

        for c in input.chars().filter(|c| *c != '\n' && *c != '\r').enumerate()
        {
            walls.push(if c.1 == '#' { -1 } else { 0 });
            
            if c.1 == 'E'
            {
                let idx = c.0 as i32;
                end = IVec2::new(idx % GRID_SIZE, idx / GRID_SIZE);
            }
        }

        let this = Self
        {
            m_walls: walls
        };

        return (this, end);
    }

    pub fn get_unchecked_vec(&self, pos: IVec2) -> i32
    {
        let idx = pos.x + pos.y * GRID_SIZE;
        return self.m_walls[idx as usize];
    }

    pub fn set_v(&mut self, pos: IVec2, value: i32)
    {
        self.set(pos.x, pos.y, value);
    }

    pub fn set(&mut self, x: i32, y: i32, value: i32)
    {
        let idx = x + y * GRID_SIZE;
        self.m_walls[idx as usize] = value;
    }

    pub fn inside_grid(&self, x: i32, y: i32) -> bool
    {
        return 0 <= x && x < GRID_SIZE && 0 <= y && y < GRID_SIZE;
    }
}

const SHORTCUT_LEN : i32 = 20;
const MIN_SHORTCUT : i32 = 100;

pub fn run(input: &str) -> i64
{
    let (mut grid, end_pos) = Grid::from(&input);

    // Find all distances to the end.
    find_path(end_pos, &mut grid);

    return find_potential_shrotcuts(&grid);
}

fn find_potential_shrotcuts(grid: &Grid) -> i64
{
    return (1..(GRID_SIZE-1))
        .into_par_iter()
        .map(|x| 
            {
                (1..(GRID_SIZE-1))
                    .into_par_iter()
                    .map(|y| find_shortcuts_at(IVec2::new(x, y), grid))
                    .sum::<i16>() as i64
            })
        .sum();
}

fn find_shortcuts_at(start: IVec2, grid: &Grid) -> i16
{
    let start_dist = grid.get_unchecked_vec(start);

    if start_dist < MIN_SHORTCUT
    {
        // Shortcut must start at empty space.
        return 0;
    }

    let mut num_shortcuts : i16 = 0;

    let dx_min = cmp::max(-SHORTCUT_LEN, -start.x);
    let dx_max = cmp::min(SHORTCUT_LEN, GRID_SIZE- start.x - 1);

    // Scan all possible points we can tunnel to.
    for dx in dx_min..=dx_max
    {
        let dy_range = SHORTCUT_LEN - dx.abs();

        let dy_min = cmp::max(-dy_range, -start.y);
        let dy_max = cmp::min(dy_range, GRID_SIZE- start.y - 1);

        for dy in dy_min..=dy_max
        {
            let mut end = start;
            end.x += dx;
            end.y += dy;

            let end_dist = grid.get_unchecked_vec(end);
            if end_dist == -1
            {
                continue;
            }

            if start_dist >= MIN_SHORTCUT + SHORTCUT_LEN + end_dist - dy_range + dy.abs()
            {
                num_shortcuts += 1;
            }
        }
    }

    return num_shortcuts;
}

fn find_path(start: IVec2, grid: &mut Grid)
{
    // Initial cost for the starting position
    grid.set_v(start, 0);

    let mut len = 0;
    let mut prev = start;
    let mut curr = start;

    loop
    {
        let mut any_neigh = false;
        for dir in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
        {
            let next = curr + dir;
            if grid.get_unchecked_vec(next) == -1
            {
                continue;
            }
            if next != prev
            {
                any_neigh = true;
                prev = curr;
                curr = next;
                len += 1;
                grid.set_v(curr, len);
                break;
            }
        }

        if any_neigh == false
        {
            break;
        }
    }
}