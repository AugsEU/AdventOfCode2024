const MAX_CHANGE : i32 = 3;

pub fn count_safe_str(input: String) -> i32
{
    return count_safe(parse_lists(&input));
}

pub fn count_safe_damp_str(input: String) -> i32
{
    return count_safe_damp(parse_lists(&input));
}

fn count_safe(input: Vec<Vec<i32>>) -> i32
{
    return input.iter().filter(|&x| is_safe(&x)).count() as i32;
}

fn count_safe_damp(input: Vec<Vec<i32>>) -> i32
{
    return input.iter().filter(|&x| is_safe_damp(&x)).count() as i32;
}

fn is_safe_damp(list: &Vec<i32>) -> bool
{
    for i in 0..list.len()
    {
        let mut reduce_list = list.clone();
        reduce_list.remove(i);

        if is_safe(&reduce_list)
        {
            return true;
        }
    }

    return false;
}

fn is_safe(list: &Vec<i32>) -> bool
{
    return is_safe_incresing(&list) || is_safe_decreasing(&list);
}

fn is_safe_incresing(list: &Vec<i32>) -> bool
{    
    for i in 1..list.len()
    {
        let prev = list[i-1];
        let curr = list[i];
        if prev >= curr
        {
            return false;
        }

        if curr - prev > MAX_CHANGE
        {
            return false;
        }
    }

    return true;
}

fn is_safe_decreasing(list: &Vec<i32>) -> bool
{
    for i in 1..list.len()
    {
        let prev = list[i-1];
        let curr = list[i];
        if prev <= curr
        {
            return false;
        }

        if prev - curr > MAX_CHANGE
        {
            return false;
        }
    }

    return true;
}

fn parse_lists(input: &String) -> Vec<Vec<i32>>
{
    let lines : Vec<String> = input.lines().map(String::from).collect();

    let mut result : Vec<Vec<i32>> = Vec::new();

    for line in lines.iter()
    {
        let numbers : Vec<&str> = line.split_whitespace().collect();
        let numbers : Vec<i32> = numbers.iter().map(|line| line.parse().unwrap()).collect();

        result.push(numbers);
    }

    return result;
}