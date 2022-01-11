use bevy::prelude::*;

use crate::movement::*;

pub fn handle_piece_movement_logic(
    mut commands: Commands,
    piece_movement_query: Query<(Entity, &PieceMovement)>,
) {
    for (movement_entity, piece_movement) in piece_movement_query.iter() {
        println!("Move Piece: {:?}", movement_entity);
    }   
}