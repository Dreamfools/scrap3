use crate::list_ext::{column_shrink, row_shrink};
use crate::vertical_divider::VerticalDivider;
use yakui::widgets::Button;
use yakui::{use_state, Color};

pub fn collapsing(header: impl Into<String>, children: impl FnOnce()) {
    let shown = use_state(|| false);

    column_shrink(|| {
        let arrow = if *shown.borrow() {
            inline_tweak::tweak!("v")
        } else {
            inline_tweak::tweak!(">")
        };
        let mut b = Button::unstyled(format!("{} {}", arrow, header.into()));
        let hover_color = b.hover_style.text.color.adjust(0.75);
        b.hover_style.text.color = hover_color;
        let button_res = b.show();
        if button_res.clicked {
            let b = *shown.borrow();
            *shown.borrow_mut() = !b;
        }

        if *shown.borrow() {
            row_shrink(|| {
                VerticalDivider::new(
                    if button_res.hovering {
                        hover_color
                    } else {
                        Color::WHITE
                    },
                    inline_tweak::tweak!(6.5),
                    1.0,
                )
                .show();

                column_shrink(|| {
                    children();
                });
            });
        }
    });
}
