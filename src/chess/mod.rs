use bevy::prelude::*;

use self::chess_piece_control::handle_gravity;
pub mod chess_piece_control;
pub mod spawn_chess_board;

pub struct ChessPlugin;
impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<spawn_chess_board::ChessSquare>()
            .register_type::<chess_piece_control::ChessPiece>()
            .add_startup_system(spawn_chess_board::spawn_chess_board)
            .add_startup_system(chess_piece_control::create_all_pieces)
            //.add_system(chess_piece_control::change_middle_square)
            .add_system(handle_gravity);
    }
}
