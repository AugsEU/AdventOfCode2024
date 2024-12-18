#[cfg(test)]
mod tests 
{
    use crate::part1;
    use crate::part2;

    const TEST_STR: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST_STR_2: &str = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part_1_test() 
    {
        let result = part1::compute_answer(TEST_STR);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_2_test() 
    {
        let result = part2::compute_answer(TEST_STR_2);
        assert_eq!(result, 117440);
    }
}