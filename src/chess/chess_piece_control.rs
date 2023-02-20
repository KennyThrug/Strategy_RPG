use bevy::prelude::*;

use super::spawn_chess_board::{self, *};

pub fn create_all_pieces(mut commands: Commands, server: Res<AssetServer>) {
    //    let chess_piece: Handle<Scene> = server.load("Pawn.glb#Scene0");
    let rows: [i16; 4] = [0, 1, 6, 7];
    for i in rows {
        for j in 0..8 {
            //Create the pieces
            commands
                .spawn(SceneBundle {
                    scene: server.load(get_chess_scene(j, i)),
                    transform: get_transform_from_xy(j, i, true)
                        .with_scale(Vec3::new(0.1, 0.1, 0.1)),
                    ..Default::default()
                })
                .insert(ChessPiece {
                    x_cord: j,
                    y_cord: i,
                })
                .insert(Name::new("Chess piece"));
        }
    }
}

const GRAVITYSPEED: f32 = 0.85;
pub fn create_selection() {
    //TODO make a function that highlights different squares
}

pub fn handle_gravity(time: Res<Time>, mut query: Query<&mut Transform, With<ChessPiece>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.y >= 0.995 {
            if transform.translation.y - (GRAVITYSPEED * time.delta_seconds()) >= 0.995 {
                transform.translation -= Vec3::new(0.0, GRAVITYSPEED * time.delta_seconds(), 0.0);
            } else {
                let cur_y: f32 = transform.translation.y;
                transform.translation -= Vec3::new(0.0, cur_y - 0.995, 0.0);
            }
        }
    }
}

fn get_startup_height(x_cord: i16, y_cord: i16) -> f32 {
    let math_y = if y_cord < 3 { y_cord } else { y_cord - 6 };
    return (math_y as f32 * 0.4) + (x_cord as f32 * 0.05);
}

//x: -0.9 and y: 1.2 is 0x0
/**
 * x_cord : position on the board, not in transform space
 * y_cord : same as x_cord, but... well y position
 * starting: True if starting position for game, false if called during the game
 */
fn get_transform_from_xy(x_cord: i16, y_cord: i16, starting: bool) -> Transform {
    return Transform::from_xyz(
        (-0.9) + ((x_cord as f32) * 0.3),
        0.995
            + if starting {
                get_startup_height(x_cord, y_cord)
            } else {
                0.0
            },
        (1.2) - ((y_cord as f32) * 0.3),
    );
}

fn get_chess_scene<'a>(x_cord: i16, y_cord: i16) -> &'a str {
    return if y_cord == 1 || y_cord == 6 {
        "Pawn.glb#Scene0"
    } else if x_cord == 0 || x_cord == 7 {
        "Rook.glb#Scene0"
    } else {
        "Bishop.glb#Scene0"
    };
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct ChessPiece {
    x_cord: i16,
    y_cord: i16,
}
