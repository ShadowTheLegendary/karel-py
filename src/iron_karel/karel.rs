use crate::iron_karel::direction::Direction;

#[derive(Default, Copy, Clone)]
pub struct Karel {
    pub facing: Direction,
    pub position: (u32, u32),
}

impl Karel {
    pub const DEFAULT: Karel = Karel {
        facing: Direction::East,
        position: (0, 0),
    };
}
