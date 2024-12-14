use std::collections::HashSet;

use glam::IVec2;
use crate::aoc_utils::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Robot
{
    m_pos: IVec2,
    m_speed: IVec2
}

impl Robot 
{
    pub fn from(numbers: &Vec<i32>) -> Self
    {
        Self
        {
            m_pos: IVec2::new(numbers[0], numbers[1]),
            m_speed: IVec2::new(numbers[2], numbers[3])
        }
    }
}

pub struct RobotRoom
{
    m_robots : Vec<Robot>,
    m_height: i32,
    m_width: i32
}

impl RobotRoom
{
    pub fn from(str: &str, width: i32, height: i32) -> Self
    {
        let mut robots = Vec::new();

        let lines_numbers = get_str_nums(str);

        for line_numbers in lines_numbers.iter()
        {
            robots.push(Robot::from(&line_numbers));
        }

        Self
        {
            m_robots: robots,
            m_height: height,
            m_width: width
        }
    }

    pub fn simulate_seconds(&mut self, seconds: i32)
    {
        for r in self.m_robots.iter_mut()
        {
            r.m_pos = r.m_pos + r.m_speed * seconds;

            r.m_pos.x = positive_mod(r.m_pos.x, self.m_width);
            r.m_pos.y = positive_mod(r.m_pos.y, self.m_height);
        }
    }

    pub fn get_safety_factor(&self) -> i32
    {
        let mut quad_counts: [i32; 4] = [0, 0, 0, 0];
        for r in self.m_robots.iter()
        {
            let pos = r.m_pos;

            let to_mid_x = ((self.m_width - 1) / 2) - pos.x;
            let to_mid_y = ((self.m_height - 1) / 2) - pos.y;

            match (to_mid_x.signum(), to_mid_y.signum())
            {
                (1, 1) => quad_counts[0] += 1,
                (1, -1) => quad_counts[1] += 1,
                (-1, 1) => quad_counts[2] += 1,
                (-1, -1) => quad_counts[3] += 1,
                _ => {} // Ignore cases where either coordinate is 0
            }
        }

        return quad_counts[0]  * quad_counts[1] * quad_counts[2] * quad_counts[3];
    }

    pub fn get_likely_points(&self) -> HashSet<IVec2>
    {
        let mut pos_set = HashSet::new();

        for r in self.m_robots.iter()
        {
            pos_set.insert(r.m_pos);
        }

        let mut result = HashSet::new();
        // We expect a tree to have a trunk, so there will probably be several stacked on top of each other.
        for r in self.m_robots.iter()
        {
            let mut pos = r.m_pos;

            // Check X spaces below.
            let mut all_below = true;
            for _ in 0..5
            {
                pos.y -= 1;
                if !pos_set.contains(&pos)
                {
                    all_below = false;
                    break;
                }
            }

            if all_below
            {
                result.insert(pos.clone());
            }
        }

        return result;
    }

    pub fn to_string(&self) -> String
    {
        let mut pos_set = HashSet::new();

        for r in self.m_robots.iter()
        {
            pos_set.insert(r.m_pos);
        }

        let mut chars: Vec<char> = Vec::new();

        let likely_points = self.get_likely_points();

        for x in 0..self.m_width
        {
            for y in 0..self.m_height
            {
                let pos = IVec2::new(y, x); // Reversed for some reason?
                if likely_points.contains(&pos)
                {
                    chars.push('@');
                }
                else if pos_set.contains(&pos)
                {
                    chars.push('#');
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