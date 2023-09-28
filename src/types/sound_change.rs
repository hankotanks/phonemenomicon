use slotmap::DefaultKey;

use crate::types::PhonemeSelector;
use crate::types::category::{Articulation, Region, Voicing, Constriction, Place, Rounding};

pub enum SoundChangeContext {
    Consonant(PhonemeSelector<Articulation, Region, Voicing>),
    Vowel(PhonemeSelector<Constriction, Place, Rounding>),
    Multiple(Vec<SoundChangeContext>)
}

pub struct SoundChange {
    pub src: DefaultKey,
    pub dst: DefaultKey,
    pub context: (SoundChangeContext, SoundChangeContext)
}