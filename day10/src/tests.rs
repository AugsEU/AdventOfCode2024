#[cfg(test)]
mod tests 
{
    use crate::problem;

    const TEST_STR: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part_1_test() 
    {
        let result = problem::total_trail_score(&String::from(TEST_STR));
        assert_eq!(result, 36);
    }

    #[test]
    fn part_2_test() 
    {
         let result = problem::total_trail_score_part2(&String::from(TEST_STR));
         assert_eq!(result, 81);
    }
}