use glam::IVec2;
use strum::IntoEnumIterator;
use std::collections::BinaryHeap;
use fxhash::{FxHashMap, FxHashSet};

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Shortcut
{
    start: IVec2,
    end: IVec2,
    length: i32,
    saving: usize
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

#[derive(Debug, Clone, Eq, PartialEq)]
struct PathInfo
{
    costs: FxHashMap<IVec2, usize>,
    came_from: FxHashMap<IVec2, IVec2>
}

pub fn compute_answer(input: &String, min_shortcut: usize, shortcut_len: i32) -> usize
{
    let mut grid = CharGrid::from(&input);

    let start_pos = grid.find_first('S').expect("Couldn't find start.");
    let end_pos = grid.find_first('E').expect("Couldn't find end.");
    let _ = grid.set_v(start_pos, '.');
    let _ = grid.set_v(end_pos,'.');

    // Find all distances to the end.
    let to_end_path_info = dijkstra(end_pos, &grid);
    let end_to_start_path = build_up_path(end_pos, start_pos, &to_end_path_info).expect("Can't path from end to start.");
    let mut points_on_path: FxHashSet<IVec2> = FxHashSet::default();
    points_on_path.extend(end_to_start_path.iter());

    // Now find shortcuts
    let potential_shortcuts = find_potential_shrotcuts(shortcut_len, &grid, &to_end_path_info);

    println!("Found {} potential", potential_shortcuts.len());

    // for i in 0..potential_shortcuts.len()
    // {
    //     let mut grid_clone = grid.clone();
    //     let _ = grid_clone.set_v(potential_shortcuts[i].start, 's');
    //     let _ = grid_clone.set_v(potential_shortcuts[i].end, 'S');
    //     println!("{}", grid_clone.to_string());
    //     println!("\n\n\n");
    // }

    let shortcuts_on_path : Vec<&Shortcut> = potential_shortcuts.iter().filter(|&s| points_on_path.contains(&s.start)).collect();
    assert!(shortcuts_on_path.len() == potential_shortcuts.len(), "Not all shortcuts are on the path, think how to filter those too?");

    let num_long_shortcuts = shortcuts_on_path.iter().filter(|&&s| s.saving >= min_shortcut).count();

    return num_long_shortcuts;
}

fn find_potential_shrotcuts(shortcut_len: i32, grid: &CharGrid, to_end_path_info: &PathInfo) -> Vec<Shortcut>
{
    let mut shortcuts : Vec<Shortcut> = Vec::new();

    for x in 0..grid.m_width
    {
        for y in 0..grid.m_height
        {
            let start = IVec2::new(x, y);
            let char_at_start = grid.at_vec(start).unwrap();
            if char_at_start != '.'
            {
                // Shortcut must start at empty space.
                continue;
            }

            let start_dist = to_end_path_info.costs.get(&start);
            if start_dist.is_none()
            {
                // Start is infinite distance from path.
                continue;
            }
            let start_dist = *start_dist.unwrap();

            // Scan all possible points we can tunnel to.
            for dx in -shortcut_len..=shortcut_len
            {
                let dy_range = (dx.abs() - shortcut_len).abs();
                for dy in -dy_range..=dy_range
                {
                    let delta = IVec2::new(dx, dy);
                    let delta_cost = manhattan_size(delta);
                    assert!(manhattan_size(delta) <= shortcut_len);

                    let end = start + delta;

                    if let Some(char_at_end) = grid.at_vec(end)
                    {
                        if char_at_end != '.'
                        { 
                            // Shortcut must end at empty space.
                            continue;
                        }
                    }
                    else
                    {
                        // Shortcut must end inside grid.
                        continue;
                    }

                    let end_dist = to_end_path_info.costs.get(&end);
                    if end_dist.is_none()
                    {
                        // Start is infinite distance from path.
                        continue;
                    }
                    let end_dist = *end_dist.unwrap();

                    if end_dist + (delta_cost as usize) < start_dist
                    {
                        let saving = start_dist - (end_dist + (delta_cost as usize));
                        let new_shortcut = Shortcut { start: start, end: end, length: delta_cost, saving: saving };
                        shortcuts.push(new_shortcut);
                    }
                }
            }
        }
    }

    return shortcuts;
}

/// Dijkstra's algorithm on a 2D grid to find all shortest distances from start point
/// - `start`: Starting point
/// - `grid`: A grid to check if a tile is passable
/// - `shortcuts`: Shortcuts to travel through
fn dijkstra(start: IVec2, grid: &CharGrid) -> PathInfo
{
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

    return PathInfo{ costs: costs, came_from: came_from };
}

fn build_up_path(start: IVec2, end: IVec2, path_info: &PathInfo) -> Option<Vec<IVec2>>
{
    assert!(*path_info.costs.get(&start).unwrap() == 0, "Starting at non-zero point? Wrong path info");

    if path_info.came_from.get(&end).is_none()
    {
        return None;
    }

    let mut result = Vec::new();
    let mut curr = end.clone();

    // Build path backwards
    while curr != start
    {
        result.push(curr);
        curr = *path_info.came_from.get(&curr).unwrap();
    }

    result.push(start);
    result.reverse();

    return Some(result);
}

fn manhattan_size(vec: IVec2) -> i32
{
    return vec.x.abs() + vec.y.abs();
}