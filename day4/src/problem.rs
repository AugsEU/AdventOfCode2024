use strum::{EnumIter, IntoEnumIterator};

const XMAS_STRING : &str = "XMAS";
const MAS_STRING : &str = "MAS";

struct CrossWord
{
    m_width : i32,
    m_height : i32,
    m_data : Vec<char>
}

#[derive(EnumIter)]
enum SearchDir
{
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

impl CrossWord
{
    fn from(input : &String) -> Self 
    {
        let width = count_chars_before_newline(&input) as i32;
        let height = count_lines(&input) as i32;

        let san_string = strip_newlines(input.as_str());
        let chars: Vec<char> = san_string.chars().collect();

        Self
        {
            m_width: width,
            m_height: height,
            m_data: chars
        }
    }

    fn at(&self, x : i32, y : i32) -> char
    {
        if x < 0 || x >= self.m_width 
        {
            return ' ';
        }

        if y < 0 || y >= self.m_height 
        {
            return ' ';
        }

        let index : i32 = x +  y * self.m_width;

        if !(index >= 0 && index < self.m_data.len() as i32)
        {
            println!("Idx: {index}, Len: {}", self.m_data.len());
            println!("W: {}, H: {}", self.m_width, self.m_height);
        }
        assert!(index >= 0 && index < self.m_data.len() as i32);
        return self.m_data[index as usize];
    }

    fn is_word(&self, x: i32, y: i32, dir: SearchDir, word: &str) -> bool
    {
        let mut search_i: usize = 0;
        let dir = dir_to_tuple(dir);
        let mut point: (i32, i32) = (x, y);

        while search_i < word.len()
        {
            let char_at = self.at(point.0, point.1);
            let char_to_check = word.chars().nth(search_i).unwrap();

            if char_at != char_to_check
            {
                return false;
            }

            point = (point.0 + dir.0, point.1 + dir.1);

            search_i += 1;
        }

        return true;
    }

    fn is_cross(&self, x: i32, y: i32, word: &str) -> bool
    {
        // Check \ diag
        if self.is_word(x, y, SearchDir::SE, word) || self.is_word(x+2, y+2, SearchDir::NW, word)
        {
            // Check / diag
            if self.is_word(x+2, y, SearchDir::SW, word) || self.is_word(x, y+2, SearchDir::NE, word)
            {
                return true;
            }
        }

        return false;
    }
}

// Problem
pub fn count_xmas(input : &String) -> i32
{
    let cw: CrossWord = CrossWord::from(input);
    let mut total_matches = 0;

    for x in 0..cw.m_width
    {
        for y in 0..cw.m_height
        {
            for dir in SearchDir::iter()
            {
                if cw.is_word(x, y, dir, XMAS_STRING)
                {
                    total_matches += 1;
                }
            }
        }
    }

    return total_matches;
}

pub fn count_mas_crosses(input : &String) -> i32
{
    let cw: CrossWord = CrossWord::from(input);
    let mut total_matches = 0;

    for x in 0..cw.m_width
    {
        for y in 0..cw.m_height
        {
            if cw.is_cross(x, y, MAS_STRING)
            {
                total_matches += 1;
            }
        }
    }

    return total_matches;
}


// Utility (v slow)
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

fn dir_to_tuple(dir: SearchDir) -> (i32, i32)
{
    match dir
    {
        SearchDir::N =>
        {
            return (0, -1);
        }
        SearchDir::NE =>
        {
            return (1, -1);
        }
        SearchDir::E =>
        {
            return (1, 0);
        }
        SearchDir::SE =>
        {
            return (1, 1);
        }
        SearchDir::S =>
        {
            return (0, 1);
        }
        SearchDir::SW =>
        {
            return (-1, 1);
        }
        SearchDir::W =>
        {
            return (-1, 0);
        }
        SearchDir::NW =>
        {
            return (-1, -1);
        }
    };
}