mod tests;
mod problem;

use std::fs;

fn read_all_lines(path : String) -> String
{
    let contents : String = fs::read_to_string(path).expect("Should have been able to read the file");
    return contents;
}

fn main()
{
    let file_path : String = String::from("./input.txt");
    let file_contents = read_all_lines(file_path);

    let part1 = problem::count_safe_str(file_contents.clone());
    let part2 = problem::count_safe_damp_str(file_contents.clone());

    println!("{}", part1);
    println!("{}", part2);
}