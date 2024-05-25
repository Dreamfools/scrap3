use crate::assets::sprite::SpriteId;
use crate::{collections, singletons};
use scrapcore_serialization::derive::registry;
use scrapcore_serialization::registry::SerializationRegistry;
use scrapcore_serialization::serialization::error::{
    DeserializationError, DeserializationErrorKind,
};
use thiserror::Error;

#[registry(error = "RegistryError", registry_name = "Registry")]
pub enum Registry {
    #[model(singleton)]
    Settings(singletons::settings::ModSettings),

    #[model(collection)]
    GemColor(collections::gem::GemColor),

    #[model(asset)]
    Textures(SpriteId),
}

#[derive(Debug, Clone, Error)]
pub enum RegistryError {
    #[error("{}", .0)]
    ParseColorError(csscolorparser::ParseColorError),
}

impl From<csscolorparser::ParseColorError> for RegistryError {
    fn from(err: csscolorparser::ParseColorError) -> Self {
        Self::ParseColorError(err)
    }
}

impl<Registry: SerializationRegistry<Error = RegistryError>> From<RegistryError>
    for DeserializationError<Registry>
{
    fn from(value: RegistryError) -> Self {
        DeserializationErrorKind::Custom(value).into()
    }
}
