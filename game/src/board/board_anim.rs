use comfy::egui::Slider;
use comfy::{MathExtensions, Vec2};
use enum_decompose::decompose;
use strum::EnumIs;

use egui_tweak::slider::tweak_slider;
use egui_tweak::ui::tweak_ui;
use math::{arc_angles, arc_center_radius};
use with_setter_macro::WithSetters;

use crate::board::gem::{draw_gem, Gem};
use crate::ui::{GridMath, Ui};

#[decompose(prefix = "Gem", suffix = "Animation")]
#[derive(Debug, Clone)]
pub enum GemMovement {
    /// Still gem
    Still,
    /// Gem that is held by mouse
    Held { mouse_pos: Vec2 },
    /// Gem that is currently getting swapped
    Swap { from: usize, to: usize, flip: bool },
    /// Gem that is currently falling
    Fall { target: usize, height: usize },
}

impl GemMovement {
    /// Creates a new animation, calculating the duration based on the animation kind
    pub fn animate(
        self,
        grid: &GridMath,
        start: f64,
        speed: f64,
        easing: fn(f32) -> f32,
    ) -> GemAnimation {
        let duration = match &self {
            GemMovement::Still => f64::INFINITY,
            GemMovement::Held(_) => 0.0,
            GemMovement::Swap(swap) => {
                let distance = grid.grid_distance(swap.from, swap.to);
                distance as f64
                    / tweak_ui(
                        "board.swapSpeed",
                        12.5,
                        &|ui: &mut comfy::egui::Ui, value: &mut f64| {
                            ui.add(Slider::new(value, 0.1..=20.0).logarithmic(true));
                        },
                    )
            }
            GemMovement::Fall(fall) => fall.height as f64 / 10.0,
        } * speed;
        GemAnimation::new(self, start, start + duration, easing)
    }

    pub fn ui(&self, ui: Ui, grid: &GridMath, progress: f32) -> Ui {
        let ui = ui.shrink(0.1);
        match self {
            GemMovement::Still => ui,
            GemMovement::Held(held) => ui.recenter(held.mouse_pos).next_layer().next_layer(),
            GemMovement::Swap(swap) => {
                match swap_pos(grid, progress, swap.from, swap.to, swap.flip) {
                    None => ui,
                    Some(pos) => ui.recenter(pos).next_layer(),
                }
            }
            GemMovement::Fall(fall) => {
                let oy = fall.height as f32 * grid.cell_height() * progress;

                ui.map_rect(|r| r.shift((0.0, -oy)))
            }
        }
    }
}

fn swap_pos(grid: &GridMath, progress: f32, from: usize, to: usize, flip: bool) -> Option<Vec2> {
    if from == to {
        return None;
    }
    let from = grid.center_at_index(from);
    let to = grid.center_at_index(to);

    let bulge =
        grid.cell_height() * tweak_slider("board.swapHeight", 0.7, 0.01, 1.0) / from.distance(to);

    let (center, radius) = arc_center_radius(from, to, bulge, flip);

    let (start_angle, end_angle) = arc_angles(center, radius, bulge, from, to, flip);

    let progress = if flip { 1.0 - progress } else { progress };

    let t = start_angle.lerp(end_angle, progress);
    let pos = Vec2::from_angle(t) * radius + center;
    Some(pos)
}

#[derive(Debug, Clone, Default, EnumIs)]
pub enum GemVisuals {
    #[default]
    Normal,
    Ghost,
    Held,
    Cracking,
}

impl GemVisuals {
    pub fn draw(&self, gem: &Gem, mut ui: Ui, progress: f32) {
        let opacity = match self {
            GemVisuals::Normal => 1.0,
            GemVisuals::Ghost => tweak_slider("board.opacity.ghost", 0.3, 0.0, 1.0),
            GemVisuals::Held => tweak_slider("board.opacity.held", 0.7, 0.0, 1.0),
            GemVisuals::Cracking => 1.0 - progress,
        };

        if self.is_cracking() {
            ui = ui.map_rect(|r| {
                r.contract_x(r.width() * simple_easing::cubic_out(progress) / 2.0)
                    .contract_y(r.height() * simple_easing::back_in(progress) / 2.0)
            });
        }

        draw_gem(gem, ui, opacity);
    }
}

#[derive(Debug, Clone, WithSetters)]
pub struct GemAnimation {
    pub movement: GemMovement,
    #[setters(vis = "pub")]
    pub visuals: GemVisuals,
    pub start: f64,
    pub end: f64,
    pub easing: fn(f32) -> f32,
}

impl Default for GemAnimation {
    fn default() -> Self {
        Self::still()
    }
}

impl GemAnimation {
    pub fn new(kind: impl Into<GemMovement>, start: f64, end: f64, easing: fn(f32) -> f32) -> Self {
        Self {
            movement: kind.into(),
            visuals: Default::default(),
            start,
            end,
            easing,
        }
    }

    pub fn still() -> Self {
        Self::new(
            GemMovement::Still,
            0.0,
            f64::INFINITY,
            simple_easing::linear,
        )
    }

    pub fn held(mouse_pos: Vec2) -> Self {
        Self::new(
            GemHeldAnimation { mouse_pos },
            0.0,
            0.0,
            simple_easing::linear,
        )
    }

    pub fn swap(from: usize, to: usize, flip: bool) -> GemMovement {
        GemSwapAnimation { from, to, flip }.into()
    }

    pub fn fall(target: usize, height: usize) -> GemMovement {
        GemFallAnimation { target, height }.into()
    }

    pub fn crack(start: f64, speed: f64, easing: fn(f32) -> f32) -> Self {
        let crack_duration = tweak_slider("board.crackTime", 1.0 / 3.0, 0.0, 1.0) * speed;
        Self {
            movement: GemMovement::Still,
            visuals: GemVisuals::Cracking,
            start,
            end: start + crack_duration,
            easing,
        }
    }

    pub fn update(&self, gem: &Gem, ui: Ui, grid: &GridMath, time: f64) {
        let progress = ((time - self.start) / (self.end - self.start)).clamp(0.0, 1.0);

        let ui = self.movement.ui(ui, grid, progress as f32);
        self.visuals.draw(gem, ui, progress as f32);
    }

    pub fn replace_if_still(&mut self, anim: impl FnOnce() -> Self) {
        if let GemMovement::Still = self.movement {
            *self = anim()
        }
    }

    pub fn replace_if_done(&mut self, time: f64, anim: impl FnOnce() -> Self) {
        if self.done(time) {
            *self = anim()
        }
    }

    pub fn done(&self, time: f64) -> bool {
        time >= self.end
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.end = self.start + duration;
        self
    }

    pub fn with_delay(mut self, delay: f64) -> Self {
        self.start += delay;
        self
    }
}
