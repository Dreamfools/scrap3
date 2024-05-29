use yakui::{column, label};

use crate::{Name, Tweakable};

pub trait DrawUiFunc<T: Tweakable, const INLINE: bool> {
    fn draw(&self, name: &str, value: &mut T);
}

impl<T: Tweakable, F: Fn(&mut T)> DrawUiFunc<T, true> for F {
    fn draw(&self, name: &str, value: &mut T) {
        column(|| {
            label(name.to_string());
            self(value)
        });
    }
}

impl<T: Tweakable, F: Fn(&str, &mut T)> DrawUiFunc<T, false> for F {
    fn draw(&self, name: &str, value: &mut T) {
        self(name, value);
    }
}

// #[derive(Clone)]
#[derive(Clone)]
pub struct UiTweakable<T: Tweakable, const INLINE: bool> {
    pub value: T,
    pub draw: &'static (dyn DrawUiFunc<T, INLINE> + Send + Sync),
}

impl<T: Tweakable, const INLINE: bool> Tweakable for UiTweakable<T, INLINE> {
    fn draw(&mut self, name: &str) {
        self.draw.draw(name, &mut self.value);
    }
}

pub fn tweak_ui<T: Tweakable + Clone, const INLINE: bool>(
    name: impl Into<Name>,
    value: T,
    func: &'static (dyn DrawUiFunc<T, INLINE> + Send + Sync),
) -> T {
    #[cfg(debug_assertions)]
    return crate::tweak(name, UiTweakable { value, draw: func }).value;
    #[cfg(not(debug_assertions))]
    return value;
}

#[cfg(feature = "release-tweak")]
pub fn release_tweak_ui<T: Tweakable + Clone, const INLINE: bool>(
    name: impl Into<Name>,
    value: T,
    func: &'static (dyn DrawUiFunc<T, INLINE> + Send + Sync),
) -> T {
    crate::release_tweak(name, UiTweakable { value, draw: func }).value
}
