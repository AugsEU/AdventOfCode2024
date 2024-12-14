use num::{Integer, Signed};

pub fn positive_mod<T>(a: T, b: T) -> T
where
    T: Integer + Signed + Copy,
{
    let mut r = a % b;
    r = r + b;
    r = r % b;

    return r;
}


pub fn get_str_nums(input: &str) -> Vec<Vec<i32>>
{
    // Split the input into lines
    let result = input.lines()
        .map(|line| 
            {
            line.split_whitespace()
                    .flat_map(|token| {
                        token
                            .split(|c: char| !c.is_ascii_digit() && c != '-') // Split on non-numeric characters
                            .filter_map(|num| num.parse::<i32>().ok()) // Parse valid numbers
                    })
                    .collect::<Vec<i32>>()
            })
            .collect();
    
    return result;
}
