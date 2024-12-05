use std::collections::HashMap;
use regex::Regex;

const MAX_ITER_ATTEMPTS : usize = 100000;

// Update
#[derive(Debug, Clone)]
struct Update
{
    m_pages : Vec<i32>,
    m_page_to_index : HashMap<i32, usize>
}

impl Update
{
    fn from(line: &str) -> Result<Update,()>
    {
        let page_numbers = line.split(',').map(|l| l.parse::<i32>()).collect();
        let page_numbers : Vec<i32> = match page_numbers
        {
            Ok(numbers) =>
            {
                numbers
            }
            Err(_) =>
            {
                return Err(());
            }
        };

        let mut page_to_index: HashMap<i32, usize> = HashMap::new();

        for i in 0..page_numbers.len()
        {
            page_to_index.insert(page_numbers[i], i);
        }

        let result : Update = Update
        {
            m_pages: page_numbers,
            m_page_to_index: page_to_index
        };

        return Ok(result);
    }

    fn get_middle_page_num(&self) -> i32
    {
        assert!(self.m_pages.len() % 2 == 1, "Even number of pages, where is the middle?!");
        let idx = self.m_pages.len() / 2;
        return self.m_pages[idx];
    }

    fn swap_pages(&mut self, page1 : i32, page2: i32)
    {
        let page1_idx = self.m_page_to_index.get(&page1).expect("Page not in rule?!").clone();
        let page2_idx = self.m_page_to_index.get(&page2).expect("Page not in rule?!").clone();

        self.m_pages.swap(page1_idx, page2_idx);

        self.m_page_to_index.insert(page1, page2_idx);
        self.m_page_to_index.insert(page2, page1_idx);
    }

    fn fix(&mut self, rules: &Vec<Rule>)
    {
        let mut cont : bool = true;
        let mut attempt_count : usize = 0;

        // Prayge this stops eventually....
        while cont && attempt_count < 10000
        {
            cont = false;
            for rule in rules
            {
                if rule.breaks_rule(self)
                {
                    cont = true;
                    self.swap_pages(rule.m_before_page, rule.m_page);
                }
            }

            attempt_count += 1;
        }

        if attempt_count > MAX_ITER_ATTEMPTS
        {
            dbg!(self);
            panic!("Couldn't solve this update!");
        }
    }
}

// Rule
#[derive(Debug, Clone)]
struct Rule
{
    m_before_page : i32,
    m_page : i32
}

impl Rule
{
    fn from(line: &str) -> Result<Rule,()>
    {
        let find_numbers = Regex::new(r"(\d+)\|(\d+)").unwrap();
        let mut captures_iter = find_numbers.captures_iter(line);

        if let Some(captures) = captures_iter.next()
        {
            // Access the first and second capture groups
            let first_number = captures.get(1).map(|m| m.as_str()).ok_or(())?;
            let second_number = captures.get(2).map(|m| m.as_str()).ok_or(())?;

            let first_number = first_number.parse::<i32>().map_err(|_| ())?;
            let second_number = second_number.parse::<i32>().map_err(|_| ())?;

            let result : Rule = Rule
            {
                m_before_page: first_number,
                m_page: second_number
            };

            return Ok(result);
        }
        else
        {
            println!("No match found");
        }

        return Err(());
    }

    fn breaks_rule(&self, update: &Update) -> bool
    {
        let idx_first = update.m_page_to_index.get(&self.m_before_page);
        let idx_after = update.m_page_to_index.get(&self.m_page);

        if let Some(idx_first) = idx_first
        {
            if let Some(idx_after) = idx_after
            {
                // Both indices are in the rule, so now we enforce it.
                return idx_first >= idx_after;
            }
        }

        // Not both the indices are in the rule, so it's not broken.
        return false;
    }
}


// Problem
pub fn count_middle_pages_of_updates(input: &String) -> i32
{
    let (rules, updates) = parse_input(input);

    let mut middle_page_count = 0;
    for update in updates.iter()
    {
        if rules.iter().any(|rule| rule.breaks_rule(&update)) == false
        {
            middle_page_count += update.get_middle_page_num();
        }
    }

    return middle_page_count;
}


pub fn count_middle_pages_of_fixuped_updates(input: &String) -> i32
{
    let (rules, updates) = parse_input(input);

    // Filter out working updates.
    let mut broken_updates : Vec<Update> = updates.iter()
                                    .filter(|u| rules.iter().any(|rule| rule.breaks_rule(u)))
                                    .cloned()  // This clones the `Update` value
                                    .collect();

    // Fix broken updates.
    for update in broken_updates.iter_mut()
    {
        update.fix(&rules);
    }

    let mut middle_page_count = 0;
    for update in broken_updates.iter()
    {
        middle_page_count += update.get_middle_page_num();
    }

    return middle_page_count;
}


// Parse
fn parse_input(input: &String) -> (Vec<Rule>, Vec<Update>)
{
    let mut rules : Vec<Rule> = Vec::new();
    let mut updates : Vec<Update> = Vec::new();

    for line in input.lines()
    {
        if line.len() == 0
        {
            continue;
        }

        if let Ok(update) = Update::from(line)
        {
            updates.push(update);
        }
        else if let Ok(rule) = Rule::from(line)
        {
            rules.push(rule);
        }
        else
        {
            panic!("Non-empty line cannot be parsed");     
        }
    }

    return (rules, updates);
}