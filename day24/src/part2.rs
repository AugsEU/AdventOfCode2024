use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::mem::swap;

use rand::Rng;

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

#[derive(Clone)]
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

// Machine
#[derive(Clone)]
struct Machine
{
    known_values: HashMap<String, bool>,
    equations: Vec<Equation>
}

impl Machine
{
    fn from(input: &str) -> Self
    {
        let (init_bits_str, gates_str) = input.split_once("\n\n").unwrap();

        // Find initial values.
        let mut known_values = HashMap::new();
        for line in init_bits_str.lines().filter(|l| l.len() > 0)
        {
            let (symbol, num) = line.split_once(":").unwrap();

            let symbol = String::from(symbol);
            let num = num.trim().parse::<i32>().unwrap() != 0;

            known_values.insert(symbol, num);
        }

        // Find equations
        let equations: Vec<Equation> = gates_str.lines()
                                    .filter(|l| l.len() > 0)
                                    .map(|l| Equation::from(l))
                                    .collect();
        
        Self
        {
            known_values: known_values,
            equations: equations
        }
    }

    fn set_init_registers(&mut self, x: i64, y: i64)
    {
        self.known_values.clear();

        for i in 0..64
        {
            let x_set = (x >> i & 0b0001) != 0;
            self.known_values.insert(format!("x{:02}", i), x_set);
            
            let y_set = (y >> i & 0b0001) != 0;
            self.known_values.insert(format!("y{:02}", i), y_set);
        }
    }

    fn compute_output(&self) -> i64
    {
        let mut mut_clone = self.clone();

        while mut_clone.equation_solve_step()
        {
            // Keep solving until valid.
        }
    
        let mut kv_list: Vec<(String, bool)> = mut_clone.known_values.iter().map(|(k, v)| (k.clone(), *v)).collect();
        kv_list.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    
        let mut final_val = 0;
        for (i, kv) in kv_list.iter().filter(|(k, _)| k.starts_with("z")).enumerate()
        {
            if kv.1
            {
                final_val += 1 << i;
            }
        }

        return final_val;
    }

    fn equation_solve_step(&mut self) -> bool
    {
        let mut advanced_eq = false;
        let mut i = 0;
        while i < self.equations.len()
        {
            if let Some(new_value) = self.equations[i].attempt_solve(&self.known_values)
            {
                let eq_name = self.equations[i].result.clone();

                // Remove equation as we no longer need it.
                self.equations.remove(i);

                let added = self.known_values.insert(eq_name, new_value).is_none();
                assert!(added, "Value already known?");

                advanced_eq = true;
            }
            else
            {
                i += 1;    
            }
        }

        return advanced_eq;
    }

    fn swap_wires(&mut self, wire1_idx: usize, wire2_idx: usize)
    {
        assert!(wire1_idx != wire2_idx);

        let wire1 = self.equations[wire1_idx].result.clone();
        let wire2 = self.equations[wire2_idx].result.clone();

        self.equations[wire1_idx].result = wire2;
        self.equations[wire2_idx].result = wire1;
    }
}

pub fn compute_answer(input: &str) -> i64
{
    let mut machine = Machine::from(input);

    fix_machine(&mut machine);

    return 0;
}

fn fix_machine(machine: &mut Machine)
{
    const NUM_BITS_TO_SEARCH : usize = 45;

    let mut last_left_off_idx : Vec<usize> = vec![0; NUM_BITS_TO_SEARCH];

    let all_swaps = generate_swaps(machine.equations.len());
    let mut cont = true;

    while cont
    {
        println!("");
        println!("NEW RUN==========");

        let mut machine = machine.clone(); // Start a fresh machine.
        cont = false; // Assume this will work. Chat is this the run?

        let mut swaps_that_fixed: Vec<(usize, usize)> = Vec::new();

        for bit in 7..=NUM_BITS_TO_SEARCH
        {
            println!("");
            println!("Machine test {}", bit);
            if test_machine(bit, &mut machine)
            {
                // Machine works for this so it's fine.
                continue;
            }

            let mut swap_that_fixed = None;
            let start_idx = last_left_off_idx[bit-1];

            println!("   Not working, attempting swaps from {start_idx}");

            for i in start_idx..all_swaps.len()
            {
                last_left_off_idx[bit-1] = i+1;

                let swap_ids = all_swaps[i];
                let banned_swap_id = (bit, swap_ids.0, swap_ids.1);

                machine.swap_wires(swap_ids.0, swap_ids.1);

                if test_machine(bit, &mut machine)
                {
                    swap_that_fixed = Some(swap_ids);
                    break;
                }
                else
                {
                    // Swap them back.
                    machine.swap_wires(swap_ids.0, swap_ids.1);
                }
            }
            
            if let Some(swap_that_fixed) = swap_that_fixed
            {
                swaps_that_fixed.push(swap_that_fixed);

                let wire1_name = &machine.equations[swap_that_fixed.0].result;
                let wire2_name = &machine.equations[swap_that_fixed.1].result;

                println!("    Swap ({}, {}) fixed it.", wire1_name, wire2_name);

                for i in 0..(bit-1)
                {
                    if last_left_off_idx[i] > 0
                    {
                        last_left_off_idx[i] -= 1;
                    }
                }
            }
            else
            {
                last_left_off_idx[bit-1] = 0;
                println!("    Couldn't fix machine on bit {bit}. Trying again.");
                cont = true;
                break;
            }
        }

        if cont == false
        {
            swaps_that_fixed.sort();
            println!("Swaps {:?}", swaps_that_fixed);
        }
    }
}

// Heuristic to see if machine is working.
fn test_machine(num_bits: usize, machine: &mut Machine) -> bool
{
    let mut rng = rand::thread_rng();
    let modulo = 1 << (num_bits);

    // First test: random numbers
    for i in 0..1000
    {
        let x_reg : i64 = rng.gen_range(0..modulo);
        let y_reg : i64 = rng.gen_range(0..modulo);

        machine.set_init_registers(x_reg, y_reg);
        let machine_output = machine.compute_output();

        if machine_output != x_reg + y_reg
        {
            return false;
        }
    }

    // Second test: all carry bits
    for i in 0..num_bits
    {
        let x_reg : i64 = generate_n_ones(i);
        let y_reg : i64 = generate_n_ones(i);

        machine.set_init_registers(x_reg, y_reg);
        let machine_output = machine.compute_output();

        if machine_output != x_reg + y_reg
        {
            return false;
        }
    }

    // Third test: all carry bits & random numbers
    for i in 0..num_bits
    {
        let x_reg : i64 = generate_n_ones(i);
        let y_reg : i64 = rng.gen_range(0..modulo);

        machine.set_init_registers(x_reg, y_reg);
        let machine_output = machine.compute_output();

        if machine_output != x_reg + y_reg
        {
            return false;
        }
    }

    // Tests passed, this machine probably works......
    return true;
}


fn generate_n_ones(n: usize) -> i64
{
    return (1 << n) - 1;
}

fn generate_swaps(size: usize) -> Vec<(usize, usize)>
{
    let mut swaps = Vec::new();

    for i in 0..size
    {
        for j in (i + 1)..size
        {
            swaps.push((i, j));
        }
    }

    return swaps;
}