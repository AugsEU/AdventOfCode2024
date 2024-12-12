use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

use crate::{char_grid::CharGrid, direction::Direction};

type Point = (i32, i32);

pub fn total_fence_price(input: &String) -> i32
{
    let grid = CharGrid::from(input);

    return get_total_fence_prices(&grid);
}

pub fn total_fence_price_discounted(input: &String) -> i32
{
    let grid = CharGrid::from(input);

    return get_total_fence_prices_discount(&grid);
}

// Analyse grid
fn categorise_grid(grid: &CharGrid) -> HashMap<char, HashSet<Point>>
{
    let mut result : HashMap<char, HashSet<Point>> = HashMap::new();

    for x in 0..grid.m_width
    {
        for y in 0..grid.m_height
        {
            let char_at = grid.at(x, y).unwrap();
            if !result.contains_key(&char_at)
            {
                result.insert(char_at, HashSet::new());
            }

            let char_points: &mut HashSet<Point> = result.get_mut(&char_at).unwrap();
            char_points.insert((x, y));
        }
    }

    return result;
}

// Normal fence price
fn get_total_fence_prices(grid: &CharGrid) -> i32
{
    let char_to_points = categorise_grid(&grid);

    let mut total_price = 0;
    for char_points in char_to_points.iter()
    {
        let mut all_char_points = char_points.1.clone();

        while let Some(&region_seed) = all_char_points.iter().next()
        {
            let region_info = get_region_and_price(&all_char_points, region_seed);

            // remove all points from the region.
            for region_point in region_info.0.iter()
            {
                let removed = all_char_points.remove(region_point);
                assert!(removed, "Point ({}, {}) in region but not in char set?", region_point.0, region_point.1);
            }

            // Add on region's price.
            total_price += region_info.1;
        }
    }

    return total_price as i32;
}

fn get_region_and_price(char_points : &HashSet<Point>, region_seed: Point) -> (HashSet<Point>, usize)
{
    let mut points_to_search : HashSet<Point> = HashSet::new();
    let mut points_searched : HashSet<Point> = HashSet::new();

    let mut perimeter = 0;
    let mut area = 1;

    points_to_search.insert(region_seed);

    while let Some(search_point) = points_to_search.iter().next()
    {
        let search_point = search_point.clone();
        points_to_search.remove(&search_point);

        let new_point  = points_searched.insert(search_point);

        assert!(new_point, "Searched {}, {} twice.", search_point.0, search_point.1);

        // Find neighbours
        for dir in Direction::iter()
        {
            let neighbour = dir.add_to(search_point);

            // Already searched or already in the pipeline to search
            if points_searched.contains(&neighbour) || points_to_search.contains(&neighbour)
            {
                continue;
            }

            // Is the neighbour the same character type?
            let neighbour_our_type = char_points.contains(&neighbour);

            if neighbour_our_type
            {
                // Search it, and it is part of this region.
                points_to_search.insert(neighbour);
                area += 1;
            }
            else
            {
                // Point outside of our char type. It is part of the perimeter.
                perimeter += 1;
            }
        }
    }

    assert!(area == points_searched.len(), "Area mismatch. {} vs {}", area, points_searched.len());

    return (points_searched, perimeter * area);
}


// Discount fence price
fn get_total_fence_prices_discount(grid: &CharGrid) -> i32
{
    let char_to_points = categorise_grid(&grid);

    let mut total_price = 0;
    for char_points in char_to_points.iter()
    {
        let mut all_char_points = char_points.1.clone();

        while let Some(&region_seed) = all_char_points.iter().next()
        {
            let region_info = get_region_and_price_discount(&all_char_points, region_seed);

            // remove all points from the region.
            for region_point in region_info.0.iter()
            {
                let removed = all_char_points.remove(region_point);
                assert!(removed, "Point ({}, {}) in region but not in char set?", region_point.0, region_point.1);
            }

            // Add on region's price.
            total_price += region_info.1;
        }
    }

    return total_price as i32;
}

fn get_region_and_price_discount(char_points : &HashSet<Point>, region_seed: Point) -> (HashSet<Point>, usize)
{
    let mut points_to_search : HashSet<Point> = HashSet::new();
    let mut points_searched : HashSet<Point> = HashSet::new();

    let mut perimeter_points : HashSet<(Point, Direction)> = HashSet::new();

    let mut area = 1;

    points_to_search.insert(region_seed);

    while let Some(search_point) = points_to_search.iter().next()
    {
        let search_point = search_point.clone();
        points_to_search.remove(&search_point);

        let new_point  = points_searched.insert(search_point);

        assert!(new_point, "Searched {}, {} twice.", search_point.0, search_point.1);

        // Find neighbours
        for dir in Direction::iter()
        {
            let neighbour = dir.add_to(search_point);

            // Already searched or already in the pipeline to search
            if points_searched.contains(&neighbour) || points_to_search.contains(&neighbour)
            {
                continue;
            }

            // Is the neighbour the same character type?
            let neighbour_our_type = char_points.contains(&neighbour);

            if neighbour_our_type
            {
                // Search it, and it is part of this region.
                points_to_search.insert(neighbour);
                area += 1;
            }
            else
            {
                // Point outside of our char type. It is part of the perimeter.
                perimeter_points.insert((neighbour, dir));
            }
        }
    }

    assert!(area == points_searched.len(), "Area mismatch. {} vs {}", area, points_searched.len());

    let num_sides = get_number_of_sides(&mut perimeter_points);
    return (points_searched, num_sides * area);
}


fn get_number_of_sides(perimeter_points: &mut HashSet<(Point, Direction)>) -> usize
{
    let mut num_sides = 0;

    while let Some(side_seed) = perimeter_points.iter().next()
    {
        let side_seed = side_seed.clone();
        let side_point = side_seed.0;
        let side_normal = side_seed.1.clone();
        num_sides += 1;

        perimeter_points.remove(&side_seed);

        // remove all points to the left and to the right.
        let right = side_normal.rot_right();
        let left = right.invert();
        for dir in [right, left]
        {
            let mut remove_point = dir.add_to(side_point);

            while perimeter_points.remove(&(remove_point, side_normal.clone()))
            {
                remove_point = dir.add_to(remove_point);
            }
        }
    }

    return num_sides;
}