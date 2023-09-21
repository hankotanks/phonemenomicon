use slotmap::SlotMap;

use crate::types::Phoneme;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    pub phonemes: SlotMap<slotmap::DefaultKey, Phoneme>
}

impl Default for State {
    fn default() -> Self {
        Self {
            phonemes: SlotMap::new()
        }
    }
}