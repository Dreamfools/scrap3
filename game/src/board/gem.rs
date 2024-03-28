use crate::ui::Ui;
use comfy::{draw_sprite_pro, texture_id, Color, DrawTextureProParams, SpriteAlign};
use inline_tweak::tweak_fn;
use match3::{BoardGem, MatchColor, SimpleGem};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum GemColor {
    #[default]
    Empty,
    Red,
    Green,
    Blue,
    Light,
    Dark,
}

pub static GEM_COLORS: [GemColor; 5] = [
    GemColor::Red,
    GemColor::Green,
    GemColor::Blue,
    GemColor::Light,
    GemColor::Dark,
];

impl MatchColor for GemColor {
    fn matches(&self, other: &Self) -> bool {
        self == other
    }

    fn can_start_match(&self) -> bool {
        self != &GemColor::Empty
    }
}

pub type Gem = SimpleGem<GemColor>;

#[tweak_fn]
pub fn draw_gem(gem: &Gem, ui: Ui, alpha: f32) {
    let ui = ui.trim_to_aspect_ratio(1.0);
    let center = ui.rect.center().into();
    let color = match gem.color() {
        GemColor::Empty => Color::rgb8(0x0, 0x0, 0x0),
        GemColor::Red => Color::rgb8(0xFF, 0x10, 0x0),
        GemColor::Green => Color::rgb8(0x0, 0xFF, 0x0),
        GemColor::Blue => Color::rgb8(0x00, 0x33, 0xff),
        GemColor::Light => Color::rgb8(0xFF, 0xFF, 0x44),
        GemColor::Dark => Color::rgb8(0x55, 0x33, 0xFF),
    }
    .alpha(alpha);
    // info!(
    //     "Drawing gem; gem={:?} center={:?}, r={:?}, color={:?}",
    //     gem, center, radius, color,
    // );

    draw_sprite_pro(
        texture_id(crate::assets::gems::gem_type_2_black),
        center,
        color,
        ui.z,
        DrawTextureProParams {
            source_rect: None,
            align: SpriteAlign::Center,
            pivot: None,
            size: ui.rect.into(),
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            blend_mode: Default::default(),
            rotation_x: 0.0,
            y_sort_offset: 0.0,
        },
    );

    // draw_rect(center, vec2(ui.rect.width(), ui.rect.height()), color, ui.z);
    // draw_circle(center, radius, color, ui.z)
}
