use bevy::prelude::*;

use crate::board::*;

pub fn active_square_loop(
    mut commands: Commands,
    piece_query: Query<&Tile>,
) {
    
}

// if tile_query.is_empty() {
//     panic!("Tile Query is empty. Tiles must already exist.");
// }
// for tile in tile_query.iter() {
//     if tile.x == column_index && tile.y == row_index {
//         return *tile;
//     }
// }
// panic!("A tile should always exist for a piece on the board");