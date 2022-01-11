use bevy::prelude::*;

use crate::board::*;

pub fn active_square_loop(
    commands: Commands,
    query: Query<&ActiveSquare>,
) {
    println!("Awesome Active Square.")
}