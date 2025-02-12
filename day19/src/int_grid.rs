pub struct IntGrid
{
    m_raw_chars: Vec<i32>,
    pub m_width: i32,
    pub m_height: i32,
}

impl IntGrid
{
    pub fn from(input : &String) -> Self 
    {
        let width = count_chars_before_newline(&input) as i32;
        let height = count_lines(&input) as i32;

        let san_string = strip_newlines(input.as_str());
        let chars: Vec<i32> = san_string.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

        let this = Self
        {
            m_width: width,
            m_height: height,
            m_raw_chars: chars
        };

        return this;
    }

    pub fn at_vec(&self, pos: (i32, i32)) -> Option<i32>
    {
        return self.at(pos.0, pos.1);
    }

    pub fn at(&self, x: i32, y: i32) -> Option<i32>
    {
        if !self.inside_grid(x, y)
        {
            return None;
        }

        let idx = x + y * self.m_width;
        return Some(self.m_raw_chars[idx as usize]);
    }

    pub fn set(&mut self, x: i32, y: i32, value: i32) -> Result<(), ()>
    {
        if !self.inside_grid(x, y)
        {
            return Err(());
        }

        let idx = x + y * self.m_width;
        self.m_raw_chars[idx as usize] = value;

        return Ok(());
    }

    pub fn inside_grid(&self, x: i32, y: i32) -> bool
    {
        return (0..self.m_width).contains(&x) && (0..self.m_height).contains(&y);
    }

    pub fn inside_grid_vec(&self, pos: (i32, i32)) -> bool
    {
        return self.inside_grid(pos.0, pos.1);
    }
}

fn count_chars_before_newline(text: &String) -> usize
{
    let mut count = 0;
    let mut char_it = text.chars().peekable();

    while let Some(c) = char_it.next()
    {
        count += 1;
        if c == '\r' || c == '\n'
        {
            break;
        }
    }

    return count - 1;
}

fn count_lines(text: &String) -> usize
{
    return text.lines().count();
}

fn strip_newlines(s: &str) -> String
{
    return s.replace('\n', "").replace('\r', "");
}

