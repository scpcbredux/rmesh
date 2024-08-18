use bevy::prelude::*;
use bevy_rmesh::RMeshPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RMeshPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: asset_server.load("cube.rmesh#Mesh0"),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
