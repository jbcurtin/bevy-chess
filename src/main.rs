use bevy::prelude::*;
use bevy_mod_picking::*;

mod pieces;
use pieces::*;
mod board;
use board::*;
mod controls;
use controls::*;
mod game;
use game::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4})
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_startup_system(setup)
        .add_startup_system(create_board)
        .add_startup_system(create_white_pieces)
        .add_startup_system(create_black_pieces)
        .add_system(active_square_loop)
        .add_system_to_stage(CoreStage::PostUpdate , handle_piece_and_tile_events)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 5.0, 8.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert_bundle(PickingCameraBundle::default());
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(30.0, 30.0, 30.0)),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..Default::default()
        },
        ..Default::default()
    });
}
