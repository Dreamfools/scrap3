use comfy::egui::Slider;
use comfy::{MathExtensions, Vec2};
use enum_decompose::decompose;

use egui_tweak::slider::tweak_slider;
use egui_tweak::ui::tweak_ui;
use math::{arc_angles, arc_center_radius};

use crate::board::gem::{draw_gem, Gem};
use crate::ui::{GridMath, Ui};

#[decompose(prefix = "Gem", suffix = "Animation")]
#[derive(Debug, Clone)]
pub enum GemAnimationKind {
    /// Still gem
    Still,
    /// Gem that is held by mouse
    Held { mouse_pos: Vec2 },
    /// Gem that is currently getting swapped
    Swap { from: usize, to: usize, flip: bool },
    /// Gem that is currently falling
    Fall { target: usize, height: usize },
}

impl GemAnimationKind {
    /// Creates a new animation, calculating the duration based on the animation kind
    pub fn animate(self, grid: &GridMath, start: f64, speed: f64) -> GemAnimation {
        let duration = match &self {
            GemAnimationKind::Still => f64::INFINITY,
            GemAnimationKind::Held(_) => 0.0,
            GemAnimationKind::Swap(swap) => {
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
            GemAnimationKind::Fall(fall) => fall.height as f64 / 10.0,
        } * speed;
        GemAnimation::new(self, start, start + duration)
    }

    pub fn draw(&self, gem: &Gem, ui: Ui, grid: &GridMath, progress: f32) {
        let ui = ui.shrink(0.1);
        match self {
            GemAnimationKind::Still => draw_gem(gem, ui, 1.0),
            GemAnimationKind::Held(held) => draw_gem(
                gem,
                ui.recenter(held.mouse_pos).next_layer().next_layer(),
                0.5,
            ),
            GemAnimationKind::Swap(swap) => {
                if swap.from == swap.to {
                    draw_gem(gem, ui, 1.0);
                    return;
                }
                let from = grid.center_at_index(swap.from);
                let to = grid.center_at_index(swap.to);

                let bulge = grid.cell_height() * tweak_slider("board.swapHeight", 0.66, 0.01, 1.0)
                    / from.distance(to);

                let (center, radius) = arc_center_radius(from, to, bulge, swap.flip);

                let (start_angle, end_angle) =
                    arc_angles(center, radius, bulge, from, to, swap.flip);

                let progress = if swap.flip { 1.0 - progress } else { progress };

                let t = start_angle.lerp(end_angle, progress);
                let pos = Vec2::from_angle(t) * radius + center;

                draw_gem(gem, ui.recenter(pos).next_layer(), 1.0)
            }
            GemAnimationKind::Fall(fall) => {
                let oy = fall.height as f32 * grid.cell_height() * progress;

                draw_gem(gem, ui.map_rect(|r| r.shift((0.0, -oy))), 1.0)
            }
        }
    }
}

#[derive(Debug)]
pub struct GemAnimation {
    kind: GemAnimationKind,
    start: f64,
    end: f64,
}

impl Default for GemAnimation {
    fn default() -> Self {
        Self::still()
    }
}

impl GemAnimation {
    pub fn new(kind: impl Into<GemAnimationKind>, start: f64, end: f64) -> Self {
        Self {
            kind: kind.into(),
            start,
            end,
        }
    }

    pub fn still() -> Self {
        Self::new(GemAnimationKind::Still, 0.0, f64::INFINITY)
    }

    pub fn held(mouse_pos: Vec2) -> Self {
        Self::new(GemHeldAnimation { mouse_pos }, 0.0, 0.0)
    }

    pub fn swap(from: usize, to: usize, flip: bool) -> GemAnimationKind {
        GemSwapAnimation { from, to, flip }.into()
    }

    pub fn fall(target: usize, height: usize) -> GemAnimationKind {
        GemFallAnimation { target, height }.into()
    }

    pub fn update(&self, gem: &Gem, ui: Ui, grid: &GridMath, time: f64) {
        let progress = ((time - self.start) / (self.end - self.start)).clamp(0.0, 1.0);

        self.kind.draw(gem, ui, grid, progress as f32)
    }

    pub fn done(&self, time: f64) -> bool {
        time >= self.end
    }
}
