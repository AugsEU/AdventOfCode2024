#[cfg(test)]
mod tests 
{
    use crate::problem;

    const TEST_STR: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part_1_test() 
    {
        let result = problem::count_xmas(&String::from(TEST_STR));
        assert_eq!(result, 18);
    }


    #[test]
    fn part_2_test() 
    {
        let result = problem::count_mas_crosses(&String::from(TEST_STR));
        assert_eq!(result, 9);
    }
}