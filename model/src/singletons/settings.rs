use crate::SpriteData;
use scrapcore_serialization::derive::DatabaseModel;

#[derive(Debug, DatabaseModel)]
pub struct ModSettings {
    pub logo: SpriteData,
}
