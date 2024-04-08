use crate::{tweak, Name, Tweakable};
use egui::emath::Numeric;
use egui::Ui;

#[derive(Debug, Copy, Clone)]
pub struct SliderTweakable<T: Numeric + Send + Sync> {
    pub value: T,
    pub min: T,
    pub max: T,
}

impl<T: Numeric + Send + Sync> Tweakable for SliderTweakable<T> {
    fn draw(&mut self, ui: &mut Ui, name: &str) {
        ui.horizontal(|ui| {
            ui.label(name);
            ui.add(egui::Slider::new(&mut self.value, self.min..=self.max));
        });
    }
}

#[inline]
pub fn tweak_slider<T: Numeric + Send + Sync>(
    name: impl Into<Name>,
    value: T,
    min: T,
    max: T,
) -> T {
    tweak(name, SliderTweakable { value, min, max }).value
}

#[cfg(feature = "release-tweak")]
#[inline]
pub fn release_tweak_slider<T: Numeric + Send + Sync>(
    name: impl Into<Name>,
    value: T,
    min: T,
    max: T,
) -> T {
    crate::release_tweak(name, SliderTweakable { value, min, max }).value
}
