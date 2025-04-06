pub const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub fn next(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Color::White, Color::White) => true,
            (Color::Black, Color::Black) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl std::ops::Add<(i32, i32)> for Position {
    type Output = Option<Position>;

    fn add(self, other: (i32, i32)) -> Option<Position> {
        let new_x = self.x as i32 + other.0;
        let new_y = self.y as i32 + other.1;

        if new_x < 0 || new_y < 0 {
            return None;
        }

        if new_x >= BOARD_SIZE as i32 || new_y >= BOARD_SIZE as i32 {
            return None;
        }

        Some(Position {
            x: new_x as usize,
            y: new_y as usize,
        })
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

enum MoveOffsets {
    AppliableOnce(Vec<(i32, i32)>),
    AppliableMultiple(Vec<(i32, i32)>),
}

impl MoveOffsets {
    fn new_appliable_once(offsets: Vec<(i32, i32)>) -> Self {
        MoveOffsets::AppliableOnce(offsets)
    }

    fn new_appliable_multiple(offsets: Vec<(i32, i32)>) -> Self {
        MoveOffsets::AppliableMultiple(offsets)
    }

    fn apply_offsets(&self, current_position: Position) -> Vec<Position> {
        match self {
            MoveOffsets::AppliableOnce(offsets) => offsets
                .iter()
                .filter_map(|offset| current_position + *offset)
                .collect(),
            MoveOffsets::AppliableMultiple(offsets) => {
                let mut positions = vec![];
                for offset in offsets {
                    let mut current_position = current_position;
                    loop {
                        match current_position + *offset {
                            Some(new_position) => {
                                positions.push(new_position);
                                current_position = new_position;
                            }
                            None => break,
                        }
                    }
                }
                positions
            }
        }
    }
}

pub trait Movable {
    fn get_move_offsets(&self) -> MoveOffsets;

    fn color(&self) -> &Color;

    fn get_valid_moves(&self, current_position: Position) -> Vec<Position> {
        let move_offsets = self.get_move_offsets();

        move_offsets.apply_offsets(current_position)
    }
}

#[derive(Clone, Copy)]
pub struct Pawn {
    color: Color,
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Pawn { color }
    }
}

impl Movable for Pawn {
    fn get_move_offsets(&self) -> MoveOffsets {
        let offsets = match self.color {
            Color::White => vec![(0, 1)],
            Color::Black => vec![(0, -1)],
        };
        MoveOffsets::new_appliable_once(offsets)
    }

    fn color(&self) -> &Color {
        &self.color
    }
}

#[derive(Clone, Copy)]
pub struct Knight {
    color: Color,
}
const KNIGHT_MOVE_OFFSETS: [(i32, i32); 8] = [
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
];

impl Knight {
    pub fn new(color: Color) -> Self {
        Knight { color }
    }
}

impl Movable for Knight {
    fn get_move_offsets(&self) -> MoveOffsets {
        MoveOffsets::new_appliable_once(KNIGHT_MOVE_OFFSETS.to_vec())
    }

    fn color(&self) -> &Color {
        &self.color
    }
}

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

#[derive(Clone, Copy)]
pub struct Rook {
    color: Color,
}
const ROOK_MOVES: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
impl Rook {
    pub fn new(color: Color) -> Self {
        Rook { color }
    }
}

impl Movable for Rook {
    fn get_move_offsets(&self) -> MoveOffsets {
        MoveOffsets::new_appliable_multiple(ROOK_MOVES.to_vec())
    }

    fn color(&self) -> &Color {
        &self.color
    }
}

#[derive(Clone, Copy)]
pub struct Queen {
    color: Color,
}
const QUEEN_MOVES: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl Queen {
    pub fn new(color: Color) -> Self {
        Queen { color }
    }
}

impl Movable for Queen {
    fn get_move_offsets(&self) -> MoveOffsets {
        MoveOffsets::new_appliable_multiple(QUEEN_MOVES.to_vec())
    }

    fn color(&self) -> &Color {
        &self.color
    }
}

#[derive(Clone, Copy)]
pub struct King {
    color: Color,
}
const KING_MOVES: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl King {
    pub fn new(color: Color) -> Self {
        King { color }
    }
}

impl Movable for King {
    fn get_move_offsets(&self) -> MoveOffsets {
        MoveOffsets::new_appliable_once(KING_MOVES.to_vec())
    }

    fn color(&self) -> &Color {
        &self.color
    }
}
