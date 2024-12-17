use fxhash::{FxHashMap, FxHashSet};

use glam::IVec2;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node
{
    m_pos: IVec2,
    m_facing: Direction
}

impl Node
{
    fn possible_nodes_at(pos: IVec2, grid: &CharGrid) -> [Node; 4]
    {
        return [ 
            Node{m_pos: pos, m_facing: Direction::North},
            Node{m_pos: pos, m_facing: Direction::East},
            Node{m_pos: pos, m_facing: Direction::South},
            Node{m_pos: pos, m_facing: Direction::West}
        ];
    }

    fn is_valid(&self, grid: &CharGrid) -> bool
    {
        let next_pos_char = grid.at_vec(self.m_pos);
        return next_pos_char.unwrap() != '#';
    }

    fn is_valid_side(&self, grid: &CharGrid) -> bool
    {
        // Additional check where we make sure we aren't turning into a wall.
        let next_pos_char = grid.at_vec(self.m_facing.add_to(self.m_pos));
        return self.is_valid(&grid) && next_pos_char.unwrap() != '#';
    }

    fn get_neighbours(&self, grid: &CharGrid) -> [Option<(Node, i32)>; 3]
    {
        let front = Node{m_pos: self.m_facing.add_to(self.m_pos), m_facing: self.m_facing};
        let right = Node{m_pos: self.m_pos, m_facing: self.m_facing.rot_right()};
        let left = Node{m_pos: self.m_pos, m_facing: self.m_facing.rot_left()};

        let front = front.is_valid(grid).then(|| (front, 1));
        let right = right.is_valid_side(grid).then(|| (right, 1000));
        let left = left.is_valid_side(grid).then(|| (left, 1000));

        [front, right, left]
    }

  
}

pub fn compute_answer(input: &String) -> i32
{
    return search_grid(&CharGrid::from(&input));
}

fn analyse_grid(grid: &CharGrid) -> (IVec2, IVec2)
{
    let mut start: Option<IVec2> = None;
    let mut end : Option<IVec2> = None;

    for x in 0..grid.m_width
    {
        for y in 0..grid.m_height
        {
            let char = grid.at(x, y).unwrap();
            let pos = IVec2::new(x, y);
            match char
            {
                'S' => { start = Some(pos); }
                'E' => { end = Some(pos); }
                '.' | '#' => {}
                _ => { panic!("Found unexpected character in grid."); }
            }
        }
    }

    let start = start.expect("Didn't find start.");
    let end = end.expect("Didn't find end point.");

    return (start, end);
}

fn search_grid(grid: &CharGrid) -> i32
{
    let (start, end) = analyse_grid(&grid);

    // Previous node(for debug retracing the path).
    let mut prev: FxHashMap<Node, Vec<Node>> = FxHashMap::default();

    // Minimum cost to reach a specific node. If node not in map then it is infinite.
    let mut node_dist: FxHashMap<Node, i32> = FxHashMap::default();

    // Visited nodes
    let mut visited: FxHashSet<Node> = FxHashSet::default();

    // Nodes to search.
    let start_node = Node{m_pos: start, m_facing: Direction::East};
    node_dist.insert(start_node, 0);

    // Search until every node has been visited.
    loop
    {
        let curr = node_dist.iter()
                    .filter(|&(node, _)| !visited.contains(node))
                    .min_by_key(|&(_, &value)| value);

        if curr.is_none()
        {
            // Explored everything.
            break;
        }

        let curr= curr.unwrap();        
        let curr_node = *curr.0;
        let curr_dist = *curr.1;

        //println!("Visiting {:?}", curr_node);

        visited.insert(curr_node);
        
        let neighbours = curr_node.get_neighbours(&grid);
        
        for n in neighbours.iter().filter_map(|&n| n)
        {
            if visited.contains(&n.0) { continue; }

            //println!("    N {:?}", n.0);
            let alt = curr_dist + n.1;
            let working_dist = *node_dist.get(&n.0).unwrap_or(&i32::MAX);

            if alt < working_dist
            {
                node_dist.insert(n.0, alt);

                let new_bests = vec![curr_node];
                prev.insert(n.0, new_bests);
            }
            else if alt == working_dist
            {
                let node_prevs = prev.get_mut(&n.0).expect("Node has distance but no prev?");
                node_prevs.push(curr_node);
            }
        }
    }

    println!("Visited: {} nodes", visited.len());

    let end_nodes = Node::possible_nodes_at(end, &grid);

    // Find minimum
    let min_end_node = *end_nodes.iter()
                                    .filter_map(|n| node_dist.get(n))
                                    .min()
                                    .expect("No miminum node?");
    let end_nodes: Vec<&Node> = end_nodes.iter().filter(|n| node_dist.get(n) == Some(&min_end_node)).collect();
    let mut path_positions : FxHashSet<IVec2> = FxHashSet::default();

    for end_node in end_nodes.iter()
    {
        let trace_positions = trace_back_path(**end_node, &prev);
        path_positions.extend(trace_positions);
    }

    return path_positions.len() as i32;
}


fn trace_back_path(end: Node, prev_list: &FxHashMap<Node, Vec<Node>>) -> FxHashSet<IVec2>
{
    let mut positions: FxHashSet<IVec2> = FxHashSet::default();
    let mut visited: FxHashSet<Node> = FxHashSet::default();
    let mut to_search : Vec<Node> = Vec::new();
    to_search.push(end);

    while to_search.len() > 0
    {
        let curr_node = to_search.pop().unwrap();

        if visited.contains(&curr_node)
        {
            continue;
        }

        visited.insert(curr_node);
        positions.insert(curr_node.m_pos);

        if let Some(prev_nodes) = prev_list.get(&curr_node)
        {
            to_search.append(&mut prev_nodes.clone());
        }
    }

    return positions;
}

