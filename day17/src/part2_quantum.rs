use std::u64;
use std::usize;

use crate::int_grid::*;
use crate::char_grid::*;
use crate::aoc_utils::*;
use crate::direction::*;


/////////////////////////////////////////////////////////////////////////////
// Qubit - A single bit either a known value or a index to register A
/////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, PartialEq, Eq, Clone)]
enum Qubit
{
    Known(u8), // A known value, 1 or 0
    RegA(usize) // A bit pointing to a bit inside the final A register.
}


/////////////////////////////////////////////////////////////////////////////
// Q64 - Quantum 64 bit integer. Each bit is known or an unknown value of RegA
/////////////////////////////////////////////////////////////////////////////
const Q_INT_SIZE: usize = 64;

#[derive(Debug, Clone)]
struct Q64
{
    qbits: [Qubit; Q_INT_SIZE],
}

impl Q64
{
    fn make_register_a() -> Self
    {
        Self
        {
            qbits: std::array::from_fn(|i| Qubit::RegA(i))
        }
    }

    fn make_zero() -> Self
    {
        Self
        {
            qbits: std::array::from_fn(|i| Qubit::Known(0))
        }
    }

    fn from_u64(n: u64) -> Self
    {
        Self
        {
            qbits: std::array::from_fn(|i| if (1 << (Q_INT_SIZE - i - 1) & n) == 0 { Qubit::Known(0) } else { Qubit::Known(1) })
        }
    }

    fn bit_shift_right(&mut self, n: usize)
    {
        for i in (0..self.qbits.len()).rev()
        {
            if i < n
            {
                self.qbits[i] = Qubit::Known(0);
            }
            else
            {
                self.qbits[i] = self.qbits[i-n].clone();
            }
        }
    }

    fn to_mod_8(&self) -> Q64
    {
        let mut result = Q64::make_zero();

        // Clone first 3 bits.
        for i in (Q_INT_SIZE-3)..Q_INT_SIZE
        {
            let src = self.qbits[i].clone();
            result.qbits[i] = src;
        }

        return result;
    }

    // Try to convert to u64 if fully known
    fn to_u64(&self) -> Option<u64>
    {
        let mut result : u64 = 0;

        for i in 0..self.qbits.len()
        {
            if let Qubit::Known(b) = self.qbits[i]
            {
                if b != 0
                {
                    result = result | (1 << (Q_INT_SIZE - i - 1));
                }
            }
            else
            {
                return None;   
            }
        }

        return Some(result);
    }

    fn to_string(&self) -> String
    {
        let mut result = String::new();

        for (i, qbit) in self.qbits.iter().enumerate()
        {
            // Append the appropriate representation for the current Qubit
            match qbit
            {
                Qubit::Known(value) => result.push_str(&value.to_string()),
                Qubit::RegA(index) =>
                {
                    let rega_index = *index;
                    // Map the index to a letter: a-z, A-Z, then wrap around.
                    let letter = match rega_index
                    {
                        0..=25 => ('a' as u8 + rega_index as u8) as char,        // 'a' to 'z'
                        26..=64 => (':' as u8 + (rega_index - 26) as u8) as char, // 'A' to 'Z'
                        _ => panic!("RegA index exceeds 52 unique letter mapping."),
                    };

                    result.push(letter);
                }
            }

            // Add an underscore after every 4th qbit, except the last group
            if (i + 1) % 4 == 0 && i + 1 != Q_INT_SIZE {
                result.push(' ');
            }
        }

        result
    }
}



/////////////////////////////////////////////////////////////////////////////
// Operator - An operator in an equation
/////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, PartialEq, Eq, Clone)]
enum Operator
{
    Xor,
    RShift
}



/////////////////////////////////////////////////////////////////////////////
// Equation - Represents several symbols XORed to make a desired result.
/////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
struct Equation
{
    operands: Vec<Q64>,
    operators: Vec<Operator>
}

impl Equation
{
    fn new(base: Q64) -> Self
    {
        Self
        {
            operands: vec![base],
            operators: vec![]
        }
    }

    fn append_num(&mut self, op: Operator, num: Q64)
    {
        self.operands.push(num);
        self.operators.push(op);

        self.simplify_eq();
    }

    fn append_equation(&mut self, op: Operator, eq: &Equation)
    {
        self.operators.push(op);
        self.operands.extend(eq.operands.iter().map(|o| o.clone()));
        self.operators.extend(eq.operators.iter().map(|o| o.clone()));

        self.simplify_eq();
    }

    
    fn simplify_eq(&mut self)
    {
        // Resolve stuff we can immediately do.
        loop
        {
            if self.operands.len() <= 1
            {
                break;
            }   

            let mut left = self.operands[&self.operands.len() - 2].clone();
            let right = self.operands[&self.operands.len() - 1].clone();
            let op = self.operators[&self.operators.len() - 1].clone();

            if op == Operator::RShift
            {
                if let Some(right_known) = right.to_u64()
                {
                    left.bit_shift_right(right_known as usize);
                    self.operands.pop();
                    self.operators.pop();

                    let last= &self.operands.len() - 1;
                    self.operands[last] = left;
                }
                else
                {
                    break;    
                }
            }
            else
            {
                break;    
            }
        }
    }

    fn to_q64(&self) -> Q64
    {
        assert!(self.operands.len() == 1, "Equation is not only 1 operand");
        return self.operands[0].clone();
    }

    fn to_string(&self) -> String
    {
        if self.operands.is_empty()
        {
            return String::from("");
        }

        let mut result = self.operands[0].to_string();

        for (i, operator) in self.operators.iter().enumerate() 
        {
            let operator_str = match operator 
            {
                Operator::Xor => "^",     // XOR operator representation
                Operator::RShift => ">>", // Right shift operator representation
            };
            
            if let Some(operand) = self.operands.get(i + 1)
            {
                result.push(' ');
                result.push_str(operator_str);
                result.push(' ');
                result.push_str(&operand.to_string());
            }
        }

        result
    }
}


/////////////////////////////////////////////////////////////////////////////
// Constraint - Represents a constraint on an equation
/////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
struct Constraint
{
    equation: Equation,
    result: u64
}

impl Constraint
{
    pub fn to_string(&self) -> String
    {
        return format!("{} = {}", self.equation.to_string(), self.result);
    }
}
/////////////////////////////////////////////////////////////////////////////
// QuantumComputer - Doesn't store values but instead stores states 
//                   that could be the result of xoring many symbols.
/////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
struct QuantumComputer
{
    ra: Equation,
    rb: Equation,
    rc: Equation,
    pc: usize,
    program: Vec<u64>,
    expected_output: Vec<u64>,
    constraints: Vec<Constraint>, // Constraints on the qubits.
}

impl QuantumComputer
{
    pub fn from(input: &str) -> Self
    {
        let str_nums = get_str_nums(input);

        let a = str_nums[0][0] as u64;
        let b = str_nums[1][0] as u64;
        let c = str_nums[2][0] as u64;

        let prog : Vec<u64> = str_nums[4].iter().map(|i| *i as u64).collect();
        let expected: Vec<u64> = prog.iter().rev().map(|n| *n).collect();

        Self
        {
            ra: Equation::new(Q64::make_register_a()),
            rb: Equation::new(Q64::make_zero()),
            rc: Equation::new(Q64::make_zero()),
            pc: 0,
            program: prog,
            expected_output: expected,
            constraints: Vec::new()
        }
    }

    // Process a step and return if halted
    pub fn proc_step(&mut self) -> bool
    {
        if self.pc + 1 >= self.program.len()
        {
            todo!();
        }

        if self.expected_output.len() == 0
        {
            return true;
        }

        let opcode = self.program[self.pc];
        let operand_u64 = self.program[self.pc + 1];

        let operand = Equation::new(Q64::from_u64(operand_u64));
        let combo_operand = self.combo_operand_value(operand_u64);

        match opcode
        {
            0 => // adv
            {
                self.ra.append_equation(Operator::RShift, &combo_operand);
                self.pc += 2;
            }
            1 => // bxl
            {
                self.rb.append_equation(Operator::Xor, &operand);
                self.pc += 2;
            }
            2 => // bst
            {
                self.rb = Equation::new(combo_operand.clone().to_q64().to_mod_8());
                self.pc += 2;
            }
            3 => // jnz
            {
                self.pc = operand_u64 as usize; // Branch unconditionally.
            }
            4 => // bxc
            {
                self.rb.append_equation(Operator::Xor, &self.rc);
                self.pc += 2;
            }
            5 => // out
            {
                let expected_num = self.expected_output.pop().unwrap();
                let new_constraint = Constraint
                {
                    equation: combo_operand.clone(),
                    result: expected_num
                };

                self.constraints.push(new_constraint);

                self.pc += 2;
            }
            6 => // bdv
            {
                self.rb = self.ra.clone();
                self.rb.append_equation(Operator::RShift, &combo_operand);
                self.pc += 2;
            }
            7 => // cdv
            {
                self.rc = self.ra.clone();
                self.rc.append_equation(Operator::RShift, &combo_operand);
                self.pc += 2;
            }
            _ =>
            {
                panic!("Invalid opcode {opcode}");
            }
        }

        return false;
    }

    fn combo_operand_value(&self, operand: u64) -> Equation
    {
        match operand
        {
            (0..=3) => { return Equation::new(Q64::from_u64(operand)); }
            4 => { return self.ra.clone(); }
            5 => { return self.rb.clone(); }
            6 => { return self.rc.clone(); }
            _ => { panic!("Invalid combo operand."); }
        }
    }

}

pub fn compute_answer(input: &str) -> u64
{
    let mut computer = QuantumComputer::from(input);

    for i in 0..7 
    {
        println!("STEP: {}", i);
        println!("");
        println!("");
        println!("RA: {}", computer.ra.to_string());
        println!("RB: {}", computer.rb.to_string());
        println!("RC: {}", computer.rc.to_string());
        println!("");
        println!("");
        computer.proc_step();
    }

    for c in computer.constraints.iter()
    {
        println!("   {}", c.to_string());
    }

    return 3;
}
