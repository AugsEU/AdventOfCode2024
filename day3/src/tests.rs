#[cfg(test)]
mod tests 
{
    use crate::problem;

    const TEST_STR: &str = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_1_test() 
    {
        let result = problem::resolve_mults(String::from(TEST_STR));
        assert_eq!(result, 161);
    }

    #[test]
    fn part_2_test() 
    {
         let result = problem::resolve_mults_do_dont(String::from(TEST_STR));
         assert_eq!(result, 48);
    }
}