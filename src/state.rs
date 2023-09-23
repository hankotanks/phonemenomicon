use slotmap::SlotMap;

use crate::types::{Phoneme, Language, Alphabet, PhonemeQuality};
use crate::types::category;

#[allow(unused_imports)]
use crate::types::{add_symbol_to_alphabet, CONSONANT, VOWEL};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    pub phonemes: SlotMap<slotmap::DefaultKey, Phoneme>,
    pub inventory: Language,
    pub ipa: Language,
}

impl Default for State {
    fn default() -> Self {
        // TODO: This must be cleaned up later
        let mut phonemes = SlotMap::new();
        let ipa = init_ipa(&mut phonemes);

        Self {
            phonemes,
            inventory: Language::default(),
            ipa
        }
    }
}

// TODO: Finish implementing the IPA
fn init_ipa(phonemes: &mut SlotMap<slotmap::DefaultKey, Phoneme>) -> Language {
    let mut consonants = Alphabet::new();

    {
        use category::Articulation::{self, *};
        use category::Region::{self, *};
        use category::Voicing::{self, *};

        let mut add = |
            symbol: &'static str, 
            quality: PhonemeQuality<Articulation, Region, Voicing>| {
            add_symbol_to_alphabet(phonemes, &mut consonants, symbol, CONSONANT, quality);
        };

        add("p", (Plosive, Bilabial, Unvoiced).into());
        add("b", (Plosive, Bilabial, Voiced).into());
        add("t", (Plosive, &[Dental, Alveolar][..], Unvoiced).into());
        add("d", (Plosive, &[Alveolar, PostAlveolar][..], Voiced).into());
        add("ʈ", (Plosive, Retroflex, Unvoiced).into());
        add("ɖ", (Plosive, Retroflex, Voiced).into());
        add("c", (Plosive, Palatal, Unvoiced).into());
        add("ɟ", (Plosive, Palatal, Voiced).into());
        add("k", (Plosive, Velar, Unvoiced).into());
        add("g", (Plosive, Velar, Voiced).into());
        add("q", (Plosive, Uvular, Unvoiced).into());
        add("ɢ", (Plosive, Uvular, Voiced).into());
        add("ʔ", (Plosive, Glottal, Unvoiced).into());
    }

    let mut vowels = Alphabet::new();

    {
        use category::Constriction::{self, *};
        use category::Place::{self, *};
        use category::Rounding::{self, *};

        let mut add = |
            symbol: &'static str, 
            quality: PhonemeQuality<Constriction, Place, Rounding>| {
            add_symbol_to_alphabet(phonemes, &mut vowels, symbol, VOWEL, quality);
        };

        add("i", (Close, Front, Unrounded).into());
    }
    

    Language { consonants, vowels }
}