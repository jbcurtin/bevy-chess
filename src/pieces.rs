use bevy::prelude::*;

use crate::board::*;

const COORDINATE_OFFSET_X: f32 = 3.5;
const COORDINATE_OFFSET_Z: f32 = 3.5;
const WHITE_PIECE_COLOR: Color = Color::rgb(1.0, 0.8, 0.8);
const BLACK_PIECE_COLOR: Color = Color::rgb(0.0, 0.2, 0.2);
const PIECE_SCALE_X: f32 = 0.18;
const PIECE_SCALE_Y: f32 = 0.18;
const PIECE_SCALE_Z: f32 = 0.18;
const WHITE_PIECE_ROTATION_X: f32 = 1.65;
const WHITE_PIECE_ROTATION_Y: f32 = 1.35;
const BLACK_PIECE_ROTATION_X: f32 = 1.55;
const BLACK_PIECE_ROTATION_Y: f32 = 0.65;

pub fn create_white_pieces (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let king_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh0/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh1/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh2/Primitive0");
    let knight_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh3/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh4/Primitive0");
    let pawn_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh5/Primitive0");
    let white_material = materials.add(WHITE_PIECE_COLOR.into());
    
    // King's Side
    let king_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0., 0.0)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: king_handle.clone(),
            material: white_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(0.5, 0., 3.8));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(WHITE_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(WHITE_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(king_identity).insert(Piece { 
        team: Team::White,
        unit: Unit::King,
        tile: Tile::phrase_to_tile("D1".to_string()),
    });
    let king_bishop_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: bishop_handle.clone(),
            material: white_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(1.5, 0., 3.8));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(WHITE_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(WHITE_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(king_bishop_identity).insert(Piece {
        team: Team::White,
        unit: Unit::KingBishop,
        tile: Tile::phrase_to_tile("F1".to_string())
    });
    let king_knight_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: knight_handle.clone(),
            material: white_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(2.5, 0., 3.8));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(WHITE_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(WHITE_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(king_knight_identity).insert(Piece {
        team: Team::White,
        unit: Unit::KingBishop,
        tile: Tile::phrase_to_tile("G1".to_string())
    });
    let king_rook_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: rook_handle.clone(),
            material: white_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(3.5, 0., 3.8));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(WHITE_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(WHITE_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(king_rook_identity).insert(Piece {
        team: Team::White,
        unit: Unit::KingKnight,
        tile: Tile::phrase_to_tile("H1".to_string())
    });

    // Queen's Side
    let queen_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0. ,0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: queen_handle.clone(),
            material: white_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.5, 0., 3.8));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(WHITE_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(WHITE_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(queen_identity).insert(Piece {
        team: Team::White,
        unit: Unit::Queen,
        tile: Tile::phrase_to_tile("E1".to_string()),
    });
    let queen_bishop_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: bishop_handle.clone(),
            material: white_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-1.5, 0., 3.8));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(WHITE_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(WHITE_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(queen_bishop_identity).insert(Piece {
        team: Team::White,
        unit: Unit::QueenBishop,
        tile: Tile::phrase_to_tile("C1".to_string())
    });
    let queen_knight_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: knight_handle.clone(),
            material: white_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-2.5, 0., 3.8));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(WHITE_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(WHITE_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(queen_knight_identity).insert(Piece {
        team: Team::White,
        unit: Unit::QueenKnight,
        tile: Tile::phrase_to_tile("B1".to_string())
    });
    let queen_rook_identity  = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: rook_handle.clone(),
            material: white_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-3.5, 0., 3.8));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(WHITE_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(WHITE_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(queen_rook_identity).insert(Piece {
        team: Team::White,
        unit: Unit::QueenRook,
        tile: Tile::phrase_to_tile("A1".to_string())
    });

    let x_position_start = -3.5;
    for idx in 0..8 {
        let pawn_identity = commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: pawn_handle.clone(),
                material: white_material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(x_position_start + (idx as f32), 0., 2.8));
                    transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                    transform.rotate(Quat::from_rotation_x(WHITE_PIECE_ROTATION_X));
                    transform.rotate(Quat::from_rotation_y(WHITE_PIECE_ROTATION_Y));
                    transform
                },
                ..Default::default()
            });
        }).id();
        let column_name = COLUMNS[idx as usize];
        let phrase = format!("{}{}", column_name, '2');
        commands.entity(pawn_identity).insert(Piece {
            team: Team::White,
            unit: Unit::Pawn,
            tile: Tile::phrase_to_tile(phrase)
        });
    }
}

pub fn create_black_pieces (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let king_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh0/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh1/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh2/Primitive0");
    let knight_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh3/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh4/Primitive0");
    let pawn_handle: Handle<Mesh> = asset_server.load("models/all.glb#Mesh5/Primitive0");
    let black_material = materials.add(BLACK_PIECE_COLOR.into());

    // King's Side
    let king_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0., 0.0)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: king_handle.clone(),
            material: black_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(0.5, 0., -3.0));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(BLACK_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(BLACK_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(king_identity).insert(Piece {
        team: Team::Black,
        unit: Unit::King,
        tile: Tile::phrase_to_tile("E8".to_string())
    });
    let king_bishop_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: bishop_handle.clone(),
            material: black_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(1.5, 0., -3.0));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(BLACK_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(BLACK_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(king_bishop_identity).insert(Piece {
        team: Team::Black,
        unit: Unit::KingBishop,
        tile: Tile::phrase_to_tile("F8".to_string())
    });
    let king_knight_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: knight_handle.clone(),
            material: black_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(2.5, 0.,  -3.0));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(BLACK_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(BLACK_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(king_knight_identity).insert(Piece {
        team: Team::Black,
        unit: Unit::KingKnight,
        tile: Tile::phrase_to_tile("G8".to_string())
    });
    let king_rook_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: rook_handle.clone(),
            material: black_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(3.5, 0.,  -3.0));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(BLACK_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(BLACK_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(king_rook_identity).insert(Piece {
        team: Team::Black,
        unit: Unit::KingRook,
        tile: Tile::phrase_to_tile("H8".to_string())
    });

    // Queen's Side
    let queen_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0. ,0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: queen_handle.clone(),
            material: black_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.5, 0., -3.0));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(BLACK_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(BLACK_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(queen_identity).insert(Piece {
        team: Team::Black,
        unit: Unit::Queen,
        tile: Tile::phrase_to_tile("D8".to_string())
    });
    let queen_bishop_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: bishop_handle.clone(),
            material: black_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-1.5, 0.,  -3.0));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(BLACK_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(BLACK_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(queen_bishop_identity).insert(Piece {
        team: Team::Black,
        unit: Unit::QueenBishop,
        tile: Tile::phrase_to_tile("C8".to_string())
    });
    let queen_knight_identity = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: knight_handle.clone(),
            material: black_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-2.5, 0.,  -3.0));
                transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                transform.rotate(Quat::from_rotation_x(BLACK_PIECE_ROTATION_X));
                transform.rotate(Quat::from_rotation_y(BLACK_PIECE_ROTATION_Y));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(queen_knight_identity).insert(Piece {
        team: Team::Black,
        unit: Unit::QueenKnight,
        tile: Tile::phrase_to_tile("B8".to_string())
    });
    let queen_rook_identity  = commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: rook_handle.clone(),
            material: black_material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-3.5, 0.,  -3.0));
                transform.apply_non_uniform_scale(Vec3::new(0.18, 0.18, 0.18));
                transform.rotate(Quat::from_rotation_x(1.55));
                transform.rotate(Quat::from_rotation_y(1.35));
                transform
            },
            ..Default::default()
        });
    }).id();
    commands.entity(queen_rook_identity).insert(Piece {
        team: Team::Black,
        unit: Unit::QueenRook,
        tile: Tile::phrase_to_tile("A8".to_string())
    });
    let x_position_start = -3.5;
    for idx in 0..8 {
        let pawn_identity = commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: pawn_handle.clone(),
                material: black_material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(x_position_start + (idx as f32), 0.,  -2.0));
                    transform.apply_non_uniform_scale(Vec3::new(PIECE_SCALE_X, PIECE_SCALE_Y, PIECE_SCALE_Z));
                    transform.rotate(Quat::from_rotation_x(BLACK_PIECE_ROTATION_X));
                    transform.rotate(Quat::from_rotation_y(BLACK_PIECE_ROTATION_Y));
                    transform
                },
                ..Default::default()
            });
        }).id();
        let column_name = COLUMNS[idx as usize];
        let phrase = format!("{}{}", column_name, '7');
        commands.entity(pawn_identity).insert(Piece {
            team: Team::White,
            unit: Unit::Pawn,
            tile: Tile::phrase_to_tile(phrase)
        });
    }
}
