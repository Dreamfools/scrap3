use crate::user_textures::{clear_user_textures, SpriteDataExt};
use macroquad::color::WHITE;
use macroquad::prelude::{clear_background, next_frame, screen_height};
use miette::{Diagnostic, IntoDiagnostic, Report};
use model::assets_manager::AssetCache;
use model::LoadedMod;
use std::fmt::Display;
use std::process::exit;
use thiserror::Error;

pub mod user_textures;

#[macroquad::main("scrap3")]
async fn main() {
    println!("Cur dir: {}", std::env::current_dir().unwrap().display());

    let cache = AssetCache::new("game/mod")
        .into_diagnostic()
        .unwrap_or_else(|e| todo!("Handle AssetCache::new error: {}", e));

    let mut mod_data =
        LoadedMod::load_mod(&cache).unwrap_or_else(|e| exit_on_error(e, "Failed to load mod"));

    loop {
        mod_data.hot_reload().unwrap_or_else(|e| {
            print_error(e, "Hot reloading failed");
            false
        });

        let registry = mod_data.registry();

        clear_user_textures();

        clear_background(WHITE);

        yakui_macroquad::start();

        yakui_macroquad::cfg(|yak| {
            yak.set_scale_factor(screen_height() / 1080.0);
        });

        yakui::center(|| {
            yakui::colored_box_container(yakui::Color::CORNFLOWER_BLUE, || {
                yakui::pad(yakui::widgets::Pad::all(16.0), || {
                    let size = registry.settings.logo.texture().size();
                    let ar = size.x / size.y;
                    yakui::image(
                        registry.settings.logo.yakui_id(),
                        yakui::geometry::Vec2::new(128.0 * ar, 128.0),
                    );
                });
            });
        });

        yakui_macroquad::finish();

        yakui_macroquad::draw();

        next_frame().await;
    }
}

#[derive(Debug, Error, Diagnostic)]
#[error("{}", .0)]
struct Reporter(String, #[diagnostic_source] Report);
fn print_error(err: Report, context: impl Display) {
    println!("{:?}", Report::from(Reporter(context.to_string(), err)))
}

fn exit_on_error(err: Report, context: impl Display) -> ! {
    print_error(err, context);
    exit(1);
}
