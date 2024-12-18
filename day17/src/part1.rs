use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;

#[derive(Debug)]
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

pub fn compute_answer(input: &str) -> String
{
    let mut computer = Computer::from(input);

    while !computer.proc_step()
    {
    }

    let output_str =  computer.output_buf.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
    return output_str;
}