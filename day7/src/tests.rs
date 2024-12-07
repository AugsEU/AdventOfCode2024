#[cfg(test)]
mod tests 
{
    use crate::problem;

    const TEST_STR: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    const SIMPLE_STR: &str = r"190: 10 19";

    const SIMPLE_CONCAT_STR: &str = r"7290: 6 8 6 15";

    #[test]
    fn simple_test() 
    {
        let result = problem::sum_total_valid_tests_mult_or_add(&String::from(SIMPLE_STR));
        assert_eq!(result, 190);
    }

    #[test]
    fn simple_concat_test() 
    {
        let result = problem::sum_total_valid_tests_mult_or_add_or_concat(&String::from(SIMPLE_CONCAT_STR));
        assert_eq!(result, 7290);
    }

    #[test]
    fn concat_digits_test() 
    {
        assert_eq!(problem::concat_base10(12, 345), 12345);
        assert_eq!(problem::concat_base10(1, 1), 11);
        assert_eq!(problem::concat_base10(10, 1), 101);
        assert_eq!(problem::concat_base10(13, 10), 1310);
    }


    #[test]
    fn part_1_test() 
    {
        let result = problem::sum_total_valid_tests_mult_or_add(&String::from(TEST_STR));
        assert_eq!(result, 3749);
    }



    #[test]
    fn part_2_test() 
    {
        let result = problem::sum_total_valid_tests_mult_or_add_or_concat(&String::from(TEST_STR));
        assert_eq!(result, 11387);
    }
}