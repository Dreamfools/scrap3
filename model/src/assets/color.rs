use macroquad::color::Color;
use schemars::JsonSchema;
use scrapcore_serialization::registry::PartialRegistry;
use scrapcore_serialization::serialization::error::{
    DeserializationError, DeserializationErrorKind,
};
use scrapcore_serialization::serialization::{DeserializeModel, SerializationFallback};
use serde::{Deserialize, Serialize};

/// Color data that can be deserialized from various formats:
/// - CSS color notation string, parsed via [csscolorparser]
/// - Array in `[r, g, b, a]` notation. Example: `[1.0, 1.0, 0.0, 0.5]` for half opacity yellow
/// - Object in `{r, g, b, a}` notation. Example: `{r: 1.0, g: 1.0, b: 0.0, a: 0.5}` for half opacity yellow
#[derive(Debug)]
pub struct ColorData(pub Color);

// <================================>
// <===== Deserialization code =====>
// <================================>

impl From<ColorData> for Color {
    fn from(data: ColorData) -> Color {
        data.0
    }
}

impl SerializationFallback for ColorData {
    type Fallback = ColorDataSerialized;
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ColorDataSerialized {
    String(String),
    Array([f32; 4]),
    Obj { r: f32, g: f32, b: f32, a: f32 },
}

impl<Registry: PartialRegistry> DeserializeModel<ColorData, Registry> for ColorDataSerialized
where
    Registry::Error: From<csscolorparser::ParseColorError>,
{
    fn deserialize(
        self,
        _registry: &mut Registry,
    ) -> Result<ColorData, DeserializationError<Registry>> {
        let color = match self {
            ColorDataSerialized::String(string) => {
                let data = csscolorparser::parse(&string)
                    .map_err(|e| DeserializationErrorKind::Custom(Registry::Error::from(e)))?;
                Color::new(data.r as f32, data.g as f32, data.b as f32, data.a as f32)
            }
            ColorDataSerialized::Array(array) => Color::from(array),
            ColorDataSerialized::Obj { r, g, b, a } => Color::new(r, g, b, a),
        };

        Ok(ColorData(color))
    }
}
