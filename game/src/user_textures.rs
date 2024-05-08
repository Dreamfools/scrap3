use model::SpriteData;

/// Clears all yakui user textures, should be called at the start of each frame
pub fn clear_user_textures() {
    yakui_macroquad::user_textures(|tx| tx.clear());
}

pub trait SpriteDataExt {
    fn yakui_id(&self) -> yakui::TextureId;
}

impl SpriteDataExt for SpriteData {
    fn yakui_id(&self) -> yakui::TextureId {
        let id = yakui_macroquad::user_textures(|tx| {
            tx.push(self.texture_id());
            tx.len() - 1
        });

        yakui::TextureId::User(id as u64)
    }
}
