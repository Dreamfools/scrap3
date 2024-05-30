use yakui::paint::PaintRect;
use yakui::util::widget;
use yakui::widget::{LayoutContext, PaintContext, Widget};
use yakui::{Color, Constraints, Rect, Response, TextureId, Vec2};

use match3::{BoardGem, Shape};
use math::board::{CellIndex, GridMath, GridSize};
use math::glam::vec2;
use math::glamour::{point, size2};
use model::scrapcore_serialization::registry::index::RegistryIndex;
use model::{GemColorModel, Registry};

use crate::state::combat::anim::{BoardView, BoardViewRect, GemDrawInfo};
use crate::state::combat::gem::{Gem, GemColor};
use crate::state::combat::{CombatInput, CombatState};
use crate::user_textures::SpriteDataExt;

pub fn board(state: &CombatState, registry: &Registry, time: f64) -> Response<Vec<CombatInput>> {
    widget::<BoardWidget>((state, registry, time))
}

#[derive(Debug, Clone, Default)]
pub struct BoardWidget {
    board_size: GridSize,
    draw_calls: Vec<(TextureId, GemDrawInfo)>,
}

impl BoardWidget {
    pub fn draw(&mut self, registry: &Registry, gem: &Gem, draw: GemDrawInfo) {
        let GemColor::Color(color) = gem.color() else {
            // Skip drawing empty gems
            return;
        };

        if draw.opacity == 0.0 {
            // Skip drawing fully transparent gems
            return;
        }

        let color = RegistryIndex::<GemColorModel>::get(&color, registry);
        let id = color.sprite.yakui_id();
        self.draw_calls.push((id, draw));
    }
}

impl Widget for BoardWidget {
    type Props<'a> = (&'a CombatState, &'a Registry, f64);
    type Response = Vec<CombatInput>;

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        let (state, registry, time) = props;
        let shape = &state.board.board.shape;
        let [width, height] = shape.as_array();
        self.board_size = size2!(width as u64, height as u64);

        self.draw_calls.clear();

        let gmath = GridMath::new(
            BoardViewRect::new(point!(0.0, 0.0), size2!(1.0, 1.0)),
            width,
            height,
            false,
        );

        for (i, gem) in state.board.board.board.iter().enumerate() {
            // Draw grid BG
            if (i / width + i % width) % 2 == 0 {
                // draw_rect(ui.rect.center().into(), ui.rect.into(), secondary, bg_z)
            };
            self.draw(
                registry,
                gem,
                state.animations[i].update(CellIndex(i), &gmath, time),
            );
        }

        for (pos, gem, animation) in &state.extra_animations {
            self.draw(registry, gem, animation.update(*pos, &gmath, time));
        }

        self.draw_calls.sort_by_key(|(_, draw)| draw.layer);

        vec![]
    }

    fn layout(&self, _ctx: LayoutContext<'_>, constraints: Constraints) -> Vec2 {
        let ar = self.board_size.height as f32 / self.board_size.width as f32;

        let width = constraints.max.x;
        let height = width * ar;

        let constrained_height = constraints.constrain_height(height);
        if constrained_height == height {
            vec2(width, height)
        } else {
            vec2(constrained_height / ar, constrained_height)
        }
    }

    fn paint(&self, ctx: PaintContext<'_>) {
        fn view_to_screen(view: BoardViewRect, board_rect: Rect) -> Rect {
            Rect::from_pos_size(
                Vec2::from(view.origin) * board_rect.size() + board_rect.pos(),
                Vec2::from(view.size) * board_rect.size(),
            )
        }
        #[inline_tweak::tweak_fn]
        fn board_colors() -> (Color, Color) {
            (Color::rgb(0x49, 0x2b, 0x1b), Color::rgb(0x36, 0x20, 0x16))
        }

        let layout_node = ctx.layout.get(ctx.dom.current()).unwrap();
        let board_rect = layout_node.rect;

        // Drawing grid
        let (primary, secondary) = board_colors();

        let mut paint = PaintRect::new(board_rect);
        paint.color = primary;
        paint.add(ctx.paint);

        let gmath = GridMath::<BoardView, 1000>::new(
            BoardViewRect::new(board_rect.pos(), board_rect.size()),
            self.board_size.width as usize,
            self.board_size.height as usize,
            false,
        );

        for y in 0..self.board_size.height {
            for x in 0..self.board_size.width {
                if (x + y) % 2 == 0 {
                    continue;
                }
                let i = x + y * self.board_size.width;
                let rect = gmath.rect_at_index(CellIndex(i as usize));

                let mut paint =
                    PaintRect::new(Rect::from_pos_size(rect.origin.into(), rect.size.into()));
                paint.color = secondary;
                paint.add(ctx.paint);
            }
        }

        // Gems
        for (id, draw) in &self.draw_calls {
            let rect = view_to_screen(draw.rect, board_rect);

            let mut paint = PaintRect::new(rect);
            paint.texture = Some((*id, Rect::ONE));
            paint.add(ctx.paint);
        }
    }
}
