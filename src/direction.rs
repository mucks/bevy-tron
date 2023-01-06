use self::Direction::*;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

impl Default for Direction {
    fn default() -> Self {
        Forward
    }
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Left => Backward,
            Right => Forward,
            Forward => Left,
            Backward => Right,
        }
    }
    pub fn turn_right(&mut self) -> Self {
        match self {
            Left => Forward,
            Right => Backward,
            Forward => Right,
            Backward => Left,
        }
    }
}
