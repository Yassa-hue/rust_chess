use crate::movables::{BOARD_SIZE, Bishop, Color, King, Knight, Movable, Pawn, Queen, Rook};
use std::array::from_fn;

pub struct Chessboard {
    chessboard: [[Option<Box<dyn Movable>>; BOARD_SIZE]; BOARD_SIZE],
}

impl Chessboard {
    pub fn new() -> Self {
        let mut chessboard: [[Option<Box<dyn Movable>>; BOARD_SIZE]; BOARD_SIZE] =
            from_fn(|_| from_fn(|_| None));

        chessboard[0][0] = Some(Box::new(Rook::new(Color::White)));
        chessboard[0][1] = Some(Box::new(Knight::new(Color::White)));
        chessboard[0][2] = Some(Box::new(Bishop::new(Color::White)));
        chessboard[0][3] = Some(Box::new(Queen::new(Color::White)));
        chessboard[0][4] = Some(Box::new(King::new(Color::White)));
        chessboard[0][5] = Some(Box::new(Bishop::new(Color::White)));
        chessboard[0][6] = Some(Box::new(Knight::new(Color::White)));
        chessboard[0][7] = Some(Box::new(Rook::new(Color::White)));
        chessboard[1][0] = Some(Box::new(Pawn::new(Color::White)));
        chessboard[1][1] = Some(Box::new(Pawn::new(Color::White)));
        chessboard[1][2] = Some(Box::new(Pawn::new(Color::White)));
        chessboard[1][3] = Some(Box::new(Pawn::new(Color::White)));
        chessboard[1][4] = Some(Box::new(Pawn::new(Color::White)));
        chessboard[1][5] = Some(Box::new(Pawn::new(Color::White)));
        chessboard[1][6] = Some(Box::new(Pawn::new(Color::White)));
        chessboard[1][7] = Some(Box::new(Pawn::new(Color::White)));
        chessboard[7][0] = Some(Box::new(Rook::new(Color::Black)));
        chessboard[7][1] = Some(Box::new(Knight::new(Color::Black)));
        chessboard[7][2] = Some(Box::new(Bishop::new(Color::Black)));
        chessboard[7][3] = Some(Box::new(Queen::new(Color::Black)));
        chessboard[7][4] = Some(Box::new(King::new(Color::Black)));
        chessboard[7][5] = Some(Box::new(Bishop::new(Color::Black)));
        chessboard[7][6] = Some(Box::new(Knight::new(Color::Black)));
        chessboard[7][7] = Some(Box::new(Rook::new(Color::Black)));
        chessboard[6][0] = Some(Box::new(Pawn::new(Color::Black)));
        chessboard[6][1] = Some(Box::new(Pawn::new(Color::Black)));
        chessboard[6][2] = Some(Box::new(Pawn::new(Color::Black)));
        chessboard[6][3] = Some(Box::new(Pawn::new(Color::Black)));
        chessboard[6][4] = Some(Box::new(Pawn::new(Color::Black)));
        chessboard[6][5] = Some(Box::new(Pawn::new(Color::Black)));
        chessboard[6][6] = Some(Box::new(Pawn::new(Color::Black)));
        chessboard[6][7] = Some(Box::new(Pawn::new(Color::Black)));
        Chessboard { chessboard }
    }
}
