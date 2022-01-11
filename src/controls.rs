use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::board::*;

pub fn handle_events(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
) {
    for event in events.iter() {
        match &event {
            PickingEvent::Clicked(entity) => {
                // commands.entity(*selected_entity).insert(TileActive { selected: true })
            }
            PickingEvent::Selection(selection_event) => {
                match &selection_event {
                    SelectionEvent::JustSelected(entity) => {
                        println!("Selected: {:?}", entity);
                        // commands.entity(*entity).insert(TileActive {selected: true });
                    }
                    SelectionEvent::JustDeselected(entity) => {
                        println!("DeSelceted: {:?}", entity );
                    }
                }
            },
            PickingEvent::Hover(hover_event) => {
                match &hover_event {
                    HoverEvent::JustEntered(entity) => {

                    }
                    HoverEvent::JustLeft(entity) => {

                    }
                }
            },
        }
    }
}

pub fn handle_piece_and_tile_events(
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    piece_query: Query<&Piece>,
    tile_query: Query<&Tile>,
) {
    for event in events.iter() {
        match &event {
            PickingEvent::Clicked(entity) => {}
            PickingEvent::Selection(selection) => {
                match &selection {
                    SelectionEvent::JustSelected(entity) => {
                        println!("Awesome Selection")
                    }
                    SelectionEvent::JustDeselected(entity) => {
                        // println!("Awesome deselection");
                        // println!("Piece: {:?}", piece_query.is_empty());
                        // println!("Tile: {:?}", tile_query.is_empty());
                        // let piece = piece_query.get(entity);
                        let tile = tile_query.get(*entity);
                        match &tile {
                            Ok(&tile) => {
                                println!("Tile: {:?}", tile.phrase());
                                for piece in piece_query.iter() {
                                    if piece.tile.phrase() == tile.phrase() {
                                        println!("Piece exists: {:?}", piece);
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