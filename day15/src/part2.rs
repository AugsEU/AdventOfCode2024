use std::collections::HashSet;
use std::thread::panicking;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

use glam::{Vec2, IVec2};

use crate::{char_grid::CharGrid, direction::Direction};

use std::io::{self, Write};

struct RobotRoom
{
    pub m_robot: IVec2,
    pub m_walls: HashSet<IVec2>,
    pub m_boxes: HashSet<IVec2>,
    pub m_size: IVec2
}

impl RobotRoom
{
    pub fn parse_room_and_instructions(str: &str) -> (RobotRoom, Vec<Direction>)
    {
        let (grid_str, instructions_str) = str.split_once("\n\n").unwrap();

        let grid = CharGrid::from(grid_str);
        let instructions : Vec<Direction> = instructions_str.
                                    chars().
                                    filter_map(|c| Direction::from(c)).collect();
        
        let mut robot_pos: IVec2 = IVec2::ZERO;
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();

        for x in 0..grid.m_width
        {
            for y in 0..grid.m_height
            {
                let char_at = grid.at(x, y).unwrap();
                let virt_coord1 = IVec2::new(2*x, y);
                let virt_coord2 = IVec2::new(2*x + 1, y);

                match char_at
                {
                    '@' => { robot_pos = virt_coord1; }
                    '#' => { walls.insert(virt_coord1); walls.insert(virt_coord2); }
                    'O' => { boxes.insert(virt_coord1); }
                    '.' => {}
                    _ => { panic!("Unexpected char found."); }
                }
            }
        }

        let robot_room = RobotRoom {
            m_robot: robot_pos,
            m_walls: walls,
            m_boxes: boxes,
            m_size: IVec2::new(grid.m_width, grid.m_height)
        };

        return (robot_room, instructions);
    }

    pub fn process_all_instructions(&mut self, instructions: &Vec<Direction>)
    {
        for dir in instructions.iter()
        {
            self.move_robot(dir.clone());
        }
    }

    fn move_robot(&mut self, dir: Direction)
    {
        let next_pos = dir.add_to(self.m_robot);
        let seed_box = self.check_for_box(next_pos);

        // Simple case: wall
        if self.m_walls.contains(&next_pos)
        {
            // Can't walk here.
            return;
        }

        // Simple case: free space
        if seed_box.is_none()
        {
            self.m_robot = next_pos;
            return;
        }

        let prev_boxes = self.m_boxes.clone();
        let can_push = self.try_move_box_recurse(seed_box.unwrap(), dir);
        if can_push
        {
            self.m_robot = next_pos;
        }
        else
        {
            // Revert all boxes.
            self.m_boxes = prev_boxes;
        }
    }

    fn try_move_box_recurse(&mut self, box_pos: IVec2, dir: Direction) -> bool
    {
        let next_pos = dir.add_to(box_pos);

        // Can't move box into wall.
        if self.box_vs_wall(next_pos)
        {
            return false;
        }

        let left_box = self.check_for_box(next_pos);
        let right_box = self.check_for_box(next_pos + IVec2::X);

        let boxes_to_check = if left_box==right_box { [left_box, None] } else { [left_box, right_box] };
        let mut all_boxes_can_move = true;

        for pushed_box in boxes_to_check.iter()
        {
            if let Some(pushed_box) = pushed_box
            {
                if *pushed_box == box_pos
                {
                    // Found ourselves.
                    continue;
                }

                // Try pushing this box.
                if !self.try_move_box_recurse(*pushed_box, dir.clone())
                {
                    all_boxes_can_move = false;
                    break;
                }
            }
        }

        if !all_boxes_can_move
        {
            return false;
        }

        // move us.
        let removed = self.m_boxes.remove(&box_pos);
        assert!(removed);
        let added = self.m_boxes.insert(next_pos);
        assert!(added);

        return true;
    }

    fn box_vs_wall(&self, pos: IVec2) -> bool
    {
        return self.m_walls.contains(&pos) || self.m_walls.contains(&(pos + IVec2::new(1, 0)));
    }

    fn check_for_box(&self, pos: IVec2) -> Option<IVec2>
    {
        let left_pos = pos - IVec2::X;

        // Check point or one to the left of point.
        if self.m_boxes.contains(&pos)
        {
            return Some(pos);
        }
        else if self.m_boxes.contains(&left_pos)
        {
            return Some(left_pos);    
        }

        return None;
    }

    fn point_free(&self, pos: IVec2) -> bool
    {
        return !self.m_walls.contains(&pos) && self.check_for_box(pos).is_none();
    }

    pub fn sum_all_gps_coords(&self) -> i32
    {
        return self.m_boxes.iter().map(|b| b.x + b.y * 100).sum();
    }

    pub fn to_string(&self) -> String
    {
        let mut chars: Vec<char> = Vec::new();

        for y in 0..self.m_size.y
        {
            for x in 0..self.m_size.x * 2
            {
                let pos = IVec2::new(x, y);
                let prev_pos = IVec2::new(x-1, y);
                if self.m_robot == pos
                {
                    chars.push('@');
                }
                else if self.m_walls.contains(&pos)
                {
                    chars.push('#');
                }
                else if self.m_boxes.contains(&pos)
                {
                    chars.push('[');
                }
                else if self.m_boxes.contains(&prev_pos)
                {
                    chars.push(']');
                }
                else
                {
                    chars.push('.');
                }
            }

            chars.push('\n');
        }

        return chars.into_iter().collect();
    }
}

pub fn compute_answer(input: &String) -> i32
{
    let (mut robot_room, instructions) = RobotRoom::parse_room_and_instructions(&input.as_str());
    robot_room.process_all_instructions(&instructions);

    return robot_room.sum_all_gps_coords();
}