use crate::ui::UiTweakable;
use crate::{get_all_tweakables, Name};
use std::fmt::{Debug, Display};
use yakui::{button, label};
use yakui_widgets_plus::list_ext::row_shrink;

pub fn debug_display(name: impl Into<Name>, value: impl Debug) {
    let text = format!("{value:?}");
    display_text(name, text);
}

pub fn display_text(name: impl Into<Name>, text: impl Display) {
    let text = text.to_string();
    let mut tweakables = get_all_tweakables();
    let ui = UiTweakable {
        value: text,
        draw: &|name: &str, value: &mut String| {
            row_shrink(|| {
                label(name.to_string());
                if button(value.clone()).clicked {
                    eprintln!("{name} = {value}");
                }
            });
        },
    };
    tweakables.insert(name.into(), Box::new(ui));
}
