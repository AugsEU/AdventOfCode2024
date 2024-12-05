#[cfg(test)]
mod tests 
{
    use crate::problem;

    const TEST_STR: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part_1_test() 
    {
        let result = problem::count_middle_pages_of_updates(&String::from(TEST_STR));
        assert_eq!(result, 143);
    }

    #[test]
    fn part_2_test() 
    {
        let result = problem::count_middle_pages_of_fixuped_updates(&String::from(TEST_STR));
        assert_eq!(result, 123);
    }
}