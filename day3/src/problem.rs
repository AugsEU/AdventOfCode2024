use regex::Regex;

pub fn resolve_mults(input : String) -> i32
{
    let find_mult_commands = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let matches : Vec<&str> = find_mult_commands.find_iter(&input).map(|mat| mat.as_str()).collect();

    let mut total_sum : i32 = 0; 
    for re_match in matches.iter()
    {
        let find_numbers = Regex::new(r"(\d+)").unwrap();
        let captures = find_numbers.captures_iter(re_match);

        let numbers: Vec<i32> = captures
            .filter_map(|cap| cap.get(1)) // Get the first capturing group
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();

        total_sum += numbers[0] * numbers[1];
    }   

    return total_sum;
}

pub fn resolve_mults_do_dont(input : String) -> i32
{
    let remove_donts  = Regex::new(r"(?s)don't\(\)(.)*?do\(\)").unwrap();
    let trimmed_string = remove_donts.replace_all(&input, "");

    return resolve_mults(trimmed_string.to_string());
}