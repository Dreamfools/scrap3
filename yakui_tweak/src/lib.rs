use downcast_rs::{impl_downcast, Downcast};
use std::any::Any;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

pub mod display;
pub mod editor;
pub mod slider;
pub mod tweakables;
pub mod ui;

pub trait Tweakable: Any + Sync + Send + Downcast {
    fn draw(&mut self, name: &str);
}

impl_downcast!(Tweakable);

type Name = Cow<'static, str>;

pub type TweakablesMap = BTreeMap<Name, Box<dyn Tweakable>>;

static TWEAKABLES: OnceLock<Mutex<TweakablesMap>> = OnceLock::new();

pub fn get_all_tweakables() -> MutexGuard<'static, TweakablesMap> {
    TWEAKABLES
        .get_or_init(|| Mutex::new(TweakablesMap::default()))
        .lock()
        .unwrap()
}

#[inline]
fn tweak_internal<T: Tweakable + Clone>(name: impl Into<Name>, value: T) -> T {
    let mut tweakables = get_all_tweakables();

    let value = tweakables
        .entry(name.into())
        .or_insert_with(|| Box::new(value.clone()));

    value
        .downcast_ref::<T>()
        .expect("Should not have tweakable name collisions")
        .clone()
}

#[inline]
pub fn tweak<T: Tweakable + Clone>(name: impl Into<Name>, value: T) -> T {
    #[cfg(debug_assertions)]
    return tweak_internal(name, value);
    #[cfg(not(debug_assertions))]
    return value;
}

#[cfg(feature = "release-tweak")]
#[inline]
pub fn release_tweak<T: Tweakable + Clone>(name: impl Into<Name>, value: T) -> T {
    tweak_internal(name, value)
}
