use crate::state::combat::{CombatInput, CombatState};
use match3::Shape;
use math::board::GridSize;
use math::glam::vec2;
use math::glamour::size2;
use yakui::widget::{LayoutContext, Widget};
use yakui::{Constraints, Vec2};

pub mod anim;

#[derive(Debug, Clone, Default)]
pub struct BoardWidget {
    board_size: GridSize,
}

impl Widget for BoardWidget {
    type Props<'a> = &'a CombatState;
    type Response = Vec<CombatInput>;

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        let [width, height] = props.board.board.shape.as_array();
        self.board_size = size2!(width as u64, height as u64);
        todo!()
    }

    fn layout(&self, _ctx: LayoutContext<'_>, constraints: Constraints) -> Vec2 {
        let ar = self.board_size.width as f32 / self.board_size.height as f32;

        let width = constraints.max.x;
        let height = width * ar;

        let constrained_height = constraints.constrain_height(height);
        if constrained_height == height {
            vec2(width, height)
        } else {
            vec2(constrained_height / ar, constrained_height)
        }
    }
}
