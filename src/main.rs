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

// commands.spawn_bundle(PerspectiveCameraBundle {
//     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//     ..Default::default()
// });
// commands.spawn_bundle(OrthographicCameraBundle {
//     transform: Transform::from_translation(Vec3::new(0. , 0., 8.))
//         .looking_at(Vec3::default(), Vec3::Y),
//     orthographic_projection: OrthographicProjection {
//         scale: 0.01,
//         ..Default::default()
//     },
//     ..OrthographicCameraBundle::new_3d()
// });
// commands.spawn_bundle(PbrBundle {
//     mesh: meshes.add(Mesh::from(shape::Icosphere {
//         radius: 0.45,
//         subdivisions: 32,
//     })),
//     material: materials.add(StandardMaterial {
//         base_color: Color::hex("ffd891").unwrap(),
//         unlit: true,
//         ..Default::default()
//     }),
//     transform: Transform::from_xyz(-5.0, -2.5, 0.0),
//     ..Default::default()
// });
