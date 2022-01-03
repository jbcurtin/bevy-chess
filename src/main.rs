use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4})
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600.0,
            height: 1600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(create_board)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 8.0 })),
        material: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0., 0.0 )),
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
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

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane {size: 1.0 }));
    let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0.0, 0.1, 0.1).into());

    // Add 64 Squares
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..Default::default()
            });
        }
    }
}