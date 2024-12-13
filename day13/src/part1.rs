use crate::prize_machine::PrizeMachine;

pub fn get_min_token_cost(input: &String) -> i64
{
    let machines = PrizeMachine::parse_list(input);

    let mut total_cost = 0;
    for machine in machines.iter()
    {
        let cost = machine.get_minimum_cost();

        if let Some(cost) = cost
        {
            total_cost += cost;
        }
    }

    return total_cost;
}