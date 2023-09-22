use slotmap::SlotMap;

use crate::types::{Phoneme, Language};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    pub phonemes: SlotMap<slotmap::DefaultKey, Phoneme>,
    pub inventory: Language
}

impl Default for State {
    fn default() -> Self {
        Self {
            phonemes: SlotMap::new(),
            inventory: Language::default()
        }
    }
}