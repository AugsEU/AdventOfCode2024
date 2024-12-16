use glam::IVec2;
use strum::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
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

    pub fn add_to(&self, pos: IVec2) -> IVec2
    {
        match self
        {
            Direction::North => IVec2::new(pos.x, pos.y - 1),
            Direction::East => IVec2::new(pos.x + 1, pos.y),
            Direction::South => IVec2::new(pos.x, pos.y + 1),
            Direction::West => IVec2::new(pos.x - 1, pos.y)
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

    pub fn rot_left(&self) -> Direction
    {
        match self
        {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South
        }
    }
}