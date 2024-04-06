use crate::board::board::{random_board, BoardState, GemBoard};
use crate::board::board_anim::{GemAnimation, GemVisuals};

use crate::ui::{GridMath, Ui};
use crate::Scene;
use comfy::{
    draw_circle, draw_rect, get_time, is_mouse_button_down, mouse_world, Color, MouseButton, Vec2,
};
use egui_tweak::tweak;
use inline_tweak::tweak_fn;
use match3::rect_board::GridMoveStrategy;
use match3::Shape;

pub mod board;
pub mod board_anim;
pub mod gem;

impl BoardScene {
    pub fn new(width: usize, height: usize) -> Self {
        let (board, state) = random_board(width, height);
        Self {
            animations: board
                .board
                .iter()
                .map(|_| GemAnimation::default())
                .collect(),
            extra_animations: vec![],
            state,
            board,
            held_gem: None,
        }
    }
}
#[derive(Debug)]
pub struct BoardScene {
    state: BoardState,
    animations: Vec<GemAnimation>,
    /// Extra animations for gems that are not on the board (like the held gem)
    extra_animations: Vec<(usize, GemAnimation)>,
    board: GemBoard,
    held_gem: Option<(usize, Vec2)>,
}

impl BoardScene {
    fn process_input(&mut self, interactive_ui: Ui, gmath: &GridMath, time: f64) {
        let mouse_pos = mouse_world();
        let trapped_pos = interactive_ui.clamp_pos(mouse_pos);

        // println!("Mouse pos: {:?}, trapped pos: {:?}, state: {:?}, held_gem: {:?}, mouse_down: {:?}", mouse_pos, trapped_pos, self.state, self.held_gem, is_mouse_button_down(MouseButton::Left));

        if !matches!(self.state, BoardState::Idle | BoardState::Matching) {
            return;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let pressed = gmath.pos_to_index(trapped_pos);
            let pressed_cell = gmath.rect_at_index(pressed);
            if let Some((held, touch)) = &mut self.held_gem {
                *touch = trapped_pos;

                let dist2 = trapped_pos.distance_squared(pressed_cell.center().into());
                let hit_radius = pressed_cell.width() / 1.8;

                if tweak("board.showMatchHitbox", false) {
                    draw_circle(
                        pressed_cell.center().into(),
                        hit_radius,
                        Color::rgba8(255, 0, 0, 80),
                        interactive_ui.z,
                    );
                }

                if *held != pressed && dist2 <= hit_radius * hit_radius {
                    for cell in self
                        .board
                        .move_gem(*held, pressed, GridMoveStrategy::Diagonals)
                    {
                        let [fx, fy] = gmath.shape().delinearize(*held);
                        let [tx, ty] = gmath.shape().delinearize(cell);
                        let flip = tx > fx || ty > fy;
                        self.animations[cell] = GemAnimation::swap(*held, cell, flip)
                            .animate(gmath, time, 1.0)
                            .with_visuals(GemVisuals::Ghost);
                        self.animations[*held] =
                            GemAnimation::swap(cell, *held, flip).animate(gmath, time, 1.0);

                        self.board.board.swap(cell, *held);
                        *held = cell;
                    }
                }
            } else if interactive_ui.rect.contains(mouse_pos) {
                self.held_gem = Some((pressed, trapped_pos));
            }
        } else {
            self.held_gem = None;
        }

        // Assigning held animation to the held gem
        if tweak("board.showHeldAnimation", true) {
            if let Some((id, pos)) = self.held_gem {
                self.extra_animations
                    .push((id, GemAnimation::held(pos).with_visuals(GemVisuals::Held)));
                self.animations[id].replace_if_still(|| {
                    GemAnimation::still()
                        .with_visuals(GemVisuals::Ghost)
                        .with_duration(0.0)
                });
            }
        }
    }

    fn check_animations(&mut self, time: f64) {
        for anim in &mut self.animations {
            anim.replace_if_done(time, GemAnimation::still);
        }
        self.extra_animations.retain(|(_, anim)| !anim.done(time));
    }
}

#[tweak_fn]
fn board_colors() -> (Color, Color) {
    (Color::rgb8(0x49, 0x2b, 0x1b), Color::rgb8(0x36, 0x20, 0x16))
}

impl Scene for BoardScene {
    // #[tweak_fn]
    fn update(&mut self, ui: Ui) {
        let [width, height] = self.board.shape.as_array();

        let ui = ui.trim_to_aspect_ratio(width as f32 / height as f32);

        let gmath = GridMath::new(ui.rect, width, height);
        let grid_layer = ui.next_layer();
        let grid = grid_layer.clone().grid(width, height);

        let interactive_ui = ui.clone().shrink(0.01);

        let now = get_time();

        self.process_input(interactive_ui, &gmath, now);

        let (main, secondary) = board_colors();
        let bg_z = ui.z;
        draw_rect(ui.rect.center().into(), ui.rect.into(), main, bg_z);
        for (i, (ui, gem)) in grid.into_iter().zip(self.board.board.iter()).enumerate() {
            // Draw grid BG
            if (i / width + i % width) % 2 == 0 {
                draw_rect(ui.rect.center().into(), ui.rect.into(), secondary, bg_z)
            };
            self.animations[i].update(gem, ui, &gmath, now);
        }

        for (pos, animation) in &self.extra_animations {
            let ui = grid_layer.clone().with_rect(gmath.rect_at_index(*pos));
            animation.update(&self.board.board[*pos], ui, &gmath, now);
        }

        self.check_animations(now);
    }
}
