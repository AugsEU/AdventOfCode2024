use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

use glam::IVec2;

use crate::{char_grid::CharGrid, direction::Direction};

struct RobotRoom
{
    pub m_tiles: CharGrid,
    pub m_robot: IVec2,
    pub m_instructions: Vec<Direction>
}

impl RobotRoom
{
    pub fn from(str: &str) -> Self
    {
        let (grid_str, instructions_str) = str.split_once("\n\n").unwrap();

        let mut grid = CharGrid::from(grid_str);
        let instructions : Vec<Direction> = instructions_str.
                                    chars().
                                    filter_map(|c| Direction::from(c)).collect();
        
        let robot_pos = grid.find_first('@').unwrap();

        let _ = grid.set(robot_pos.x, robot_pos.y, '.');

        Self
        {
            m_tiles: grid,
            m_robot: robot_pos,
            m_instructions: instructions
        }
    }

    pub fn process_all_instructions(&mut self)
    {
        // Rust borrow checker is annoying....
        let instructions_copy = self.m_instructions.clone();

        for dir in instructions_copy.iter()
        {
            self.move_robot(dir.clone());
        }
    }

    fn move_robot(&mut self, dir: Direction)
    {
        let mut scan_pos = dir.add_to(self.m_robot);

        // Keep scanning for free space
        loop
        {
            let char_at = self.m_tiles.at_vec(scan_pos);
            if char_at.is_none()
            {
                // Can't move outside of map
                panic!("How did we get outside of the map?");
            }
            let char_at = char_at.unwrap();

            if char_at == '#'
            {
                // Hit wall, can't move.
                return;
            }

            if char_at == '.'
            {
                // Empty space, can move here.
                break;
            }

            assert!(char_at == 'O', "Unexpected char found:|{char_at}|");
            scan_pos = dir.add_to(scan_pos);
        }

        // Move robot in dir.
        self.m_robot = dir.add_to(self.m_robot);


        // Push any boxes.
        let char_at_robot = self.m_tiles.at_vec(self.m_robot).expect("Robot on invalid tile.");

        // Move robot
        if char_at_robot == 'O'
        {
            // Put box at end.
            let _ = self.m_tiles.set_v(scan_pos, 'O');

            // Remove box at start.
            let _ = self.m_tiles.set_v(self.m_robot, '.');
        }
        else
        {
            assert!(char_at_robot == '.', "Robot not on empty space.");    
        }
    }

    pub fn get_all_gps_coords(&self) -> Vec<i32>
    {
        let mut result = Vec::new();
        for x in 0..self.m_tiles.m_width
        {
            for y in 0..self.m_tiles.m_height
            {
                let char_at = self.m_tiles.at(x, y).unwrap();
                if char_at == 'O'
                {
                    result.push(x + y * 100);
                }
            }
        }

        return result;
    }
}

pub fn compute_answer(input: &String) -> i32
{
    let mut robot_room = RobotRoom::from(&input.as_str());
    robot_room.process_all_instructions();

    return robot_room.get_all_gps_coords().iter().sum();
}