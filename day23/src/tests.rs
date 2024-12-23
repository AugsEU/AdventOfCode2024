#[cfg(test)]
mod tests 
{
    use crate::part1;
    use crate::part2;

    const TEST_STR: &str = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part_1_test() 
    {
        let result = part1::compute_answer(TEST_STR);
        assert_eq!(result, 7);
    }

    #[test]
    fn part_2_test() 
    {
        let result = part2::compute_answer(TEST_STR);
        assert_eq!(result, "co,de,ka,ta");
    }
}