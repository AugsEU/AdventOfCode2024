use glam::{I64Vec2, DVec2};
use regex::Regex;

pub struct PrizeMachine
{
    m_a: I64Vec2,
    m_b: I64Vec2,
    m_prize: I64Vec2
}

impl PrizeMachine
{
    const A_BUTTON_COST: i64 = 3;
    const B_BUTTON_COST: i64 = 1;

    pub fn from(input: &str) -> Self
    {
        let button_a_parse = Regex::new(r"Button A: X\+(\d+).*Y\+(\d+)").expect("Invalid input str.");
        let captures = button_a_parse.captures(input).unwrap();
        let button_a_vec: I64Vec2 = I64Vec2::new(captures[1].parse().unwrap(), captures[2].parse().unwrap());

        let button_b_parse = Regex::new(r"Button B: X\+(\d+).*Y\+(\d+)").expect("Invalid input str.");
        let captures = button_b_parse.captures(input).unwrap();
        let button_b_vec: I64Vec2 = I64Vec2::new(captures[1].parse().unwrap(), captures[2].parse().unwrap());

        let prize_parse = Regex::new(r"Prize: X=(\d+).*Y=(\d+)").expect("Invalid input str.");
        let captures = prize_parse.captures(input).unwrap();
        let prize_vec: I64Vec2 = I64Vec2::new(captures[1].parse().unwrap(), captures[2].parse().unwrap());

        Self
        {
            m_a: button_a_vec,
            m_b: button_b_vec,
            m_prize: prize_vec
        }
    }

    pub fn parse_list(input: &String) -> Vec<PrizeMachine>
    {
        return input.split("\n\n").
                        into_iter().
                        map(|str| Self::from(str)).
                        collect();
    }

    pub fn get_minimum_cost(&self) -> Option<i64>
    {
        if let Some(sol) = self.get_solution()
        {
            return Some(sol.x * Self::A_BUTTON_COST + sol.y * Self::B_BUTTON_COST);
        }

        return None;
    }

    pub fn get_solution(&self) -> Option<I64Vec2>
    {
        if self.m_prize == I64Vec2::ZERO
        {
            return Some(I64Vec2::ZERO);
        }

        let determinant = self.m_a.x * self.m_b.y - self.m_a.y * self.m_b.x;

        // Case where buttons are linearly dependant.
        if determinant == 0
        {
            return self.get_linear_cost();
        }

        let determinant_f = determinant as f64;
        let a_f = DVec2::new(self.m_a.x as f64, self.m_a.y as f64);
        let b_f = DVec2::new(self.m_b.x as f64, self.m_b.y as f64);
        let p_f = DVec2::new(self.m_prize.x as f64, self.m_prize.y as f64);

        let a_steps = (b_f.y * p_f.x - b_f.x * p_f.y) / determinant_f;
        let b_steps = (a_f.x * p_f.y - a_f.y * p_f.x) / determinant_f;

        let a_steps = try_round(a_steps);
        let b_steps = try_round(b_steps);

        if a_steps.is_some() && b_steps.is_some()
        {
            return Some(I64Vec2::new(a_steps.unwrap(), b_steps.unwrap()));
        }

        return None;
    }

    fn get_linear_cost(&self) -> Option<I64Vec2>
    {
        assert!(self.m_prize != I64Vec2::ZERO);

        // Note: Check b button first to minimise cost.
        if self.m_prize.x * self.m_b.y == self.m_prize.y * self.m_b.x
        {
            let mult = get_mult_to(&self.m_b, &self.m_prize);
            if let Some(num_steps) = try_round(mult)
            {
                return Some(I64Vec2::new(0, num_steps));
            }
        }

        if self.m_prize.x * self.m_a.y == self.m_prize.y * self.m_a.x
        {
            let mult = get_mult_to(&self.m_a, &self.m_prize);
            if let Some(num_steps) = try_round(mult)
            {
                return Some(I64Vec2::new(num_steps, 0));
            }
        }

        return None;
    }


    pub fn add_to_prize(&mut self, delta: I64Vec2)
    {
        self.m_prize += delta;
    }

}

fn get_mult_to(vec: &I64Vec2, dest: &I64Vec2) -> f64
{
    assert!(*vec != I64Vec2::ZERO);
    assert!(*dest != I64Vec2::ZERO);

    if vec.x == 0 
    { 
        return (dest.y as f64) / (vec.y as f64);
    }
    else
    {
        return (dest.x as f64) / (vec.x as f64);
    }
}

// Try to round a float but only if it is within a rounding error of an int.
fn try_round(num: f64) -> Option<i64>
{
    let rounded = num.round();
    if (rounded - num).abs() > 0.00001
    {
        return None;
    }

    return Some(rounded as i64);
}