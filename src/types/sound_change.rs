use slotmap::DefaultKey;

use crate::types::PhonemeSelector;
use crate::types::category::{Articulation, Region, Voicing, Constriction, Place, Rounding};

#[derive(serde::Deserialize, serde::Serialize)]
pub enum SoundChangeContext {
    Consonant(PhonemeSelector<Articulation, Region, Voicing>),
    Vowel(PhonemeSelector<Constriction, Place, Rounding>),
    Multiple(Vec<SoundChangeContext>),
    Unrestricted
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SoundChange {
    pub src: DefaultKey,
    pub dst: DefaultKey,
    pub context: (SoundChangeContext, SoundChangeContext)
}