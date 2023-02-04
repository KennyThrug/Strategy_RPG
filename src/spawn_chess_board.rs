use bevy::prelude::*;
use core::f32::consts::PI;

pub fn spawn_chess_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
            commands
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
                .insert(Name::new("Board"));
        }
    }
}

pub fn create_game_piece(mut commands: Commands, server: Res<AssetServer>) {
    let chess_piece: Handle<Scene> = server.load("chess.glb#Scene0");
    commands
        .spawn(SceneBundle {
            scene: chess_piece,
            transform: Transform::from_xyz(0.0, 0.995, 0.6).with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..Default::default()
        })
        .insert(ChessPiece {
            x_cord: 0,
            y_cord: 0,
        })
        .insert(Name::new("Chess piece"));
}
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ChessSquare {
    x_cord: i16,
    y_cord: i16,
    normal_color: Color,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ChessPiece {
    x_cord: i16,
    y_cord: i16,
}
