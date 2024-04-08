use crate::board::BoardScene;
use crate::ui::Ui;
use comfy::*;
pub use rect::Rect as CutRect;

mod assets;
mod board;
mod ui;

simple_game!("Contexts are fun :)", GameState, config, setup, update);

pub trait Scene {
    fn update(&mut self, ui: Ui, now: f64);
}

pub struct GameState {
    pub scene: Box<dyn Scene>,
}

impl GameState {
    pub fn new(_c: &EngineState) -> Self {
        Self {
            scene: Box::new(BoardScene::new(6, 5)),
        }
    }
}

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        resolution: ResolutionConfig::Physical(1080 * 9 / 16, 1080),
        min_resolution: ResolutionConfig::Physical(108, 108 * 16 / 9),
        ..config
    }
}

pub fn screen_rect() -> CutRect {
    let screen_origin = screen_to_world((0.0, 0.0).into());
    let screen_end = screen_to_world((screen_width(), screen_height()).into());
    CutRect::new(screen_origin.x, screen_end.y, screen_end.x, screen_origin.y)
}

// pub fn screen_rect() -> CutRect {
//     let tl = MAIN_CAMERA.borrow().world_viewport();
//
//     CutRect::new(-tl.x / 2.0, -tl.y / 2.0, tl.x / 2.0, tl.y / 2.0)
// }

fn load_asset(c: &mut EngineContext, path: &str) {
    let bytes = fs_err::read(Path::join(env!("CARGO_MANIFEST_DIR").as_ref(), "assets").join(path))
        .expect("Should be able to read the asset");
    c.load_texture_from_bytes(path, &bytes);
}

fn setup(_state: &mut GameState, c: &mut EngineContext) {
    load_asset(c, assets::gems::gem_type_4_black);
    load_asset(c, assets::gems::gem_type_3_black);
    load_asset(c, assets::gems::gem_type_2_black);
    load_asset(c, assets::gems::gem_type_1_black);
}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    let mut camera = MAIN_CAMERA.borrow_mut();
    camera.zoom = screen_height();
    camera.aspect_ratio = screen_width() / screen_height();
    drop(camera);
    // let rect = screen_rect().contract(1.0);
    // draw_ellipse(
    //     rect.center().into(),
    //     vec2(rect.width() / 2.0, rect.height() / 2.0),
    //     GREEN,
    //     0,
    // );
    state.scene.update(Ui::new(screen_rect(), 0), get_time());
    #[cfg(debug_assertions)]
    debug();
}

fn debug() {
    egui::Window::new("Tweaking")
        .default_open(false)
        .show(egui(), |ui| {
            egui_tweak::editor::edit_tweakables(ui);
        });
}
