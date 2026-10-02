use pixels_util::color::{Color, TRANSPARENT};

#[derive(Copy, Clone)]
pub struct WorldSquare {
    pub color: Color,
}

impl WorldSquare {
    pub const DEFAULT: WorldSquare = WorldSquare { color: TRANSPARENT };
}

#[derive(Clone)]
pub struct World {
    pub size: (u32, u32),
    pub squares: Vec<WorldSquare>,
}

impl World {
    pub fn get(&self, pos: (u32, u32)) -> WorldSquare {
        self.squares[self.coordinate_to_idx(pos.0, pos.1)]
    }

    pub fn set(&mut self, pos: (u32, u32), square: WorldSquare) {
        let pos = self.coordinate_to_idx(pos.0, pos.1);
        self.squares[pos] = square;
    }

    fn coordinate_to_idx(&self, x: u32, y: u32) -> usize {
        y as usize * self.size.0 as usize + x as usize
    }
}
