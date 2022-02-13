#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Clockwise => Direction::CounterClockwise,
            Direction::CounterClockwise => Direction::Clockwise,
        }
    }
}
