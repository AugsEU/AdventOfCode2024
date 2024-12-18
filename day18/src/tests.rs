#[cfg(test)]
mod tests 
{
    use crate::part1;
    use crate::part2;

    const TEST_STR: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part_1_test() 
    {
        let result = part1::compute_answer(&String::from(TEST_STR), 7, 7, 12);
        assert_eq!(result, 22);
    }

    // #[test]
    // fn part_2_test() 
    // {
    //     let result = part2::compute_answer(&String::from(TEST_STR));
    //     assert_eq!(result, 0);
    // }
}