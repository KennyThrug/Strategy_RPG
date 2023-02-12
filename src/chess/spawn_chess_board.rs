use bevy::prelude::*;
use core::f32::consts::PI;

pub fn spawn_chess_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let parent = commands
        .spawn(PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("BoardParent"))
        .insert(ParentChessBoard {})
        .id();
    for i in 0..8 {
        for j in 0..8 {
            let spawn_transform: Transform =
                Transform::from_xyz((i as f32 * 0.3) - 0.9, 0.7, (j as f32 * 0.3) - 0.9)
                    .with_rotation(Quat::from_rotation_y(-PI / 2.0));

            let spawn_color: Color = if (i + j) % 2 == 0 {
                Color::rgb(0.87, 0.44, 0.42)
            } else {
                Color::rgb(0.42, 0.44, 0.87)
            };
            let cur_block = commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
                    material: materials.add(spawn_color.into()),
                    transform: spawn_transform,
                    ..default()
                })
                .insert(ChessSquare {
                    x_cord: i,
                    y_cord: j,
                    normal_color: spawn_color,
                })
                .insert(Name::new("Board"))
                .id();
            commands.entity(parent).push_children(&[cur_block]);
        }
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct SquareColorChanges {
    x_cord: i16,
    y_cord: i16,
    color: Color,
}
#[derive(Component)]
pub struct ParentChessBoard {}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ChessSquare {
    pub x_cord: i16,
    pub y_cord: i16,
    pub normal_color: Color,
}
