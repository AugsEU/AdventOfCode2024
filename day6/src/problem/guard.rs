use std::collections::HashSet;
use super::direction::Direction;
use super::char_grid::CharGrid;

#[derive(Debug, Clone)]
pub struct Guard
{
    pub m_pos: (i32, i32),
    pub m_facing: Direction
}

impl Guard
{
    pub fn new(x: i32, y: i32, facing: Direction) -> Self 
    {
        Self 
        {
            m_pos: (x, y),
            m_facing: facing
        }
    }

    fn walk_step(&mut self, maze: &CharGrid)
    {
        // Move one forward.
        let next_pos = self.m_facing.add_to(self.m_pos);
        let next_char = maze.at_vec(next_pos);

        // Next space is free
        if next_char.is_none() || next_char.unwrap() == '.'
        {
            // March step
            self.m_pos = next_pos;
        }
        else
        {
            // Hit wall
            let next_char = next_char.unwrap();
            assert!(next_char == '#', "Unexpected character found in maze: |{next_char}| Wanted wall. ");

            // Turn right.
            self.m_facing = self.m_facing.rot_right(); 
        }
    }
}

// Walk out of maze then get number spaces visited it took to do so.
pub fn walk_until_out_of_maze_visited(guard: &Guard, maze: &CharGrid) -> i32
{
    let mut guard= guard.clone();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while maze.inside_grid_vec(guard.m_pos)
    {
        visited.insert(guard.m_pos);
        guard.walk_step(&maze);
    }

    return visited.len() as i32;
}

// Check if this guard is in an infinite loop
pub fn is_in_infinite_loop(guard: &Guard, maze: &CharGrid) -> bool
{
    let mut guard= guard.clone();
    let mut visited_dir: HashSet<(i32, i32, Direction)> = HashSet::new();

    while maze.inside_grid_vec(guard.m_pos)
    {
        let guard_state = (guard.m_pos.0, guard.m_pos.1, guard.m_facing.clone());
        let new_state = visited_dir.insert(guard_state);

        // We have been here before, facing the same direction. Stuck in a loop.
        if new_state == false
        {
            return true;
        }

        guard.walk_step(&maze);
    }

    // We escaped the maze, not stuck.
    return false;
}

