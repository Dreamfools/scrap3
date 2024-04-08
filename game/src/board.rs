use crate::board::board_anim::{GemAnimation, GemVisuals};
use crate::board::state::{
    random_board, random_gem, BoardState, GemBoard, MatchingState, MovingState, RefillingState,
};

use crate::board::gem::{Gem, GemColor};
use crate::ui::{GridMath, Ui};
use crate::Scene;
use comfy::{
    debug, draw_circle, draw_rect, info, is_mouse_button_down, mouse_world, Color, MouseButton,
    Vec2,
};
use egui_tweak::display::debug_display;
use egui_tweak::tweak;
use inline_tweak::tweak_fn;
use match3::line::LineMatcherSettings;
use match3::rect_board::GridMoveStrategy;
use match3::refilling::{remove_matched, GravityRefill};
use match3::{Shape, SimpleGem};

pub mod board_anim;
pub mod gem;
pub mod state;

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
            next_state: None,
        }
    }
}
#[derive(Debug)]
pub struct BoardScene {
    state: BoardState,
    next_state: Option<BoardState>,
    animations: Vec<GemAnimation>,
    /// Extra animations for gems that are not on the board (like the held gem)
    extra_animations: Vec<(usize, Gem, GemAnimation)>,
    board: GemBoard,
    held_gem: Option<(usize, Vec2)>,
}

impl BoardScene {
    fn next_state(&mut self, state: BoardState) {
        info!(
            "State transition requested: {:?} -> {:?}",
            self.state, state
        );
        if let Some(next_state) = &self.next_state {
            #[cfg(debug_assertions)]
            panic!(
                "State transition already queued: {:?} -> {:?}",
                self.state, next_state
            );
            #[cfg(not(debug_assertions))]
            error!(
                "State transition already queued: {:?} -> {:?}",
                self.state, next_state
            )
        }
        self.next_state = Some(state);
    }

    fn move_held_gem(&mut self, gmath: &GridMath, time: f64, to: usize) {
        let (held, _) = self.held_gem.as_mut().expect("Should have a held gem");

        debug!("Begin swap from held cell {} to {}", *held, to);
        for cell in self.board.move_gem(*held, to, GridMoveStrategy::Diagonals) {
            let [fx, fy] = gmath.shape().delinearize(*held);
            let [tx, ty] = gmath.shape().delinearize(cell);
            let flip = tx > fx || ty > fy;

            if tweak("board.showSwapAnimation", true) {
                self.animations[cell] = GemAnimation::swap(*held, cell, flip)
                    .animate(gmath, time, 1.0, simple_easing::linear)
                    .with_visuals(GemVisuals::Ghost);
                self.animations[*held] = GemAnimation::swap(cell, *held, flip).animate(
                    gmath,
                    time,
                    1.0,
                    simple_easing::linear,
                );
            }

            self.board.board.swap(cell, *held);
            debug!("Swapping {} and {}", *held, cell);
            *held = cell;
        }
    }

    /// Process input and updates board state and animations
    fn process_input(&mut self, interactive_ui: Ui, gmath: &GridMath, time: f64) {
        let mouse_pos = mouse_world();
        let trapped_pos = interactive_ui.clamp_pos(mouse_pos);

        if !self.state.is_idle() && !self.state.is_moving() {
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
                    if self.state.is_idle() {
                        self.start_moving(time);
                    } else {
                        self.move_held_gem(gmath, time, pressed);
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
                self.extra_animations.push((
                    id,
                    self.board.board[id].clone(),
                    GemAnimation::held(pos).with_visuals(GemVisuals::Held),
                ));
                self.animations[id].replace_if_still(|| {
                    GemAnimation::still()
                        .with_visuals(GemVisuals::Ghost)
                        .with_duration(0.0)
                });
            }
        }

        if !tweak("board.unlimitedMatching", false)
            && self.held_gem.is_none()
            && self.state.is_moving()
        {
            self.start_matching();
        }
    }

    fn update_state(&mut self, now: f64) {
        match &mut self.state {
            BoardState::Idle => {}
            BoardState::Moving(_) => {}
            BoardState::Refilling(refilling) => {
                if now >= refilling.end {
                    self.start_matching()
                }
            }
            BoardState::Matching(matching) => {
                if now >= matching.end {
                    if matching.groups.is_empty() {
                        self.start_idle()
                    } else {
                        self.start_refilling()
                    }
                }
            }
        }
    }

    fn start_refilling(&mut self) {
        self.next_state(BoardState::Refilling(RefillingState { end: 0.0 }));
    }

    fn start_matching(&mut self) {
        let matches = self
            .board
            .find_matches_linear(&LineMatcherSettings::common_match3().with_merge_neighbours(true));

        self.next_state(BoardState::Matching(MatchingState {
            groups: matches,
            end: 0.0,
        }));
    }

    fn start_moving(&mut self, now: f64) {
        self.next_state(BoardState::Moving(MovingState {
            spin_end: now + 5.0,
        }));
    }

    fn start_idle(&mut self) {
        self.next_state(BoardState::Idle)
    }

    fn check_animations(&mut self, time: f64) {
        for anim in &mut self.animations {
            anim.replace_if_done(time, GemAnimation::still);
        }
        self.extra_animations
            .retain(|(_, _, anim)| !anim.done(time));
    }

    fn on_state_enter(&mut self, gmath: &GridMath, now: f64) {
        match &mut self.state {
            BoardState::Idle => {}
            BoardState::Moving(_) => {}
            BoardState::Refilling(refilling) => {
                let actions = GravityRefill::refill(&self.board.board, self.board.vertical_lines());
                for action in actions {
                    action.apply(&mut self.board.board, |_| random_gem());
                    let height = action.height();
                    let target = action.target();
                    let anim = GemAnimation::fall(target, height).animate(
                        gmath,
                        now,
                        1.0,
                        simple_easing::linear,
                    );
                    refilling.end = refilling.end.max(anim.end);
                    self.animations[target] = anim;
                }
            }
            BoardState::Matching(matching) => {
                let mut group_time = now;
                for group in &matching.groups {
                    let anim = GemAnimation::crack(group_time, 1.0, simple_easing::linear);
                    group_time = anim.end;
                    for &pos in group.cells() {
                        let gem = self.board.board[pos].clone();
                        self.extra_animations.push((pos, gem, anim.clone()));
                    }
                }
                matching.end = group_time;

                remove_matched(&mut self.board.board, &matching.groups, || {
                    SimpleGem(GemColor::Empty)
                });
            }
        }
    }
}

#[tweak_fn]
fn board_colors() -> (Color, Color) {
    (Color::rgb8(0x49, 0x2b, 0x1b), Color::rgb8(0x36, 0x20, 0x16))
}

impl Scene for BoardScene {
    fn update(&mut self, ui: Ui, now: f64) {
        debug_display("board.state", &self.state);

        let [width, height] = self.board.shape.as_array();

        let ui = ui.trim_to_aspect_ratio(width as f32 / height as f32);

        let gmath = GridMath::new(ui.rect, width, height, true);

        if let Some(next_state) = self.next_state.take() {
            self.state = next_state;
            self.on_state_enter(&gmath, now);
        }

        let interactive_ui = ui.clone().shrink(0.01);

        self.process_input(interactive_ui, &gmath, now);
        self.update_state(now);

        let grid_layer = ui.next_layer();

        let (main, secondary) = board_colors();
        let bg_z = ui.z;
        draw_rect(ui.rect.center().into(), ui.rect.into(), main, bg_z);
        for (i, gem) in self.board.board.iter().enumerate() {
            let ui = grid_layer.clone().with_rect(gmath.rect_at_index(i));
            // Draw grid BG
            if (i / width + i % width) % 2 == 0 {
                draw_rect(ui.rect.center().into(), ui.rect.into(), secondary, bg_z)
            };
            self.animations[i].update(gem, ui, &gmath, now);
        }

        for (pos, gem, animation) in &self.extra_animations {
            let ui = grid_layer.clone().with_rect(gmath.rect_at_index(*pos));
            animation.update(gem, ui, &gmath, now);
        }

        self.check_animations(now);
    }
}
