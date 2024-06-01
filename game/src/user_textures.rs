use ahash::AHashMap;
use send_wrapper::SendWrapper;
use std::cell::{RefCell, RefMut};
use std::sync::OnceLock;

static MAPPING: OnceLock<
    SendWrapper<RefCell<AHashMap<macroquad::miniquad::TextureId, yakui::TextureId>>>,
> = OnceLock::new();

fn mappings() -> RefMut<'static, AHashMap<macroquad::miniquad::TextureId, yakui::TextureId>> {
    MAPPING
        .get_or_init(|| SendWrapper::new(RefCell::new(AHashMap::new())))
        .borrow_mut()
}

/// Clears all yakui user textures, should be called at the start of each frame
pub fn clear_user_textures() {
    mappings().clear();
    yakui_macroquad::user_textures(|tx| tx.clear());
}

// pub trait SpriteDataExt {
//     fn yakui_id(&self) -> yakui::TextureId;
// }

// impl SpriteDataExt for SpriteData {
//     fn yakui_id(&self) -> yakui::TextureId {
//         *mappings().entry(self.texture_id()).or_insert_with(|| {
//             let id = yakui_macroquad::user_textures(|tx| {
//                 tx.push(self.texture_id());
//                 tx.len() - 1
//             });
//
//             yakui::TextureId::User(id as u64)
//         })
//     }
// }
