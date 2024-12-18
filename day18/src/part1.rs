use glam::IVec2;
use strum::IntoEnumIterator;
use std::collections::{BinaryHeap, HashMap};

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

pub fn compute_answer(input: &String, width: i32, height: i32, num_bytes: i32) -> i32
{
    let lines_numbers = get_str_nums(&input);
    let mut grid = CharGrid::from_char('.', width, height);

    for line_numbers in lines_numbers.iter().enumerate()
    {
        if line_numbers.0 >= num_bytes as usize
        {
            break;
        }
        let byte_pos = IVec2::new(line_numbers.1[0], line_numbers.1[1]);

        let err = grid.set_v(byte_pos, '#');
        if err.is_err()
        {
            panic!("Set outside bounds of grid");
        }
    }

    let path = dijkstra(IVec2::new(0, 0), IVec2::new(width-1, height-1), &grid).expect("Couldn't find path.");

    println!("{}", grid.to_string());

    return path.len() as i32 - 1;
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
/// - `is_passable`: A closure to check if a tile is passable
pub fn dijkstra(start: IVec2, end: IVec2, grid: &CharGrid) -> Option<Vec<IVec2>> {

    let mut open_set = BinaryHeap::new();
    open_set.push(Node 
    {
        position: start,
        cost: 0,
    });

    // Maps positions to their cheapest cost and the preceding node
    let mut costs = HashMap::new();
    let mut came_from = HashMap::new();

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
    None
}