use std::cmp;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

#[derive(Debug, Clone)]
struct Computer
{
    ra: u64,
    rb: u64,
    rc: u64,
    pc: usize,
    program: Vec<u64>,
    output_buf: Vec<u64>
}

impl Computer
{
    pub fn from(input: &str) -> Self
    {
        let str_nums = get_str_nums(input);

        let a = str_nums[0][0] as u64;
        let b = str_nums[1][0] as u64;
        let c = str_nums[2][0] as u64;

        let prog = str_nums[4].iter().map(|i| *i as u64).collect();

        Self
        {
            ra: a,
            rb: b,
            rc: c,
            pc: 0,
            program: prog,
            output_buf: Vec::new()
        }
    }

    // Process a step and return if halted
    pub fn proc_step(&mut self) -> bool
    {
        if self.pc + 1 >= self.program.len()
        {
            // reached end
            return true;
        }

        let opcode = self.program[self.pc];
        let operand = self.program[self.pc + 1];

        match opcode
        {
            0 => // adv
            {
                self.ra = self.ra >> self.combo_operand_value(operand);
                self.pc += 2;
            }
            1 => // bxl
            {
                self.rb = self.rb ^ operand;
                self.pc += 2;
            }
            2 => // bst
            {
                self.rb = self.combo_operand_value(operand) % 8;
                self.pc += 2;
            }
            3 => // jnz
            {
                if self.ra != 0
                {
                    self.pc = operand as usize;
                }
                else
                {
                    self.pc += 2;    
                }
            }
            4 => // bxc
            {
                self.rb = self.rb ^ self.rc;
                self.pc += 2;
            }
            5 => // out
            {
                self.output_buf.push(self.combo_operand_value(operand) % 8);
                self.pc += 2;
            }
            6 => // bdv
            {
                self.rb = self.ra >> self.combo_operand_value(operand);
                self.pc += 2;
            }
            7 => // cdv
            {
                self.rc = self.ra >> self.combo_operand_value(operand);
                self.pc += 2;
            }
            _ =>
            {
                panic!("Invalid opcode {opcode}");
            }
        }


        return false;
    }

    fn combo_operand_value(&self, operand: u64) -> u64
    {
        match operand
        {
            (0..=3) => { return operand; }
            4 => { return self.ra; }
            5 => { return self.rb; }
            6 => { return self.rc; }
            _ => { panic!("Invalid combo operand."); }
        }
    }

}

pub fn compute_answer(input: &str) -> u64
{
    const ANSWER_SHIFT: u64 = 3; // abuse specific knowledge of program. A register always shift by 3

    // New dumbass approach.
    let computer = Computer::from(input);

    let expected_output = computer.program.clone();
    let num_answer_bits = (expected_output.len() as u64) * ANSWER_SHIFT;

    let mut bits_found = 0;

    let mut answer: u64 = 0;

    println!("Looking for {} bits", num_answer_bits);

    let mut section_search_size = ANSWER_SHIFT * 2;

    while bits_found < num_answer_bits - ANSWER_SHIFT
    {
        let mut new_section = 0;
        while new_section < (1 << section_search_size)
        {
            let test_input = answer | (new_section << (num_answer_bits - bits_found - section_search_size));

            let output = simulate_computer(&computer, test_input);

            if output.len() != expected_output.len()
            {
                new_section += 1;
                continue;
            }

            let mut tails_match = true;
            let outputs_to_satisfy = cmp::min((1 + (bits_found + section_search_size) / ANSWER_SHIFT) as usize, expected_output.len());

            for i in 0..outputs_to_satisfy
            {
                let i = i as usize;
                let size = expected_output.len();
                let idx = size - i - 1;

                if expected_output[idx] != output[idx]
                {
                    tails_match = false;
                    break;
                }
            }

            if tails_match
            {
                answer = test_input;
                bits_found += section_search_size;
                println!("Found {}", bits_found);
                println!("Input: {} | Out: {:?} | Want {}", test_input, &output, outputs_to_satisfy);

                section_search_size = 0;
                break;
            }

            new_section += 1;
        }


        // Bump up section_search_size
        section_search_size += ANSWER_SHIFT * 2;
        section_search_size = cmp::min(section_search_size, num_answer_bits - bits_found);
    }

    let test_output = simulate_computer(&computer, answer);
    println!("Ans: {} | {:?}", answer, test_output);
    assert!(test_output == expected_output);

    return answer;
}

fn simulate_computer(computer: &Computer, reg_a: u64) -> Vec<u64>
{
    let mut clone_computer = computer.clone();

    clone_computer.ra = reg_a;

    while !clone_computer.proc_step()
    {
    }

    return clone_computer.output_buf;
}

fn round_up_to_section_size(num: u64, section_size: u64) -> u64 
{
    if num % section_size == 0
    {
        return num;
    }
    else
    {
        return ((num + section_size) / section_size) * section_size;
    }
}