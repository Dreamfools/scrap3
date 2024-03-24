use crate::board::{draw_board, random_board};
use comfy::*;
pub use rect::Rect as CutRect;

mod board;

simple_game!("Nice red circle", setup, update);

pub fn screen_rect() -> CutRect {
    let screen_origin = screen_to_world((0.0, 0.0).into());
    let screen_end = screen_to_world((screen_width(), screen_height()).into());
    CutRect::new(screen_origin.x, screen_end.y, screen_end.x, screen_origin.y)
}

fn setup(_c: &mut EngineContext) {
    world_mut().spawn(random_board(10, 10));
}

fn update(_c: &mut EngineContext) {
    draw_board();
}
