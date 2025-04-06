use crate::pieces::traits::Movable;
use crate::pieces::types::{Color, MoveOffsets};

#[derive(Clone, Copy)]
pub struct Bishop {
    color: Color,
}
const BISHOP_MOVES: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

impl Bishop {
    pub fn new(color: Color) -> Self {
        Bishop { color }
    }
}

impl Movable for Bishop {
    fn get_move_offsets(&self) -> MoveOffsets {
        MoveOffsets::new_appliable_multiple(BISHOP_MOVES.to_vec())
    }

    fn color(&self) -> &Color {
        &self.color
    }
}
