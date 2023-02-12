use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 4.5, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
