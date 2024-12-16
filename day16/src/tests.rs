#[cfg(test)]
mod tests 
{
    use crate::part1;
    use crate::part2;

    const TINY_MAZE_STR: &str = r"######
#...E#
#.#..#
#.#..#
#S#.##
######";

    const SMALL_MAZE_STR: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const BIG_MAZE_STR: &str = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part_1_test() 
    {
        assert_eq!(2006, part1::compute_answer(&String::from(TINY_MAZE_STR)));
        assert_eq!(7036, part1::compute_answer(&String::from(SMALL_MAZE_STR)));
        assert_eq!(11048, part1::compute_answer(&String::from(BIG_MAZE_STR)));
    }

    #[test]
    fn part_2_test() 
    {
        assert_eq!(45, part2::compute_answer(&String::from(SMALL_MAZE_STR)));
        assert_eq!(64, part2::compute_answer(&String::from(BIG_MAZE_STR)));
    }
}