use bevy::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

pub struct Renderer;
impl Plugin for Renderer {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    width: WIDTH,
                    height: HEIGHT,
                    title: "Strategy RPG".to_string(),
                    resizable: true,
                    ..default()
                },
                ..default()
            }));
    }
}
