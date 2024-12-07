
struct TestLine
{
    m_expected_value: i64,
    m_operands: Vec<i64>
}


impl TestLine
{
    fn from(line: &str) -> Self
    {
        let line_split : Vec<&str> = line.split(':').collect();
        assert!(line_split.len() == 2);
        let expected_val: i64 = line_split[0].parse().expect("Parse error.");
        let operands = line_split[1].split(' ')
                                                .filter(|n| n.len() > 0)
                                                .map(|n| n.parse::<i64>().expect("Tried parsing {n}"))
                                                .collect();

        return Self
        {
            m_expected_value: expected_val,
            m_operands: operands
        };
    }

    fn could_be_valid_mult_or_add(&self) -> bool
    {
        return self.try_mult_or_add_recurse(self.m_operands[0], 1);
    }

    fn try_mult_or_add_recurse(&self, curr_value: i64, idx: usize) -> bool
    {
        if curr_value > self.m_expected_value
        {
            return false;
        }

        // Base case
        if idx == self.m_operands.len()
        {
            //dbg!((curr_value, self.m_expected_value));
            return curr_value == self.m_expected_value;
        }

        let next_value: i64 = self.m_operands[idx];
        
        // Note: do the most expensive operation last to save time.
        return self.try_mult_or_add_recurse(curr_value + next_value, idx + 1) || 
                self.try_mult_or_add_recurse(curr_value * next_value, idx + 1);
    }

    fn could_be_valid_mult_or_add_or_concat(&self) -> bool
    {
        return self.try_mult_or_add_recurse_or_concat(self.m_operands[0], 1);
    }

    fn try_mult_or_add_recurse_or_concat(&self, curr_value: i64, idx: usize) -> bool
    {
        if curr_value > self.m_expected_value
        {
            return false;
        }

        // Base case
        if idx >= self.m_operands.len()
        {
            return curr_value == self.m_expected_value;
        }

        // Note: do the most expensive operation last to save time.
        let next_value: i64 = self.m_operands[idx];
        return self.try_mult_or_add_recurse_or_concat(curr_value + next_value, idx + 1) || 
                self.try_mult_or_add_recurse_or_concat(curr_value * next_value, idx + 1) ||
                self.try_mult_or_add_recurse_or_concat(concat_base10(curr_value, next_value), idx + 1);
    }
}


// Problem
pub fn sum_total_valid_tests_mult_or_add(input: &String) -> i64
{
    let tests = parse_tests(&input);

    let sum_valid_test: i64 = tests.iter()
                                    .filter(|t| t.could_be_valid_mult_or_add())
                                    .map(|valid_test| valid_test.m_expected_value)
                                    .sum();

    return sum_valid_test;
}

pub fn sum_total_valid_tests_mult_or_add_or_concat(input: &String) -> i64
{
    let tests = parse_tests(&input);

    let sum_valid_test: i64 = tests.iter()
                                    .filter(|t| t.could_be_valid_mult_or_add_or_concat())
                                    .map(|valid_test| valid_test.m_expected_value)
                                    .sum();

    return sum_valid_test;
}


// Parse
fn parse_tests(input: &String) -> Vec<TestLine>
{
    return input.lines().into_iter().map(|l| TestLine::from(l)).collect();
}

// Maths
pub fn concat_base10(mut n1: i64, n2: i64) -> i64
{
    let digits2 = count_digits(n2);
    for _ in 0..digits2
    {
        n1 = n1 * 10;
    }

    return n1 + n2;
}

fn count_digits(mut n2: i64) -> i64
{
    let mut digits = 1;
    while n2 >= 10
    {
        n2 = n2 / 10;
        digits += 1;
    }

    return digits;
}