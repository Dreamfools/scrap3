mod assets {
    pub mod color;
    pub mod sprite;
}

mod collections {
    pub mod actions;
    pub mod gem;
}

mod singletons {
    pub mod settings;
}

mod registry;

mod loading;

// Exposing items
pub use registry::id::*;
pub use registry::{Registry, RegistryAssetKind, RegistryError, RegistryItem, RegistryItemKind};

pub use assets::color::ColorData;
pub use assets::sprite::{SpriteData, SpriteId};

pub use collections::actions::{ActionEffect, ActionOrChain, ChainType, CombatActionChain};
pub use collections::gem::{GemColorModel, GemModifierModel};

pub use singletons::settings::ModSettings;

pub use loading::LoadedMod;

// Re-exports
pub use assets_manager;
pub use scrapcore_serialization;

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
