#[cfg(test)]
mod tests 
{
    use crate::part1;
    use crate::part2;

    const TEST_STR: &str = r"029A
980A
179A
456A
379A";

    #[test]
    fn part_1_test() 
    {
        let result = part1::compute_answer(&String::from(TEST_STR));
        assert_eq!(result, 126384);
    }

    #[test]
    fn part_2_test() 
    {
        let result = part2::compute_answer(&String::from(TEST_STR), 2);
        assert_eq!(result, 126384);
    }
}