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
                    transform: get_transform_from_xy(j, i).with_scale(Vec3::new(0.1, 0.1, 0.1)),
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

pub fn change_chess_board_color(
    x_cord: i16,
    y_cord: i16,
    color: Color,
    commands: &mut Commands,
    target: &mut Query<(&ChessSquare, Entity), With<Handle<StandardMaterial>>>,
    parent: &Query<Entity, With<spawn_chess_board::ParentChessBoard>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let spawn_transform = get_transform_from_xy(x_cord, y_cord);
    for en in parent.iter() {
        for (square, entity) in target.iter_mut() {
            if square.x_cord == x_cord && square.y_cord == y_cord {
                println!("Test");
                commands.entity(entity).despawn();
                let cur_block = commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
                        material: materials.add(color.into()),
                        transform: spawn_transform,
                        ..default()
                    })
                    .insert(ChessSquare {
                        x_cord: 0,
                        y_cord: 0,
                        normal_color: color,
                    })
                    .insert(Name::new("Board"))
                    .id();
                commands.entity(en).push_children(&[cur_block]);
            }
        }
        //mat.material = materials.add(Color::rgb(0.0, 1.0, 0.0).into());
        //material = materials.add(Color::rgb(0.0, 1.0, 0.0).into());
        //material.basecolor =
        //transform.translation += Vec3::new(0.0, 0.1, 0.0);
    }
}

pub fn change_middle_square(
    mut commands: Commands,
    mut target: Query<(&ChessSquare, Entity), With<Handle<StandardMaterial>>>,
    parent: Query<Entity, With<spawn_chess_board::ParentChessBoard>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    change_chess_board_color(
        6,
        6,
        Color::rgb(0.0, 1.0, 0.0),
        &mut commands,
        &mut target,
        &parent,
        &mut materials,
        &mut meshes,
    );
}

//x: -0.9 and y: 1.2 is 0x0

fn get_transform_from_xy(x_cord: i16, y_cord: i16) -> Transform {
    return Transform::from_xyz(
        (-0.9) + ((x_cord as f32) * 0.3),
        0.995,
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
