use slotmap::SlotMap;

use crate::types::{Phoneme, PhonemeQuality, Alphabet};
use crate::types::category::{Outer, Inner, Pair, Articulation, Region, Voicing, Constriction, Place, Rounding};

pub type Modifier<A, B, C> = (PhonemeQuality<A, B, C>, String, String);

pub struct Diacritics<A: Outer<B, C>, B: Inner<C>, C: Pair> {
    pub contents: Vec<Modifier<A, B, C>>,
    pub change_state: fn(&mut Phoneme, &str),
    pub prepend_blank: bool,
    pub submenu_visible: bool
}

pub fn modifiers_consonants(
    phonemes: &SlotMap<slotmap::DefaultKey, Phoneme>,
    ipa: &Alphabet<Articulation, Region, Voicing>,
    quality: PhonemeQuality<Articulation, Region, Voicing>
) -> impl Iterator<Item = Diacritics<Articulation, Region, Voicing>> {

    let mut diacritics = Vec::new();

    if quality.0.contains(&Articulation::Plosive) {
        let mut contents = Vec::new();

        let query: PhonemeQuality<Articulation, Region, Voicing> = (
            &[Articulation::Fricative, Articulation::LatFricative][..],
            &[Region::Alveolar][..],
            quality.2[0]
        ).into();

        for fricative in ipa.select_phonemes(query) {
            let phoneme = phonemes[fricative].clone();

            let PhonemeQuality(_, regions, ..) = ipa.get_quality(fricative).unwrap();

            let mut description = String::new();
            for (idx, region) in regions.iter().enumerate() {
                let content = if idx == 0 {
                    format!("{}", region)
                } else {
                    format!("/{}", region)
                };

                description.push_str(content.as_str());
            }

            contents.push((
                PhonemeQuality::<Articulation, Region, Voicing>::blank(), 
                format!("{}", phoneme), 
                description
            ));

        }

        diacritics.push(Diacritics { 
            contents, 
            change_state: |phoneme: &mut Phoneme, symbol: &str| 
                phoneme.phone.affricate(symbol), 
            prepend_blank: false, 
            submenu_visible: true
        });
    }

    diacritics.into_iter()
}

#[allow(dead_code, unused_variables)]
pub fn modifiers_vowels(
    phonemes: &SlotMap<slotmap::DefaultKey, Phoneme>,
    ipa: &Alphabet<Constriction, Place, Rounding>,
    quality: PhonemeQuality<Constriction, Place, Rounding>
) -> impl Iterator<Item = Diacritics<Constriction, Place, Rounding>> {
    let diacritics = Vec::new();

    diacritics.into_iter()
}