use yakui::geometry::{Color, Constraints, Rect, Vec2};
use yakui::paint::PaintRect;
use yakui::widget::{LayoutContext, PaintContext, Widget};
use yakui::Response;

/// A vertical divider line. Will take up the whole width of the parent.
///
/// Responds with [DividerResponse].
#[derive(Debug)]
#[non_exhaustive]
pub struct VerticalDivider {
    /// The color of the divider.
    pub color: Color,
    /// The thickness of the divider.
    pub thickness: f32,
    /// The width of the divider.
    /// How much horizontal space it takes up.
    pub width: f32,
    /// The indent of the divider from the top.
    pub indent: f32,
    /// The indent of the divider from the bottom.
    pub end_indent: f32,
}

impl VerticalDivider {
    pub fn new(color: Color, width: f32, thickness: f32) -> Self {
        Self {
            color,
            thickness,
            width,
            indent: 0.0,
            end_indent: 0.0,
        }
    }

    pub fn show(self) -> Response<DividerResponse> {
        yakui::util::widget::<DividerWidget>(self)
    }
}

#[derive(Debug)]
pub struct DividerWidget {
    props: VerticalDivider,
}

pub type DividerResponse = ();

impl Widget for DividerWidget {
    type Props<'a> = VerticalDivider;
    type Response = DividerResponse;

    fn new() -> Self {
        Self {
            props: VerticalDivider::new(Color::WHITE, 0.0, 0.0),
        }
    }

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, _ctx: LayoutContext<'_>, input: Constraints) -> Vec2 {
        Vec2::new(
            self.props.width.clamp(input.min.x, input.max.x),
            input.min.y,
        )
    }

    fn paint(&self, ctx: PaintContext<'_>) {
        // We get the parent's height during the paint phase because
        // using constraints.max.y is often useless as it is often infinite.

        let id = ctx.dom.current();
        let Some(parent) = ctx.dom.get(id).unwrap().parent else {
            return;
        };
        let line_height = ctx.layout.get(parent).unwrap().rect.size().y;

        let outer_rect = ctx.layout.get(id).unwrap().rect;

        let line_pos = outer_rect.pos()
            + Vec2::new(
                (outer_rect.size().x - self.props.thickness) / 2.0,
                self.props.indent,
            );
        let line_size = Vec2::new(
            self.props.thickness,
            line_height - self.props.indent - self.props.end_indent,
        );

        let mut line_rect = PaintRect::new(Rect::from_pos_size(line_pos, line_size));
        line_rect.color = self.props.color;
        line_rect.add(ctx.paint);
    }
}
