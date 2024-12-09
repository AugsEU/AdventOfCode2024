#[cfg(test)]
mod tests 
{
    use crate::problem;

    const TEST_STR: &str = r"2333133121414131402";

    #[test]
    fn part_1_test() 
    {
        let result = problem::defrag_and_checksum(&String::from(TEST_STR));
        assert_eq!(result, 1928);
    }

    #[test]
    fn part_2_test() 
    {
         let result = problem::defrag_whole_files_and_checksum(&String::from(TEST_STR));
         assert_eq!(result, 2858);
    }
}