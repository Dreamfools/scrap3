use crate::board::board::{random_board, BoardState, GemBoard};
use crate::board::gem::draw_gem;
use crate::ui::{GridMath, Ui};
use crate::Scene;
use comfy::{draw_circle, draw_rect, is_mouse_button_down, mouse_world, Color, MouseButton, Vec2};
use inline_tweak::{tweak, tweak_fn};
use match3::rect_board::GridMoveStrategy;
use match3::Shape;

pub mod board;
pub mod board_anim;
pub mod gem;

impl BoardScene {
    pub fn new(width: usize, height: usize) -> Self {
        let (board, state) = random_board(width, height);
        Self {
            state,
            board,
            held_gem: None,
        }
    }
}
#[derive(Debug)]
pub struct BoardScene {
    state: BoardState,
    board: GemBoard,
    held_gem: Option<(usize, Vec2)>,
}

impl BoardScene {
    fn process_input(&mut self, interactive_ui: Ui, gmath: &GridMath) {
        if matches!(self.state, BoardState::Idle | BoardState::Matching) {
            return;
        }

        let mouse_pos = mouse_world();
        let trapped_pos = interactive_ui.clamp_pos(mouse_pos);

        if is_mouse_button_down(MouseButton::Left) {
            let pressed = gmath.pos_to_index(trapped_pos);
            let pressed_cell = gmath.rect_at_index(pressed);
            if let Some((held, touch)) = &mut self.held_gem {
                *touch = trapped_pos;

                let dist2 = trapped_pos.distance_squared(pressed_cell.center().into());
                let hit_radius = pressed_cell.width() / 1.8;

                if tweak!(false) {
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
        let grid = ui.next_layer().grid(width, height);

        let interactive_ui = ui.clone().shrink(0.01);

        self.process_input(interactive_ui, &gmath);

        let (main, secondary) = board_colors();
        let bg_z = ui.z;
        draw_rect(ui.rect.center().into(), ui.rect.into(), main, bg_z);
        for (i, (ui, gem)) in grid.into_iter().zip(self.board.board.iter()).enumerate() {
            // Draw grid BG
            if (i / width + i % width) % 2 == 0 {
                draw_rect(ui.rect.center().into(), ui.rect.into(), secondary, bg_z)
            }

            if self.held_gem.is_some_and(|(cell, _)| cell == i) {
                continue;
            }

            let ui = ui.shrink(0.1);
            draw_gem(gem, ui, 1.0);
        }

        if let Some((pos, touch)) = self.held_gem {
            let gem_ui = ui
                .next_layer()
                .next_layer()
                .with_rect(gmath.unit_cell())
                .recenter(touch)
                .shrink(0.1);

            draw_gem(&self.board.board[pos], gem_ui, 0.5)
        }
    }
}
