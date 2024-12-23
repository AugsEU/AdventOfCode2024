use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;


struct Graph
{
    pub neighbour_map: HashMap<String, HashSet<String>>,
}

impl Graph
{
    pub fn from(input: &str) -> Graph
    {
        let mut new_graph = Graph{ neighbour_map: HashMap::new() };

        for line in input.lines()
        {
            if line.len() == 0
            {
                continue;
            }

            let (n1, n2) = line.split_once("-").unwrap();

            new_graph.add_connection(n1, n2);
        }

        return new_graph;
    }

    fn add_node_if_not_present(&mut self, node: &str)
    {
        if self.neighbour_map.contains_key(node)
        {
            return;
        }

        self.neighbour_map.insert(String::from(node), HashSet::new());
    }

    fn add_connection(&mut self, node1: &str, node2: &str)
    {
        self.add_node_if_not_present(node1);
        self.add_node_if_not_present(node2);

        self.neighbour_map.get_mut(&node1.to_string()).unwrap().insert(node2.to_string());
        self.neighbour_map.get_mut(&node2.to_string()).unwrap().insert(node1.to_string());
    }

    fn find_all_kn(&self, root: &String, n: usize) -> Vec<Vec<String>>
    {
        if n == 1
        {
            return vec![vec![root.clone()]];
        }

        let neighbours = self.neighbour_map.get(root).unwrap();
        let neighbours: Vec<&String> = neighbours.iter().collect();

        let mut all_kn = Vec::new();
        for subset in neighbours.into_iter().combinations(n-1)
        {
            if self.is_kn(&subset)
            {
                // let mut debug_set = vec![root];
                // debug_set.extend(subset.iter());
                // println!("Found subset {:?}", &debug_set);
                let mut total_subset: Vec<String> = subset.iter().map(|&s| s.clone()).collect();
                total_subset.push(root.clone());
                total_subset.sort();

                all_kn.push(total_subset);
            }
        }

        return all_kn;
    }

    // Is this a complete graph?
    fn is_kn(&self, nodes: &Vec<&String>) -> bool
    {
        for i in 0..nodes.len()
        {
            for j in (i + 1)..nodes.len()
            {
                if !self.is_connected(nodes[i], nodes[j])
                {
                    return false;
                }
            }
        }

        return true;
    }

    fn is_connected(&self, n1: &String, n2: &String) -> bool
    {
        if let Some(neighbour_set) = self.neighbour_map.get(n1)
        {
            return neighbour_set.contains(n2);
        }

        return false;
    }
}

pub fn compute_answer(input: &str) -> i32
{
    let graph = Graph::from(input);

    let mut unique_matches = HashSet::new();
    for (node, _) in graph.neighbour_map.iter()
    {
        if node.starts_with("t")
        {
            //total_matches += 1;
            let total_kn = graph.find_all_kn(node, 3);

            for kn in total_kn
            {
                unique_matches.insert(kn);
            }
        }
    }

    return unique_matches.len() as i32;
}