#[cfg(test)]
mod tests 
{
    use crate::problem;

    const TEST_STR: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn part_1_test() 
    {

        let result = problem::count_safe_str(String::from(TEST_STR));
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_test() 
    {
        let result = problem::count_safe_damp_str(String::from(TEST_STR));
        assert_eq!(result, 4);
    }
}