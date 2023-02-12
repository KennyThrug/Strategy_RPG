mod chess;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::spawn_camera;
mod camera;
mod renderer;

fn main() {
    App::new()
        .add_plugin(renderer::Renderer)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_plugin(chess::ChessPlugin)
        //.register_type::<chess::spawn_chess_board::ChessSquare>()
        //.add_startup_system(chess::spawn_chess_board::spawn_chess_board)
        //.add_startup_system(spawn_chess_board::asset_loading)
        //.add_startup_system(chess::chess_piece_control::create_game_piece)
        //.add_system(spawnChessBoard::tower_shooting)
        .add_plugin(WorldInspectorPlugin)
        .run();
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}
