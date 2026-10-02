use crate::iron_karel::WORLD;

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    North,
    #[default]
    East,
    South,
    West,
}

impl Direction {
    pub fn shl(&mut self) {
        use Direction::*;
        *self = match self {
            North => West,
            West => South,
            South => East,
            East => North,
        };
    }

    pub fn shr(&mut self) {
        use Direction::*;
        *self = match self {
            North => East,
            East => South,
            South => West,
            West => North,
        };
    }

    pub fn flp(&mut self) {
        use Direction::*;
        *self = match self {
            North => South,
            East => West,
            South => North,
            West => East,
        };
    }

    pub fn mve(self, mut position: (u32, u32)) -> Option<(u32, u32)> {
        use Direction::*;
        match self {
            North => {
                let ny = position.1.saturating_add(1);
                if ny == position.1 || ny >= WORLD.lock().unwrap().size.1 {
                    // if we didn't move (world height is u32::MAX and so is player y position) or player moved out of the world
                    return None;
                }
                position.1 = ny;
            }
            East => {
                let nx = position.0.saturating_add(1);
                if nx == position.0 || nx >= WORLD.lock().unwrap().size.0 {
                    // if we didn't move (world width is u32::MAX and so is player x position) or player moved out of the world
                    return None;
                }
                position.0 = nx;
            }
            South => {
                let ny = position.1.saturating_sub(1);
                if ny == position.1 {
                    // if we didn't move (tried to move down into floor)
                    return None;
                }
                position.1 = ny;
            }
            West => {
                let nx = position.0.saturating_sub(1);
                if nx == position.0 {
                    // if we didn't move (tried to move into left wall)
                    return None;
                }
                position.0 = nx;
            }
        }

        Some(position)
    }
}
