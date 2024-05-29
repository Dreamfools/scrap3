use crate::{tweak, Name, Tweakable};
use duplicate::duplicate_item;
use std::fmt::Display;
use std::str::FromStr;
use yakui::label;
use yakui::widgets::Slider;
use yakui_widgets_plus::list_ext::row_shrink;

pub trait Numeric: 'static + Sync + Send + Clone + Display + FromStr {
    fn into_f64(self) -> f64;
    fn from_f64(num: f64) -> Self;
    fn step() -> Option<f64>;
}

#[derive(Debug, Copy, Clone)]
pub struct SliderTweakable<T: Numeric> {
    pub value: T,
    pub min: f64,
    pub max: f64,
}

impl<T: Numeric> SliderTweakable<T> {
    pub fn new(value: T, min: T, max: T) -> Self {
        Self {
            value,
            min: min.into_f64(),
            max: max.into_f64(),
        }
    }
}

impl<T: Numeric> Tweakable for SliderTweakable<T> {
    fn draw(&mut self, name: &str) {
        row_shrink(|| {
            label(name.to_string());
            let num = self.value.clone().into_f64();
            let mut slider = Slider::new(num, self.min, self.max);
            slider.step = T::step();
            if let Some(value) = slider.show().value {
                self.value = T::from_f64(value);
            }
        });
    }
}

#[inline]
pub fn tweak_slider<T: Numeric>(name: impl Into<Name>, value: T, min: T, max: T) -> T {
    #[cfg(debug_assertions)]
    return tweak(name, SliderTweakable::new(value, min, max)).value;
    #[cfg(not(debug_assertions))]
    return value;
}

#[cfg(feature = "release-tweak")]
#[inline]
pub fn release_tweak_slider<T: Numeric>(name: impl Into<Name>, value: T, min: T, max: T) -> T {
    crate::release_tweak(name, SliderTweakable::new(value, min, max)).value
}

#[duplicate_item(
num_type;
    [ u8 ];
    [ u16 ];
    [ u32 ];
    [ u64 ];
    [ u128 ];
    [ usize ];
    [ i8 ];
    [ i16 ];
    [ i32 ];
    [ i64 ];
    [ i128 ];
    [ isize ];
)]
impl Numeric for num_type {
    fn into_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(num: f64) -> Self {
        num as Self
    }

    fn step() -> Option<f64> {
        Some(1.0)
    }
}

impl Numeric for f32 {
    fn into_f64(self) -> f64 {
        self as f64
    }

    fn from_f64(num: f64) -> Self {
        num as Self
    }

    fn step() -> Option<f64> {
        None
    }
}

impl Numeric for f64 {
    fn into_f64(self) -> f64 {
        self
    }

    fn from_f64(num: f64) -> Self {
        num
    }

    fn step() -> Option<f64> {
        None
    }
}
