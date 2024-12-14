use crate::robot_room::RobotRoom;

const SIM_SECONDS: i32 = 100;

pub fn get_robot_safety_factor(input: &String, width: i32, height: i32) -> i32
{
    let mut robot_room = RobotRoom::from(input.as_str(), width, height);
    robot_room.simulate_seconds(SIM_SECONDS);

    return robot_room.get_safety_factor();
}