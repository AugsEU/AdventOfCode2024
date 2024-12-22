#[cfg(test)]
mod tests 
{
    use crate::part1;
    use crate::part2;

    const TEST_STR: &str = r"1
10
100
2024";

    const PROFIT_TEST_STR: &str = r"1
2
3
2024";

    #[test]
    fn part_1_test() 
    {
        let result = part1::compute_answer(&String::from(TEST_STR));
        assert_eq!(result, 37327623);
    }

    #[test]
    fn part_2_test() 
    {
        let result = part2::compute_answer(&String::from(PROFIT_TEST_STR));
        assert_eq!(result, 23);
    }
}