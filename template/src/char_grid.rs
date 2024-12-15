use glam::IVec2;

pub struct CharGrid
{
    m_raw_chars: Vec<char>,
    pub m_width: i32,
    pub m_height: i32,
}

impl CharGrid
{
    pub fn from(input : &str) -> Self 
    {
        let width = count_chars_before_newline(&input) as i32;
        let height = count_lines(&input) as i32;

        let san_string = strip_newlines(&input);
        let chars: Vec<char> = san_string.chars().collect();

        let this = Self
        {
            m_width: width,
            m_height: height,
            m_raw_chars: chars
        };

        return this;
    }

    pub fn at_vec(&self, pos: IVec2) -> Option<char>
    {
        return self.at(pos.x, pos.y);
    }

    pub fn at(&self, x: i32, y: i32) -> Option<char>
    {
        if !self.inside_grid(x, y)
        {
            return None;
        }

        let idx = x + y * self.m_width;
        return Some(self.m_raw_chars[idx as usize]);
    }

    pub fn set_v(&mut self, pos: IVec2, value: char) -> Result<(), ()>
    {
        return self.set(pos.x, pos.y, value);
    }

    pub fn set(&mut self, x: i32, y: i32, value: char) -> Result<(), ()>
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

    pub fn find_first(&self, item: char) -> Option<IVec2>
    {
        for x in 0..self.m_width
        {
            for y in 0..self.m_height
            {
                let char = self.at(x, y).unwrap();
                if char == item
                {
                    return Some(IVec2::new(x, y));
                }
            }
        }
        
        return None;
    }

    pub fn to_string(&self) -> String
    {
        let mut chars: Vec<char> = Vec::new();

        for y in 0..self.m_height
        {
            for x in 0..self.m_width
            {
                chars.push(self.at(x, y).unwrap());
            }

            chars.push('\n');
        }

        return chars.into_iter().collect();
    }
}

fn count_chars_before_newline(text: &str) -> usize
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

fn count_lines(text: &str) -> usize
{
    return text.lines().count();
}

fn strip_newlines(s: &str) -> String
{
    return s.replace('\n', "").replace('\r', "");
}

