#[cfg(test)]
mod tests 
{
    use crate::part1;
    use crate::part2;

    const TEST_STR: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part_1_test() 
    {
        let result = part1::compute_answer(&String::from(TEST_STR));
        assert_eq!(result, 6);
    }

    #[test]
    fn part_2_test() 
    {
        let result = part2::compute_answer(&String::from(TEST_STR));
        assert_eq!(result, 16);
    }
}