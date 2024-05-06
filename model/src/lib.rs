mod assets {
    pub mod color;
    pub mod sprite;
}

mod collections {
    pub mod gem;
}

mod singletons {
    pub mod settings;
}

pub mod registry;

mod loading;

// Exposing items

pub use assets::color::ColorData;
pub use assets::sprite::SpriteData;
pub use assets::sprite::SpriteId;

pub use collections::gem::GemColor;
pub use collections::gem::GemModifier;

pub use singletons::settings::ModSettings;

pub use loading::LoadedMod;

#[cfg(feature = "full")]
pub mod full {
    macro_rules! expose_mods {
        ($($name:tt),* $(,)?) => {
            $(
                pub mod $name {
                    pub use super::super::$name::*;
                }
            )*
        };
    }

    expose_mods!(assets, collections, singletons);
}
