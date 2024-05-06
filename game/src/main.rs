use macroquad::color::WHITE;
use macroquad::prelude::{clear_background, next_frame, screen_height};
use yakui_macroquad::*;

pub mod user_textures;

#[macroquad::main("yakui-macroquad-example")]
async fn main() {
    loop {
        clear_background(WHITE);

        // Can also use yakui_macroquad::ui(|_| { /* draw stuff here */}); to avoid start and finish

        yakui_macroquad::start();

        yakui_macroquad::cfg(|yak| {
            yak.set_scale_factor(screen_height() / 1080.0);
        });

        yakui::center(|| {
            yakui::colored_box_container(yakui::Color::CORNFLOWER_BLUE, || {
                yakui::pad(yakui::widgets::Pad::all(16.0), || {
                    yakui::text(32.0, "hello, world!");
                });
            });
        });

        yakui_macroquad::finish();

        yakui_macroquad::draw();

        next_frame().await;
    }
}
