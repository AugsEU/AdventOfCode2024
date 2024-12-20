use glam::IVec2;
use strum::IntoEnumIterator;
use std::collections::BinaryHeap;
use fxhash::{FxHashMap, FxHashSet};

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

pub fn compute_answer(input: &String, min_shortcut: i32) -> i32
{
    let mut grid = CharGrid::from(&input);

    let start_pos = grid.find_first('S').expect("Couldn't find start.");
    let end_pos = grid.find_first('E').expect("Couldn't find end.");
    let _ = grid.set_v(start_pos, '.');
    let _ = grid.set_v(end_pos,'.');

    let original_path = dijkstra(start_pos, end_pos, &grid).expect("Couldn't find path.");
    let mut shortcuts : Vec<IVec2> = Vec::new();

    for x in 0..grid.m_width
    {
        for y in 0..grid.m_height
        {
            let pos = IVec2::new(x, y);

            if !is_potential_shortcut(pos, &grid)
            {
                continue;
            }

            let pos_char = grid.at_vec(pos).unwrap();

            // Open shortcut
            let _ = grid.set_v(pos, '.');

            let short_path = dijkstra(start_pos, end_pos, &grid).expect("Couldn't shorter path.");

            // Close shortcut
            let _ = grid.set_v(pos, pos_char);

            assert!(short_path.len() <= original_path.len(), "Shortcut path is longer than original?");

            let path_diff = original_path.len() - short_path.len();

            if path_diff >= min_shortcut as usize
            {
                println!("Found shortcut of {} at {}", path_diff, pos);
                shortcuts.push(pos);
            }
        }
    }

    return shortcuts.len() as i32;
}

fn is_potential_shortcut(pos: IVec2, grid: &CharGrid) -> bool
{
    // Verify shortcut start.
    if let Some(chat_at) = grid.at_vec(pos)
    {
        if chat_at == '.'
        {
            return false;
        }

        assert!(chat_at == '#');
    }
    else
    {
        return false;
    }

    return is_neighbouring_free_space(pos, grid);
}

fn is_neighbouring_free_space(pos: IVec2, grid: &CharGrid) -> bool
{
    for dir in Direction::iter()
    {
        let neigh_pos = dir.add_to(pos);

        if let Some(neigh_char) = grid.at_vec(neigh_pos)
        {
            if neigh_char == '.'
            {
                return true;
            }
        }
    }

    return false;
}

/// A node in the priority queue
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node
{
    position: IVec2,
    cost: usize,
}

impl Ord for Node
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        // Reverse the comparison to make BinaryHeap a min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

/// Dijkstra's algorithm on a 2D grid
/// - `start`: Starting point
/// - `end`: Endpoint
/// - `grid`: A grid to check if a tile is passable
pub fn dijkstra(start: IVec2, end: IVec2, grid: &CharGrid) -> Option<Vec<IVec2>> {

    let mut open_set = BinaryHeap::new();
    open_set.push(Node 
    {
        position: start,
        cost: 0,
    });

    // Maps positions to their cheapest cost and the preceding node
    let mut costs = FxHashMap::default();
    let mut came_from = FxHashMap::default();

    // Initial cost for the starting position
    costs.insert(start, 0);

    while let Some(Node { position, cost }) = open_set.pop()
    {
        // If we've reached the end, reconstruct the path
        if position == end
        {
            let mut path = vec![end];
            while let Some(&prev) = came_from.get(path.last().unwrap())
            {
                path.push(prev);
            }
            path.reverse();
            return Some(path);
        }

        // Explore neighbors
        for dir in Direction::iter()
        {
            let neighbor_pos = dir.add_to(position);

            let char_at = grid.at_vec(neighbor_pos);

            let passable = char_at == Some('.');

            if passable
            {
                let new_cost = cost + 1;

                if new_cost < *costs.get(&neighbor_pos).unwrap_or(&usize::MAX)
                {
                    costs.insert(neighbor_pos, new_cost);
                    came_from.insert(neighbor_pos, position);
                    open_set.push(Node
                    {
                        position: neighbor_pos,
                        cost: new_cost,
                    });
                }
            }
        }
    }

    // If we exhaust the search without finding the end
    return None;
}