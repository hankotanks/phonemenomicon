use std::collections::HashSet;

use enum_iterator::all;
use slotmap::SlotMap;

use crate::types::{Phoneme, PhonemeQuality, Alphabet, Phone};
use crate::types::category::{Outer, Inner, Pair, Category};
use crate::types::category::{Articulation, Region, Voicing, Constriction, Place, Rounding};

pub type Modifier<A, B, C> = (PhonemeQuality<A, B, C>, String, String);

pub struct Diacritics<A: Outer<B, C>, B: Inner<C>, C: Pair> {
    pub category: &'static str,
    pub contents: Vec<Modifier<A, B, C>>,
    pub change_state: fn(&mut Phoneme, &str),
    pub behavior: DiacriticsBehavior,
    pub prepend_blank: bool,
}

pub enum DiacriticsBehavior {
    Single {
        contains: fn(&Phoneme) -> bool,
        remove: fn(&mut Phoneme),
    },
    Multiple {
        contains: fn(&Phoneme, symbol: &str) -> bool,
        remove: fn(&mut Phoneme, symbol: &str)
    }
}

fn excl<T: Category>(excluding: &[T]) -> Vec<T> {
    let mut collection: HashSet<T> = HashSet::from_iter(all::<T>());

    for excluding in excluding.iter() {
        collection.remove(excluding);
    }
    
    collection.into_iter().collect::<Vec<_>>()
}

fn trans<A: Category, B: Category, C: Category>(
    restriction: impl Into<PhonemeQuality<A, B, C>>, 
    symbol: &str, 
    desc: &str) -> Modifier<A, B, C> {

    (restriction.into(), String::from(symbol), String::from(desc))
}

pub fn modifiers_consonants(
    phonemes: &SlotMap<slotmap::DefaultKey, Phoneme>,
    ipa: &Alphabet<Articulation, Region, Voicing>,
    quality: PhonemeQuality<Articulation, Region, Voicing>
) -> impl Iterator<Item = Diacritics<Articulation, Region, Voicing>> {

    let mut diacritics = Vec::new();

    // AFFRICATES
    if quality.0.contains(&Articulation::Plosive) {
        let mut contents = Vec::new();

        let query: PhonemeQuality<Articulation, Region, Voicing> = (
            &[Articulation::Fricative, Articulation::LatFricative][..],
            &[][..],
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
            category: "Affrication",
            contents, 
            change_state: |phoneme: &mut Phoneme, symbol: &str| 
                phoneme.phone.affricate(symbol), 
            behavior: DiacriticsBehavior::Single { 
                contains: |phoneme: &Phoneme| -> bool {
                    if let Phone::Consonant { affricated, .. } = &phoneme.phone {
                        return affricated.is_some();
                    } else {
                        unreachable!();
                    }
                }, 
                remove: |phoneme: &mut Phoneme| {
                    if let Phone::Consonant {ref mut affricated, .. } = phoneme.phone {
                        let _ = affricated.take();
                    } else {
                        unreachable!();
                    }
                } 
            },
            prepend_blank: false
        });
    }

    // REGIONAL LEAN
    diacritics.push(Diacritics {
        category: "Regional Lean",
        contents: {
            use Region::*;

            vec![
                trans((&[][..], &[][..], &[][..]), "ʰ", "Aspirated"),
                trans((&[][..], excl(&[Bilabial, Labiodental]).as_slice(), &[][..]), "ʷ", "Labialized"),
                trans((&[][..], excl(&[Palatal]).as_slice(), &[][..]), "ʲ", "Palatalized"),
                trans((&[][..], excl(&[Velar]).as_slice(), &[][..]), "ˠ", "Velarized"),
                trans((&[][..], excl(&[Pharyngeal]).as_slice(), &[][..]), "ˤ", "Pharyngealized"),
                trans((&[][..], excl(&[Glottal]).as_slice(), &[][..]), "ˀ", "Glottalized"),
                trans((&[][..], &[][..], &[][..]), "ⁿ", "Nasal Release"),
                trans((&[][..], &[][..], &[][..]), "ˡ", "Lateral Release")
            ]
        },
        change_state: |phoneme: &mut Phoneme, symbol: &str|
            phoneme.phone.regionalize(symbol),
        behavior: DiacriticsBehavior::Single { 
            contains: |phoneme: &Phoneme| -> bool {
                if let Phone::Consonant { regionalized, .. } = &phoneme.phone {
                    return regionalized.is_some();
                } else {
                    unreachable!();
                }
            }, 
            remove: |phoneme: &mut Phoneme| {
                if let Phone::Consonant {ref mut regionalized, .. } = phoneme.phone {
                    let _ = regionalized.take();
                } else {
                    unreachable!();
                }
            } 
        },
        prepend_blank: true,
    });

    // CONSONANT DIACRITICS
    diacritics.push(Diacritics { 
        category: "Quality", 
        contents: {
            use Voicing::*;
            use Region::*;
            use Articulation::*;

            vec![
                trans((&[][..], &[][..], &[Voiced][..]), "\u{030A}", "Voiceless"),
                trans((&[][..], &[][..], &[Voiceless][..]), "\u{032C}", "Voiced"),
                trans((&[][..], &[][..], &[][..]), "\u{031F}", "Advanced"),
                trans((&[][..], &[][..], &[][..]), "\u{0320}", "Retracted"),
                trans((&[][..], &[][..], &[][..]), "\u{0324}", "Breathy"),
                trans((&[][..], &[][..], &[][..]), "\u{0330}", "Creaky"),
                trans((&[][..], &[][..], &[][..]), "\u{033C}", "Linguolabial"),
                trans((&[][..], excl(&[Velar, Pharyngeal][..]).as_slice(), &[][..]), 
                    "̴", "Velarized or Pharyngealized"),
                trans((&[][..], &[][..], &[][..]), "\u{031D}", "Raised"),
                trans((&[][..], &[][..], &[][..]), "\u{031E}", "Lowered"),
                trans((&[][..], excl(&[Dental][..]).as_slice(), &[][..]), 
                    "\u{032A}", "Dental"),
                trans((&[][..], &[][..], &[][..]), "\u{033A}", "Apical"),
                trans((&[][..], &[][..], &[][..]), "\u{033B}", "Laminal"),
                trans((Plosive, &[][..], &[][..]), "\u{033A}", "Applosive"),
                trans((excl(&[Plosive, Fricative, LatFricative][..]).as_slice(), &[][..], &[][..]), 
                    "\u{033B}", "Syllabic")
            ]
        }, 
        change_state: |phoneme: &mut Phoneme, symbol: &str| 
            phoneme.add_diacritic(symbol), 
        behavior: DiacriticsBehavior::Multiple { 
            contains: |phoneme: &Phoneme, symbol: &str| 
                format!("{}", phoneme).contains(symbol), 
            remove: |phoneme: &mut Phoneme, symbol: &str|
                phoneme.symbol.remove_matches(symbol)
        }, 
        prepend_blank: true 
    });

    diacritics.into_iter()
}

#[allow(dead_code, unused_variables)]
pub fn modifiers_vowels(
    phonemes: &SlotMap<slotmap::DefaultKey, Phoneme>,
    ipa: &Alphabet<Constriction, Place, Rounding>,
    quality: PhonemeQuality<Constriction, Place, Rounding>
) -> impl Iterator<Item = Diacritics<Constriction, Place, Rounding>> {
    let mut diacritics = Vec::new();

    diacritics.push(Diacritics { 
        category: "Quality", 
        contents: {
            use Constriction::*;
            use Place::*;
            use Rounding::*;
            
            vec![
                trans((&[][..], &[][..], &[Unrounded][..]), "̹", "Rounded"),
                trans((&[][..], &[][..], &[Rounded][..]), "̹", "Rounded"),
                trans((&[][..], excl(&[Central]).as_slice(), &[][..]), "\u{0308}", "Centralized"),
                trans((excl(&[Mid]).as_slice(), excl(&[Central]).as_slice(), &[][..]), 
                    "\u{033D}", "Mid-Centralized"), // TODO: This is problematic, should show for all but ə
                trans((&[][..], &[][..], &[][..]), "\u{032F}", "Non-Syllabic"),
                trans((&[][..], &[][..], &[][..]), "\u{0303}", "Nasal"),
                trans((&[][..], &[][..], &[][..]), "\u{0318}", "Advanced Tongue Root"),
                trans((&[][..], &[][..], &[][..]), "\u{0319}", "Retracted Tongue Root"),
                trans((&[][..], &[][..], &[][..]), "˞", "R-Colored")
            ]
        }, 
        change_state: |phoneme: &mut Phoneme, symbol: &str|
            phoneme.add_diacritic(symbol), 
        behavior: DiacriticsBehavior::Multiple { 
            contains: |phoneme: &Phoneme, symbol: &str| 
                format!("{}", phoneme).contains(symbol), 
            remove: |phoneme: &mut Phoneme, symbol: &str|
                phoneme.symbol.remove_matches(symbol)
        }, 
        prepend_blank: true 
    });

    diacritics.into_iter()
}