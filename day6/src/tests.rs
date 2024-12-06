#[cfg(test)]
mod tests 
{
    use crate::problem::*;

    const TEST_STR: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_1_test() 
    {
        let result = problem::count_number_of_guard_positions(&String::from(TEST_STR));
        assert_eq!(result, 41);
    }

    #[test]
    fn part_2_test() 
    {
         let result = problem::count_number_of_infinite_obstructions(&String::from(TEST_STR));
         assert_eq!(result, 6);
    }
}