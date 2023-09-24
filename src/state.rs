use slotmap::SlotMap;

use crate::pane::LanguagePaneRole;
use crate::types::{Phoneme, Language, Alphabet, PhonemeQuality, Phone};
use crate::types::category;

#[allow(unused_imports)]
use crate::types::{add_symbol_to_alphabet, CONSONANT, VOWEL};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    pub phonemes: SlotMap<slotmap::DefaultKey, Phoneme>,
    pub inventory: Language,
    pub ipa: Language,
    pub invalid: Phoneme,
    pub space: Phoneme,
    
    #[serde(skip)]
    pub phoneme_buffer: Option<(slotmap::DefaultKey, LanguagePaneRole)>
}

impl Default for State {
    fn default() -> Self {
        let mut phonemes = SlotMap::new();

        // We can't initialize it in the struct form below
        // Because `init_ipa` must insert elements into `phonemes`
        let ipa = init_ipa(&mut phonemes);

        Self {
            phonemes,
            inventory: Language::default(),
            ipa,
            invalid: Phoneme::new("0", Phone::consonant()),
            space: Phoneme::new(" ", Phone::consonant()),
            phoneme_buffer: None
        }
    }
}

fn init_ipa(phonemes: &mut SlotMap<slotmap::DefaultKey, Phoneme>) -> Language {
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
        add("y", (Close, Front, Rounded).into());
        add("ɨ", (Close, Central, Unrounded).into());
        add("ʉ", (Close, Central, Rounded).into());
        add("ɯ", (Close, Back, Unrounded).into());
        add("u", (Close, Back, Rounded).into());
        add("ɪ", (CloseNear, Front, Unrounded).into());
        add("ʏ", (CloseNear, Front, Rounded).into());
        add("ʊ", (CloseNear, Back, Rounded).into());
        add("e", (CloseMid, Front, Unrounded).into());
        add("ø", (CloseMid, Front, Rounded).into());
        add("ɘ", (CloseMid, Central, Unrounded).into());
        add("ɵ", (CloseMid, Central, Rounded).into());
        add("ɤ", (CloseMid, Back, Unrounded).into());
        add("o", (CloseMid, Back, Rounded).into());
        add("ə", (Mid, Central, &[Unrounded, Rounded][..]).into());
        add("ɛ", (OpenMid, Front, Unrounded).into());
        add("œ", (OpenMid, Front, Rounded).into());
        add("ɜ", (OpenMid, Central, Unrounded).into());
        add("ɞ", (OpenMid, Central, Rounded).into());
        add("ʌ", (OpenMid, Back, Unrounded).into());
        add("ɔ", (OpenMid, Back, Rounded).into());
        add("æ", (OpenNear, Front, Unrounded).into());
        add("ɐ", (OpenNear, Central, &[Unrounded, Rounded][..]).into());
        add("a", (Open, Front, Unrounded).into());
        add("ɶ", (Open, Front, Rounded).into());
        add("ɑ", (Open, Back, Unrounded).into());
        add("ɒ", (Open, Back, Rounded).into());
    }

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

        add("p", (Plosive, Bilabial, Voiceless).into());
        add("b", (Plosive, Bilabial, Voiced).into());
        add("t", (Plosive, &[Dental, Alveolar][..], Voiceless).into());
        add("d", (Plosive, &[Alveolar, Post][..], Voiced).into());
        add("ʈ", (Plosive, Retroflex, Voiceless).into());
        add("ɖ", (Plosive, Retroflex, Voiced).into());
        add("c", (Plosive, Palatal, Voiceless).into());
        add("ɟ", (Plosive, Palatal, Voiced).into());
        add("k", (Plosive, Velar, Voiceless).into());
        add("g", (Plosive, Velar, Voiced).into());
        add("q", (Plosive, Uvular, Voiceless).into());
        add("ɢ", (Plosive, Uvular, Voiced).into());
        add("ʔ", (Plosive, Glottal, Voiceless).into());
        add("m", (Nasal, Bilabial, Voiced).into());
        add("ɱ", (Nasal, Labiodental, Voiced).into());
        add("n", (Nasal, &[Alveolar, Post][..], Voiced).into());
        add("ɳ", (Nasal, Retroflex, Voiced).into());
        add("ɲ", (Nasal, Palatal, Voiced).into());
        add("ŋ", (Nasal, Velar, Voiced).into());
        add("ɴ", (Nasal, Uvular, Voiced).into());
        add("ʙ", (Trill, Bilabial, Voiced).into());
        add("r", (Trill, &[Alveolar, Post][..], Voiced).into());
        add("ʀ", (Trill, Uvular, Voiced).into());
        add("ⱱ", (Flap, Labiodental, Voiced).into());
        add("ɾ", (Flap, &[Alveolar, Post][..], Voiced).into());
        add("ɽ", (Flap, Retroflex, Voiced).into());

        add("ɸ", (Fricative, Bilabial, Voiceless).into());
        add("β", (Fricative, Bilabial, Voiced).into());
        add("f", (Fricative, Labiodental, Voiceless).into());
        add("v", (Fricative, Labiodental, Voiced).into());
        add("θ", (Fricative, Dental, Voiceless).into());
        add("ð", (Fricative, Dental, Voiced).into());
        add("s", (Fricative, Alveolar, Voiceless).into());
        add("z", (Fricative, Alveolar, Voiced).into());
        add("ʃ", (Fricative, Post, Voiceless).into());
        add("ʒ", (Fricative, Post, Voiced).into());
        add("ʂ", (Fricative, Retroflex, Voiceless).into());
        add("ʐ", (Fricative, Retroflex, Voiced).into());
        add("ç", (Fricative, Palatal, Voiceless).into());
        add("ʝ", (Fricative, Palatal, Voiced).into());
        add("x", (Fricative, Velar, Voiceless).into());
        add("ɣ", (Fricative, Velar, Voiced).into());
        add("χ", (Fricative, Uvular, Voiceless).into());
        add("ʁ", (Fricative, Uvular, Voiced).into());
        add("ħ", (Fricative, Pharyngeal, Voiceless).into());
        add("ʕ", (Fricative, Pharyngeal, Voiced).into());
        add("h", (Fricative, Glottal, Voiceless).into());
        add("ɦ", (Fricative, Glottal, Voiced).into());

        add("ɬ", (LateralFricative, &[Dental, Alveolar][..], Voiceless).into());
        add("ɮ", (LateralFricative, &[Alveolar, Post][..], Voiced).into());
        add("ʋ", (Approximant, Labiodental, Voiced).into());
        add("ɹ", (Approximant, &[Alveolar, Post][..], Voiced).into());
        add("ɻ", (Approximant, Retroflex, Voiced).into());
        add("j", (Approximant, Palatal, Voiced).into());
        add("ɰ", (Approximant, Velar, Voiced).into());

        add("l", (LateralApproximant, &[Alveolar, Post][..], Voiced).into());
        add("ɭ", (LateralApproximant, Retroflex, Voiced).into());
        add("ʎ", (LateralApproximant, Palatal, Voiced).into());
        add("ʟ", (LateralApproximant, Velar, Voiced).into());
    }    

    Language { vowels, consonants }
}