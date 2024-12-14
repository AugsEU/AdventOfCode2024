#[cfg(test)]
mod tests 
{
    use crate::part1;

    const TEST_STR: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part_1_test() 
    {
        let result = part1::get_robot_safety_factor(&String::from(TEST_STR), 11, 7);
        assert_eq!(result, 12);
    }
}