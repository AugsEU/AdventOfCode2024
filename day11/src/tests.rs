#[cfg(test)]
mod tests 
{
    use crate::problem;

    const TEST_STR: &str = r"125 17";

    #[test]
    fn part_1_test_small() 
    {
        let result = problem::get_num_stones(&String::from(TEST_STR), 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn part_1_test() 
    {
        let result = problem::get_num_stones(&String::from(TEST_STR), 25);
        assert_eq!(result, 55312);
    }

    #[test]
    fn part_2_test_small() 
    {
        let result = problem::get_num_stones_cheat_sheets(&String::from(TEST_STR), 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn part_2_test() 
    {
        let result = problem::get_num_stones_cheat_sheets(&String::from(TEST_STR), 25);
        assert_eq!(result, 55312);
    }
}