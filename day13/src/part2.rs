use crate::prize_machine::PrizeMachine;
use glam::I64Vec2;

pub fn get_min_token_cost(input: &String) -> i64
{
    const PRIZE_ADD : i64 = 10000000000000;

    let mut machines = PrizeMachine::parse_list(input);
    for machine in machines.iter_mut()
    {
        machine.add_to_prize(I64Vec2::new(PRIZE_ADD, PRIZE_ADD));
    }

    let mut total_cost = 0;
    for machine in machines.iter_mut()
    {
        let cost = machine.get_minimum_cost();

        if let Some(cost) = cost
        {
            total_cost += cost;
        }
    }

    return total_cost;
}