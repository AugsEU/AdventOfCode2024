use std::collections::{HashMap, HashSet};
use super::char_grid::CharGrid;

type Point = (i32, i32);
type AntennaMap = HashMap<char, Vec<Point>>;

// Problem
pub fn count_antinodes_in_grid(input: &String) -> i32
{
    let char_grid = CharGrid::from(input);

    let antenna_map = generate_antenna_map(&char_grid);

    let anitnodes = find_antinodes(&antenna_map);

    let num_antinnodes_in_grid = anitnodes.iter().filter(|p| char_grid.inside_grid(p.0, p.1)).count();

    return num_antinnodes_in_grid as i32;
}

pub fn count_harmonics_in_grid(input: &String) -> i32
{
    let char_grid = CharGrid::from(input);

    let antenna_map = generate_antenna_map(&char_grid);

    let anitnodes = find_harmonics(&antenna_map, &char_grid);

    return anitnodes.len() as i32;
}

// Generate list of antenna positions with their frequency as key.
fn generate_antenna_map(grid: &CharGrid) -> AntennaMap
{
    let mut result: AntennaMap = HashMap::new();

    for x in 0..grid.m_width
    {
        for y in 0..grid.m_height
        {
            let char_at = grid.at(x, y).unwrap();

            if char_at == '.'
            {
                continue;
            }

            if let Some(vec) = result.get_mut(&char_at)
            {
                vec.push((x, y));
            }
            else
            {
                let mut new_vec : Vec<Point> = Vec::new();
                new_vec.push((x, y));

                result.insert(char_at, new_vec);
            }
        }
    }

    return result;
}

// Generate set of unique antenna points.
fn find_antinodes(antenna_map : &AntennaMap) -> HashSet<Point>
{
    let mut result: HashSet<Point> = HashSet::new();

    for kv in antenna_map.iter()
    {
        let points = kv.1;

        for i in 0..points.len()
        {
            for j in i + 1..points.len()
            {
                let (antinode1, antinode2) = find_antinode_pair(points[i], points[j]);

                let an1_unique = result.insert(antinode1);
                let an2_unique =result.insert(antinode2);

                // Do they have to be unique?
                // assert!(an1_unique && an2_unique, "Found duplicate anti-node. Review question.");

                if !an1_unique { println!("Anti-node 1 not unique: {} {}", antinode1.0, antinode1.1); }
                if !an2_unique { println!("Anti-node 2 not unique: {} {}", antinode2.0, antinode2.1); }
            }
        }
    }

    return result;
}

// Find pair of anti-nodes for given two points
fn find_antinode_pair(pt1 : Point, pt2 : Point) -> (Point, Point)
{
    // pt2 -> pt1
    let diff = (pt1.0 - pt2.0, pt1.1 - pt2.1);

    // pt1 + diff
    let anti_node1 = (pt1.0 + diff.0, pt1.1 + diff.1);

    // pt2 - diff
    let anti_node2 = (pt2.0 - diff.0, pt2.1 - diff.1);

    return (anti_node1, anti_node2);
}

// Generate set of unique harmonics
fn find_harmonics(antenna_map : &AntennaMap, grid: &CharGrid) -> HashSet<Point>
{
    let mut result: HashSet<Point> = HashSet::new();

    for kv in antenna_map.iter()
    {
        let points = kv.1;

        for i in 0..points.len()
        {
            for j in i + 1..points.len()
            {
                let harmonics = find_harmonics_between_points(points[i], points[j], &grid);

                for harmonic in harmonics
                {
                    let _ = result.insert(harmonic);
                }
            }
        }
    }

    return result;
}

fn find_harmonics_between_points(pt1: Point, pt2: Point, grid: &CharGrid) -> Vec<Point>
{
    let mut result: Vec<Point> = Vec::new();

    // Pt2 -> Pt1
    let diff = (pt1.0 - pt2.0, pt1.1 - pt2.1);
    assert!(diff != (0, 0), "Diff is zero? We will be stuck here.");

    // Propagate from pt1 out
    let mut curr_point= pt1;
    while grid.inside_grid_vec(curr_point)
    {
        result.push(curr_point);

        curr_point = (curr_point.0 + diff.0, curr_point.1 + diff.1);
    }

    // Propagate from pt2 out
    let mut curr_point= pt2;
    while grid.inside_grid_vec(curr_point)
    {
        result.push(curr_point);

        curr_point = (curr_point.0 - diff.0, curr_point.1 - diff.1);
    }

    return result;
}