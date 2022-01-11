use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::board::*;
use crate::movement::*;

pub fn handle_piece_and_tile_events(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    piece_query: Query<(Entity, &Piece)>,
    piece_movement_query: Query<(Entity, &PieceMovement)>,
    tile_query: Query<&Tile>,
) {
    for event in events.iter() {
        match &event {
            PickingEvent::Clicked(entity) => {}
            PickingEvent::Selection(selection) => {
                match &selection {
                    SelectionEvent::JustSelected(entity) => {
                        let tile = tile_query.get(*entity);
                        match &tile {
                            Ok(&tile) => {
                                for (piece_entity, piece) in piece_query.iter() {
                                    if piece.tile.phrase() == tile.phrase() {
                                        let piece_movement = PieceMovement::from_tile_and_unit(piece.tile, piece.unit);
                                        commands.entity(piece_entity).insert(piece_movement);
                                        println!("Added Movement");
                                    }
                                }
                            },
                            Err(error) => {
                                panic!("There should always be a tile")
                            }
                        }
                    }
                    SelectionEvent::JustDeselected(entity) => {
                        let tile = tile_query.get(*entity);
                        match &tile {
                            Ok(&tile) => {
                                for (piece_entity, movement) in piece_movement_query.iter() {
                                    if movement.location.phrase() == tile.phrase() {
                                        commands.entity(piece_entity).remove::<PieceMovement>();
                                    };
                                }
                            }
                            Err(error) => {
                                panic!("There should always be a tile")
                            }
                        }
                        
                    }
                }
            }
            PickingEvent::Hover(hover) => {}
        }
    }
}