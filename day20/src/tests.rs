#[cfg(test)]
mod tests 
{
    use crate::part1;
    use crate::part2;

    const TEST_STR: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part_1_test() 
    {
        assert_eq!(44, part1::compute_answer(&String::from(TEST_STR), 2));
        assert_eq!(30, part1::compute_answer(&String::from(TEST_STR), 4));
        assert_eq!(16, part1::compute_answer(&String::from(TEST_STR), 6));
        assert_eq!(14, part1::compute_answer(&String::from(TEST_STR), 8));
        assert_eq!(1, part1::compute_answer(&String::from(TEST_STR), 64));
    }

    #[test]
    fn part_2_test() 
    {
        const PART_1_SHORTCUT_LEN : i32 = 2;
        assert_eq!(44, part2::compute_answer(&String::from(TEST_STR), 2, PART_1_SHORTCUT_LEN));
        assert_eq!(30, part2::compute_answer(&String::from(TEST_STR), 4, PART_1_SHORTCUT_LEN));
        assert_eq!(16, part2::compute_answer(&String::from(TEST_STR), 6, PART_1_SHORTCUT_LEN));
        assert_eq!(14, part2::compute_answer(&String::from(TEST_STR), 8, PART_1_SHORTCUT_LEN));
        assert_eq!(1,  part2::compute_answer(&String::from(TEST_STR), 64, PART_1_SHORTCUT_LEN));
    }
}