use crate::ui::UiTweakable;
use crate::{get_all_tweakables, Name};
use egui::Ui;
use std::fmt::{Debug, Display};

pub fn debug_display(name: impl Into<Name>, value: impl Debug) {
    let text = format!("{value:?}");
    display_text(name, text);
}

pub fn display_text(name: impl Into<Name>, text: impl Display) {
    let text = text.to_string();
    let mut tweakables = get_all_tweakables();
    let ui = UiTweakable {
        value: text,
        draw: &|ui: &mut Ui, name: &str, value: &mut String| {
            ui.horizontal(|ui| {
                ui.label(name);
                if ui.button(value.clone()).clicked() {
                    eprintln!("{name} = {value}");
                }
            });
        },
    };
    tweakables.insert(name.into(), Box::new(ui));
}
