use crate::state::combat::CombatState;
use crate::ui::board::board;
use crate::user_textures::clear_user_textures;
use macroquad::color::WHITE;
use macroquad::prelude::{clear_background, next_frame, screen_height};
use macroquad::time::get_time;
use miette::{Diagnostic, IntoDiagnostic, Report};
use model::assets_manager::{source, AssetCache};
use model::LoadedMod;
use std::fmt::Display;
use std::process::exit;
use thiserror::Error;
use yakui_tweak::editor::edit_tweakables;
use yakui_tweak::tweak;

pub mod random;
pub mod state;
pub mod ui;
pub mod user_textures;

#[cfg(target_arch = "wasm32")]
mod getrandom_on_web;

#[macroquad::main("scrap3")]
async fn main() {
    // println!("Cur dir: {}", std::env::current_dir().unwrap().display());

    #[cfg(not(target_arch = "wasm32"))]
    let source = source::FileSystem::new("game/mod")
        .into_diagnostic()
        .unwrap_or_else(|e| exit_on_error(e, "Failed to load mod"));

    #[cfg(target_arch = "wasm32")]
    let source = source::Embedded::from(source::embed!("game/mod"));

    let cache = AssetCache::with_source(source);

    let mut mod_data =
        LoadedMod::load_mod(&cache).unwrap_or_else(|e| exit_on_error(e, "Failed to load mod"));

    loop {
        mod_data.hot_reload().unwrap_or_else(|e| {
            print_error(e, "Hot reloading failed");
            false
        });

        let registry = mod_data.registry();
        let state = CombatState::new(registry);

        clear_user_textures();

        clear_background(WHITE);

        yakui_macroquad::start();

        yakui_macroquad::cfg(|yak| {
            yak.set_scale_factor(screen_height() / tweak("Screen Height", 600.0));
        });

        yakui::center(|| {
            board(&state, registry, get_time());
            // yakui::colored_box_container(yakui::Color::CORNFLOWER_BLUE, || {
            //     yakui::column(|| {
            //         yakui::textbox("test");
            //         yakui::pad(yakui::widgets::Pad::all(16.0), || {
            //             let size = registry.settings.logo.texture().size();
            //             let ar = size.x / size.y;
            //             constrained(Constraints::tight(Vec2::new(128.0 * ar, 128.0)), || {
            //                 offset(Vec2::new(128.0, 0.0), || {
            //                     yakui::image(
            //                         registry.settings.logo.yakui_id(),
            //                         Vec2::new(128.0 * ar, 128.0),
            //                     );
            //                 });
            //             });
            //         });
            //     });
            // });
        });

        edit_tweakables();

        yakui_macroquad::finish();

        yakui_macroquad::draw();

        // let s: &RegistryEntry<GemColorModel> = &registry.gem_color
        //     [slabmap::SlabMapUntypedId::from_raw_unchecked(0).as_typed_unchecked()];
        // let tx = s.data.sprite.texture2d();
        // draw_texture(&tx, 0.0, 0.0, WHITE);

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
