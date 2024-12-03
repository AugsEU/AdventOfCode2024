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

    let part1 = problem::resolve_mults(file_contents.clone());
    let part2 = problem::resolve_mults_do_dont(file_contents.clone());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
