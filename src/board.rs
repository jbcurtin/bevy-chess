use bevy::prelude::*;
use bevy_mod_picking::*;
use crate::movement::*;

pub const COLUMNS: [char; 8] = ['H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'];
const ROWS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
const BORAD_MIN_ROW: u8 = 1;
const BORAD_MAX_ROW: u8 = 8;
const BOARD_MAX_COLUMN: char = 'H';
const BOARD_MIN_COLUMN: char = 'A';

const TILE_SIZE: f32 = 1.0;
const WHITE_TILE_COLOR: Color = Color::rgb(1.0, 0.9, 0.9);
const BLACK_TILE_COLOR: Color = Color::rgb(0.0, 0.1, 0.1);

#[derive(Component, Debug, Copy, Clone)]
pub struct Tile {
    pub x: u8,
    pub y: u8,
}

impl Tile {
    /// Break two char phrase into column and row indexes than return the corresponding rendered 3d tile.
    ///   It should corresponding with an iteractive mesh from bevy_mod_picking
    pub fn phrase_to_tile(phrase: String) -> Self {
        // https://stackoverflow.com/a/47829753/432309
        let indexable_both: Vec<char> = phrase.chars().collect();
        let column = indexable_both[0];
        let row: char = indexable_both[1];
        let column_index: u8 = COLUMNS.iter().position(|&predicate| predicate == column).unwrap() as u8;
        let row_index: u8 = ROWS.iter().position(|&predicate| predicate == row).unwrap() as u8;
        Tile {x: column_index, y: row_index}
    }
    /// Takes the x/y location relative to the board and returns the corresponding "CR" value
    pub fn phrase(&self) -> String {
        let column: char = COLUMNS[self.x as usize];
        let row: char = ROWS[self.y as usize];
        format!("{}{}", column, row)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Team {
    White,
    Black,
}
#[derive(Debug, Clone, Copy)]
pub enum Unit {
    King,
    KingBishop,
    KingKnight,
    KingRook,
    Queen,
    QueenBishop,
    QueenKnight,
    QueenRook,
    Pawn,
}
#[derive(Component, Debug, Clone, Copy)]
pub struct Piece {
    pub team: Team,
    pub unit: Unit,
    pub tile: Tile,
}
pub struct Movement {}
pub struct Selected {
    pub moves: Movement
}

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane {size: TILE_SIZE }));
    let white_material = materials.add(WHITE_TILE_COLOR.into());
    let black_material = materials.add(BLACK_TILE_COLOR.into());

    commands.spawn_bundle(PbrBundle {
        transform: {
            let mut transform = Transform::from_translation(Vec3::new(0., 0.,  0.));
            transform.rotate(Quat::from_rotation_y(3.140));
            transform  
        },
        ..Default::default()
    }).with_children(|parent| {
        // Add 64 Squares
        for i in 0..8 {
            for j in 0..8 {
                parent.spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j) % 2 == 0 {
                        white_material.clone()
                    } else {
                        black_material.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32 - 3.5, 0., j as f32 - 3.5 )),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(Tile {x: i, y: j });
            }
        }
    });
}
