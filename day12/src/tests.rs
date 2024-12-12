#[cfg(test)]
mod tests 
{
    use crate::problem;

    const TEST_STR: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part_1_test() 
    {
        let result = problem::total_fence_price(&String::from(TEST_STR));
        assert_eq!(result, 1930);
    }

    #[test]
    fn part_2_test() 
    {
         let result = problem::total_fence_price_discounted(&String::from(TEST_STR));
         assert_eq!(result, 1206);
    }
}