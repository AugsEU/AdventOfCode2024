use strum::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction
{
    North,
    East,
    South,
    West
}

impl Direction
{
    pub fn from(chararacter : char) -> Option<Self>
    {
        match chararacter
        {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            '<' => Some(Direction::West),
            'v'|'V' => Some(Direction::South),
            _ => None
        }
    }

    pub fn add_to(&self, pos: (i32, i32)) -> (i32, i32)
    {
        match self
        {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::East => (pos.0 + 1, pos.1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::West => (pos.0 - 1, pos.1)
        }
    }

    pub fn invert(&self) -> Direction
    {
        match self
        {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East
        }
    }

    pub fn rot_right(&self) -> Direction
    {
        match self
        {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }
}