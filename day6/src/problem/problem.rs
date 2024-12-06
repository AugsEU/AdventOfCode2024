use super::direction::Direction;
use super::char_grid::CharGrid;
use super::guard::*;
use super::*;

pub fn count_number_of_guard_positions(input: &String) -> i32
{
    let (maze, guard) = parse_string(&input);

    let steps = guard::walk_until_out_of_maze_visited(&guard, &maze);

    return steps;
}

pub fn count_number_of_infinite_obstructions(input: &String) -> i32
{
    let (mut maze, guard) = parse_string(&input);
    let mut num_infinite_obstruction = 0;

    let total_space_to_check = maze.m_width * maze.m_height;
    let mut total_spaces_checked = 0;

    // Attempt obstruction at every point.
    for x in 0..maze.m_width
    {
        for y in 0..maze.m_height
        {
            // Can't put obstruction at guard pos.
            if (x, y) == guard.m_pos
            {
                continue;
            }

            let curr_char = maze.at(x, y).unwrap();

            // Can't put obstruction on existing wall.
            if curr_char == '#'
            {
                continue;
            }

            assert!(curr_char == '.', "Unexpected character found in maze|{curr_char}| Want to build obstruction here.");

            // Place wall here
            let _ = maze.set(x, y, '#');

            // Does this cause an infinite loop?
            if guard::is_in_infinite_loop(&guard, &maze)
            {
                num_infinite_obstruction += 1;
            }

            // Remove wall.
            let _ = maze.set(x, y, '.');

            total_spaces_checked += 1;

            let percent_complete = (100 * total_spaces_checked) / total_space_to_check;
            println!("Obstructions: {percent_complete}% complete");
        }
    }

    return num_infinite_obstruction;
}

// Parse
fn parse_string(input: &String) -> (CharGrid, Guard)
{
    let mut maze: CharGrid = CharGrid::from(&input);

    let mut new_guard: Option<Guard> = None;

    for x in 0..maze.m_width
    {
        for y in 0..maze.m_height
        {
            let char_at = maze.at(x,y).unwrap();

            if let Some(guard_dir) = Direction::from(char_at)
            {
                if new_guard.is_some()
                {
                    panic!("Found multiple guards!");
                }

                new_guard = Some(Guard::new(x,y, guard_dir));

                // Erase guard tile to make things clearer.
                let _ = maze.set(x,y, '.');
            }
        }
    }

    if let Some(new_guard) = new_guard
    {
        return (maze, new_guard);
    }

    panic!("Failed to parse!");
}

