mod problem;

use std::fs;

fn read_all_lines(path : String) -> String
{
    let contents : String = fs::read_to_string(path).expect("Should have been able to read the file");
    return contents;
}

pub fn get_results_program() -> (i64, i64)
{
    let file_path : String = String::from("./input.txt");
    let file_contents = read_all_lines(file_path);

    let part1 = problem::sum_total_valid_tests_mult_or_add(&file_contents);
    let part2 = problem::sum_total_valid_tests_mult_or_add_or_concat(&file_contents);

    return (part1, part2);
}