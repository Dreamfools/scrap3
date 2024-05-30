use enum_decompose::decompose;
use strum::EnumIs;

use math::arc::{arc_angles, arc_center_radius};
use math::board::{CellIndex, GridMath};
use math::glam::{FloatExt, Vec2};
use math::glamour::{Point2, Rect, Unit, Vector2};
use math::gravity::{fall_time, height_at_fall_progress};
use with_setter_macro::WithSetters;
use yakui_tweak::slider::tweak_slider;
use yakui_tweak::tweak;

/// Board view in local coordinate space
///
/// All board view coordinates should be normalized to 0..1
pub struct BoardView;
impl Unit for BoardView {
    type Scalar = f32;

    fn name() -> Option<&'static str> {
        Some("Board Unit")
    }
}

pub type BoardViewPos = Point2<BoardView>;
pub type BoardViewRect = Rect<BoardView>;

#[derive(Debug, Clone)]
pub struct GemDrawInfo {
    pub rect: BoardViewRect,
    pub layer: usize,
    pub opacity: f32,
}

pub fn fall_gravity() -> f32 {
    tweak("board.gravity", 48.0)
}

#[decompose(prefix = "Gem", suffix = "Animation")]
#[derive(Debug, Clone)]
pub enum GemMovement {
    /// Still gem
    Still,
    /// Gem that is held by mouse
    Held { mouse_pos: BoardViewPos },
    /// Gem that is currently getting swapped
    Swap {
        from: CellIndex,
        to: CellIndex,
        flip: bool,
    },
    /// Gem that is currently falling
    Fall { target: CellIndex, height: usize },
}

impl GemMovement {
    /// Creates a new animation, calculating the duration based on the animation kind
    pub fn animate(
        self,
        grid: &GridMath<BoardView>,
        start: f64,
        speed: f64,
        easing: fn(f32) -> f32,
    ) -> GemAnimation {
        let duration = match &self {
            GemMovement::Still => f64::INFINITY,
            GemMovement::Held(_) => 0.0,
            GemMovement::Swap(swap) => {
                let distance = grid.grid_distance(swap.from, swap.to);
                distance as f64 / tweak_slider("board.swapSpeed", 12.5, 0.1, 20.0)
            }
            GemMovement::Fall(fall) => fall_time(fall.height as f32, fall_gravity()) as f64,
        } * speed;
        GemAnimation::new(self, start, start + duration, easing)
    }

    pub fn layout(&self, rect: &mut GemDrawInfo, grid: &GridMath<BoardView>, progress: f32) {
        rect.rect.origin += Vector2::from(rect.rect.size * 0.05);
        rect.rect.size *= 0.9;
        match self {
            GemMovement::Still => {}
            GemMovement::Held(held) => {
                rect.rect.origin = held.mouse_pos;
                rect.layer += 2;
            }
            GemMovement::Swap(swap) => {
                match swap_pos(grid, progress, swap.from, swap.to, swap.flip) {
                    None => {}
                    Some(pos) => {
                        rect.rect.origin = pos;
                        rect.layer += 2;
                    }
                }
            }
            GemMovement::Fall(fall) => {
                let oy = height_at_fall_progress(fall.height as f32, fall_gravity(), progress)
                    * grid.cell_height();

                rect.rect.origin.y += oy;
            }
        }
    }
}

fn swap_pos(
    grid: &GridMath<BoardView>,
    progress: f32,
    from: CellIndex,
    to: CellIndex,
    flip: bool,
) -> Option<BoardViewPos> {
    if from == to {
        return None;
    }
    let from = grid.center_at_index(from);
    let to = grid.center_at_index(to);

    let bulge =
        grid.cell_height() * tweak_slider("board.swapHeight", 0.7, 0.01, 1.0) / from.distance(to);

    let (center, radius) = arc_center_radius(from.into(), to.into(), bulge, flip);

    let (start_angle, end_angle) = arc_angles(center, radius, bulge, from.into(), to.into(), flip);

    let progress = if flip { 1.0 - progress } else { progress };

    let t = start_angle.lerp(end_angle, progress);
    let pos = Vec2::from_angle(t) * radius + center;
    Some(pos.into())
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
    pub fn visuals(&self, ui: &mut GemDrawInfo, progress: f32) {
        let opacity = match self {
            GemVisuals::Normal => 1.0,
            GemVisuals::Ghost => tweak_slider("board.opacity.ghost", 0.3, 0.0, 1.0),
            GemVisuals::Held => tweak_slider("board.opacity.held", 0.7, 0.0, 1.0),
            GemVisuals::Cracking => 1.0 - progress,
        };

        if self.is_cracking() {
            ui.rect.size.width *= 1.0 - simple_easing::cubic_out(progress) / 2.0;
            ui.rect.size.height *= 1.0 - simple_easing::back_in(progress) / 2.0;
        }

        ui.opacity = opacity;
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

    pub fn held(mouse_pos: BoardViewPos) -> Self {
        Self::new(
            GemHeldAnimation { mouse_pos },
            0.0,
            0.0,
            simple_easing::linear,
        )
    }

    pub fn swap(from: CellIndex, to: CellIndex, flip: bool) -> GemMovement {
        GemSwapAnimation { from, to, flip }.into()
    }

    pub fn fall(target: CellIndex, height: usize) -> GemMovement {
        GemFallAnimation { target, height }.into()
    }

    pub fn crack(start: f64, speed: f64, easing: fn(f32) -> f32) -> Self {
        let crack_duration = tweak_slider("board.crackTime", 0.3, 0.0, 1.0) * speed;
        Self {
            movement: GemMovement::Still,
            visuals: GemVisuals::Cracking,
            start,
            end: start + crack_duration,
            easing,
        }
    }

    pub fn update(&self, pos: CellIndex, grid: &GridMath<BoardView>, time: f64) -> GemDrawInfo {
        let mut ui = GemDrawInfo {
            rect: grid.rect_at_index(pos),
            layer: 0,
            opacity: 1.0,
        };
        let progress = ((time - self.start) / (self.end - self.start)).clamp(0.0, 1.0);

        self.movement.layout(&mut ui, grid, progress as f32);
        self.visuals.visuals(&mut ui, progress as f32);
        ui
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
