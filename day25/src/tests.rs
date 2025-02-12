#[cfg(test)]
mod tests 
{
    use crate::part1;
    use crate::part2;

    const TEST_STR: &str = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn part_1_test() 
    {
        let result = part1::compute_answer(TEST_STR);
        assert_eq!(result, 3);
    }

    // #[test]
    // fn part_2_test() 
    // {
    //     let result = part2::compute_answer(TEST_STR);
    //     assert_eq!(result, 0);
    // }
}