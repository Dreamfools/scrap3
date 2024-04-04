use crate::{Name, Tweakable};
use egui::Ui;

pub trait DrawUiFunc<T: Tweakable, const INLINE: bool> {
    fn draw(&self, ui: &mut Ui, name: &str, value: &mut T);
}

impl<T: Tweakable, F: Fn(&mut Ui, &mut T)> DrawUiFunc<T, true> for F {
    fn draw(&self, ui: &mut Ui, name: &str, value: &mut T) {
        ui.horizontal(|ui| {
            ui.label(name);
            self(ui, value);
        });
    }
}

impl<T: Tweakable, F: Fn(&mut Ui, &str, &mut T)> DrawUiFunc<T, false> for F {
    fn draw(&self, ui: &mut Ui, name: &str, value: &mut T) {
        self(ui, name, value);
    }
}

// #[derive(Clone)]
#[derive(Clone)]
pub struct UiTweakable<T: Tweakable, const INLINE: bool> {
    pub value: T,
    pub draw: &'static (dyn DrawUiFunc<T, INLINE> + Send + Sync),
}

impl<T: Tweakable, const INLINE: bool> Tweakable for UiTweakable<T, INLINE> {
    fn draw(&mut self, ui: &mut Ui, name: &str) {
        self.draw.draw(ui, name, &mut self.value);
    }
}

pub fn tweak_ui<T: Tweakable + Clone, const INLINE: bool>(
    name: impl Into<Name>,
    value: T,
    func: &'static (dyn DrawUiFunc<T, INLINE> + Send + Sync),
) -> T {
    crate::tweak(name, UiTweakable { value, draw: func }).value
}

#[cfg(feature = "release-tweak")]
pub fn release_tweak_ui<T: Tweakable + Clone, const INLINE: bool>(
    name: impl Into<Name>,
    value: T,
    func: &'static (dyn DrawUiFunc<T, INLINE> + Send + Sync),
) -> T {
    crate::release_tweak(name, UiTweakable { value, draw: func }).value
}
