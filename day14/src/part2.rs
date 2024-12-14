use std::io::{self, Write};
use crate::robot_room::RobotRoom;

pub fn step_until_tree(input: &String, width: i32, height: i32) -> i32
{
    let mut robot_room = RobotRoom::from(input.as_str(), width, height);

    let mut step_num = 0;
    loop
    {
        let room_str = robot_room.to_string();

        if robot_room.get_likely_points().len() > 2 // Filter out cases that aren't likely to contain a tree
        {
            println!("Step {step_num}:");
            println!("{}", room_str);
    
            // Ask human for approval.
            let is_tree = get_input();
            if is_tree
            {
                break;
            }
        }

        // Move one second.
        robot_room.simulate_seconds(1);    
        step_num += 1;
    }

    return step_num;
}

pub fn get_input() -> bool
{
    print!("Please enter y/n: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Trim and convert input to lowercase for easy matching
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "y" =>
        {
            return true;
        }
        "n" =>
        {
            return false;
        }
        _ =>
        {
        
        }
    }

    return false;
}