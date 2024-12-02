use std::fs;

struct ListPair
{
    list1 : Vec<i32>,
    list2 : Vec<i32>
}

impl ListPair
{
    fn new() -> Self 
    {
        Self 
        {
            list1: Vec::new(),
            list2: Vec::new(),
        }
    }
}

fn read_all_lines(path : String) -> String
{
    let contents : String = fs::read_to_string(path).expect("Should have been able to read the file");
    return contents;
}


fn read_list_pairs() -> ListPair
{
    let file_path : String = String::from("./input.txt");
    let file_contents = read_all_lines(file_path);
    let lines : Vec<String> = file_contents.lines().map(|line| line.to_string()).collect();

    let lines_split : Vec<(i32, i32)> = lines.iter().map(|line| parse_to_tuple(&line)).collect();

    let mut result : ListPair = ListPair::new();

    for line_split in lines_split.iter()
    {
        result.list1.push(line_split.0);
        result.list2.push(line_split.1);
    }

    return result;
}

fn parse_to_tuple(input: &str) -> (i32, i32) 
{
    // Split the string by whitespace
    let parts: Vec<&str> = input.split_whitespace().collect();

    // Try to parse the two parts as i32
    let first = parts[0].parse::<i32>();
    let second = parts[1].parse::<i32>().expect("");

    return (first, second);
}

fn find_total_dist(lists : &mut ListPair) -> i32
{
    lists.list1.sort();
    lists.list2.sort();

    let mut total_dist : i32 = 0;

    for i in 0..lists.list1.len()
    {
        total_dist += (lists.list1[i] - lists.list2[i]).abs();
    }
    
    return total_dist;
}

fn get_similarity_dist(lists : &mut ListPair) -> i32
{
    lists.list1.sort();
    lists.list2.sort();

    let mut total_sim : i32 = 0;

    for i in 0..lists.list1.len()
    {
        let val = lists.list1[i];
        let num_occ : i32 = (lists.list2.iter().filter(|&n| *n == val).count()) as i32;

        if i < 5
        {
            println!("Val {val} appears {num_occ} times.");
        }

        total_sim += val * num_occ;
    }
    
    return total_sim;
}

fn main()
{
    let mut lists : ListPair = read_list_pairs();

    let total_dist = find_total_dist(&mut lists);
    let total_sim = get_similarity_dist(&mut lists);

    println!("Total dist: {total_dist}");
    println!("Total sim: {total_sim}");
}

