use bevy::prelude::*;
use bevy_rmesh::RMeshPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RMeshPlugin)
        .add_startup_system(setup)
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
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}