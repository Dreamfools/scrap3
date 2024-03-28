use comfy::{MathExtensions, Vec2};
use enum_decompose::decompose;
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
    Swap { from: usize, to: usize },
    /// Gem that is currently falling
    Fall { target: usize, height: usize },
}

impl GemAnimationKind {
    /// Creates a new animation, calculating the duration based on the animation kind
    pub fn animate(self, grid: &GridMath, start: f64) -> GemAnimation {
        let duration = match &self {
            GemAnimationKind::Still => f64::INFINITY,
            GemAnimationKind::Held(_) => 0.0,
            GemAnimationKind::Swap(swap) => {
                let distance = grid.distance2(swap.from, swap.to);
                distance as f64 / 10.0
            }
            GemAnimationKind::Fall(fall) => fall.height as f64 / 10.0,
        };
        GemAnimation::new(self, start, start + duration)
    }

    pub fn draw(&self, gem: &Gem, ui: Ui, grid: &GridMath, progress: f32) {
        let ui = ui.shrink(0.1);
        match self {
            GemAnimationKind::Still => draw_gem(gem, ui, 1.0),
            GemAnimationKind::Held(held) => draw_gem(gem, ui.recenter(held.mouse_pos), 0.5),
            GemAnimationKind::Swap(shrink) => {
                let from = grid.center_at_index(shrink.from);
                let to = grid.center_at_index(shrink.to);

                let bulge = 1.0f32;

                let (center, radius) = arc_center_radius(from, to, bulge);

                let (start_angle, end_angle) = arc_angles(center, radius, bulge, from);

                let t = start_angle.lerp(end_angle, progress);
                let pos = Vec2::from_angle(t) * radius + center;

                draw_gem(gem, ui.recenter(pos), 1.0)
            }
            GemAnimationKind::Fall(fall) => {
                let oy = fall.height as f32 * progress;

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

impl GemAnimation {
    pub fn new(kind: GemAnimationKind, start: f64, end: f64) -> Self {
        Self { kind, start, end }
    }

    pub fn still() -> GemAnimationKind {
        GemAnimationKind::Still
    }

    pub fn held(mouse_pos: Vec2) -> GemAnimationKind {
        GemHeldAnimation { mouse_pos }.into()
    }

    pub fn swap(from: usize, to: usize) -> GemAnimationKind {
        GemSwapAnimation { from, to }.into()
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
