use std::collections::HashMap;
use std::collections::HashSet;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operator
{
    XOR,
    OR,
    AND
}

impl Operator
{
    pub fn from(input: &str) -> Self
    {
        match input
        {
            "XOR" => { return Operator::XOR; }
            "OR" => { return Operator::OR; }
            "AND" => { return Operator::AND; }
            _ => { panic!("Invalid operator {input}"); }
        }
    }

    pub fn compute(&self, left: bool, right: bool) -> bool
    {
        match self
        {
            Operator::AND => { return left && right; }
            Operator::OR => { return left || right; }
            Operator::XOR => { return left != right; }
        }
    }
}

struct Equation
{
    left: String,
    right: String,
    operator: Operator,
    result: String
}

impl Equation
{
    pub fn from(input: &str) -> Self
    {
        let (operations, result) = input.split_once("->").unwrap(); 
        let result = String::from(result.trim());

        let mut operations = operations.trim().split(" ");

        let left = String::from(operations.next().unwrap().trim());
        let operator = Operator::from(operations.next().unwrap().trim());
        let right = String::from(operations.next().unwrap().trim());

        Self
        {
            left: left,
            right: right,
            operator: operator,
            result: result
        }
    }

    pub fn attempt_solve(&self, known_values: &HashMap<String, bool>) -> Option<bool>
    {
        let known_left = known_values.get(&self.left);
        let known_right = known_values.get(&self.right);

        if known_left.is_none() || known_right.is_none()
        {
            // Eq not solvable yet
            return None;
        }

        let known_left = *known_left.unwrap();
        let known_right = *known_right.unwrap();

        let result = self.operator.compute(known_left, known_right);

        return Some(result);
    }
}

pub fn compute_answer(input: &str) -> i64
{
    let (init_bits_str, gates_str) = input.split_once("\n\n").unwrap();
    let mut known_values = parse_init_bits(init_bits_str);
    let mut equations: Vec<Equation> = gates_str.lines()
                                .filter(|l| l.len() > 0)
                                .map(|l| Equation::from(l))
                                .collect();

    while equation_solve_step(&mut known_values, &mut equations)
    {
        // Keep solving until valid.
    }

    let mut kv_list: Vec<(String, bool)> = known_values.iter().map(|(k, v)| (k.clone(), *v)).collect();
    kv_list.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

    let mut final_val = 0;
    for (i, kv) in kv_list.iter().filter(|(k, _)| k.starts_with("z")).enumerate()
    {
        if kv.1
        {
            final_val += 1 << i;
        }
    }

    println!("{:b}", final_val);

    return final_val;
}

fn equation_solve_step(known_values: &mut HashMap<String, bool>, equations: &mut Vec<Equation>) -> bool
{
    for (i, eq) in equations.iter().enumerate()
    {
        if known_values.contains_key(&eq.result)
        {
            continue;
        }

        if let Some(new_value) = eq.attempt_solve(known_values)
        {
            let added = known_values.insert(eq.result.clone(), new_value).is_none();
            assert!(added, "Value already known?");

            return true;
        }
    }

    return false;
}

// Parse
fn parse_init_bits(init_bits_str: &str) -> HashMap<String, bool>
{
    let mut known_vals = HashMap::new();

    for line in init_bits_str.lines().filter(|l| l.len() > 0)
    {
        let (symbol, num) = line.split_once(":").unwrap();

        let symbol = String::from(symbol);
        let num = num.trim().parse::<i32>().unwrap() != 0;

        known_vals.insert(symbol, num);
    }

    return known_vals;
}