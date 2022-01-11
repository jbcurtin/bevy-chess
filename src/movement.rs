use bevy::prelude::*;
use crate::board::*;

#[derive(Component, Clone, Debug, Copy)]
pub enum Direction {
    Diagonal,
    Line,
    LShape,
    ForwardOnly,
    Poly,
}
#[derive(Component, Clone, Debug, Copy)]
pub struct Movement {
    length: u8,
    direction: Direction,
    attack: Direction,
}
#[derive(Component, Clone, Debug, Copy)]
pub struct PieceMovement{
    pub location: Tile,
    movement: Movement,
}
impl PieceMovement {
    pub fn from_tile_and_unit(tile: Tile, unit: Unit) -> PieceMovement {
        match &unit {
            // Kings are capable of moving in any direction, one space.
            Unit::King => {
                PieceMovement {
                    location: tile.clone(),
                    movement: Movement { 
                        length: 1,
                        direction: Direction::Poly,
                        attack: Direction::Poly,
                     }
                }
                
            }
            // King Bishops are capable of moving diagonally in any direction, as far as possible
            Unit::KingBishop => {
                PieceMovement {
                    location: tile.clone(),
                    movement: Movement { 
                        length: 8,
                        direction: Direction::Diagonal,
                        attack: Direction::Diagonal,
                     }
                }
            }
            // King knights are capable of moving two spaces and adjacent one space in any direction.
            Unit::KingKnight => {
                PieceMovement {
                    location: tile.clone(),
                    movement: Movement {
                        length: 3,
                        direction: Direction::LShape,
                        attack: Direction::LShape,
                    }
                }
            }
            // King rooks are capable of moving horizontally or vertically across the whole board.
            Unit::KingRook => {
                PieceMovement {
                    location: tile.clone(),
                    movement: Movement {
                        length: 8,
                        direction: Direction::Line,
                        attack: Direction::Line,
                    }
                }
            }
            // Queen's can move to any space on the board, assuming its not blocked by an other piece.
            Unit::Queen => {
                PieceMovement {
                    location: tile.clone(),
                    movement: Movement {
                        length: 8,
                        direction: Direction::Poly,
                        attack: Direction::Poly,
                    }
                }
            }
            Unit::QueenBishop => {
                PieceMovement {
                    location: tile.clone(),
                    movement: Movement {
                        length: 8,
                        direction: Direction::Diagonal,
                        attack: Direction::Diagonal,
                    }
                }
            }
            Unit::QueenKnight => {
                PieceMovement {
                    location: tile.clone(),
                    movement: Movement {
                        length: 3,
                        direction: Direction::LShape,
                        attack: Direction::LShape,
                    }
                }
            }
            Unit::QueenRook => {
                PieceMovement {
                    location: tile.clone(),
                    movement: Movement {
                        length: 8,
                        direction: Direction::Line,
                        attack: Direction::Line,
                    }
                }
            }
            Unit::Pawn => {
                PieceMovement {
                    location: tile.clone(),
                    movement: Movement {
                        length: 2,
                        direction: Direction::ForwardOnly,
                        attack: Direction::Diagonal,
                    }
                }
            }
        }
    }
}