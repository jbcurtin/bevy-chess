use bevy::prelude::*;

use crate::movement::*;
use crate::board::*;

pub fn handle_piece_movement_logic(
    mut commands: Commands,
    piece_movement_query: Query<(Entity, &PieceMovement)>,
    tile_query: Query<(Entity, &Tile)>,
) {
    
}