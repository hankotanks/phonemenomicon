use std::collections::HashMap;

use enum_map::{EnumMap, EnumArray};
use slotmap::DefaultKey;

use crate::types::category::Category;
use crate::types::PhonemeQuality;

pub struct Alphabet<A, B, C> where
    A: Category + EnumArray<EnumMap<B, EnumMap<C, Option<DefaultKey>>>>,
    B: Category + EnumArray<EnumMap<C, Option<DefaultKey>>>,
    C: Category + EnumArray<Option<DefaultKey>> {
    query: EnumMap<A, EnumMap<B, EnumMap<C, Option<DefaultKey>>>>,
    quality: HashMap<DefaultKey, PhonemeQuality<A, B, C>>
}

impl<A, B, C> serde::Serialize for Alphabet<A, B, C> where
    A: Category + EnumArray<EnumMap<B, EnumMap<C, Option<DefaultKey>>>>,
    B: Category + EnumArray<EnumMap<C, Option<DefaultKey>>>,
    C: Category + EnumArray<Option<DefaultKey>> {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        
        let mut intermediate = HashMap::new();

        for (id, quality) in self.quality.iter() {
            intermediate.insert(*id, quality.clone());
        }

        intermediate.serialize(serializer)
    }
}

impl<'de, A, B, C> serde::Deserialize<'de> for Alphabet<A, B, C> where
    A: Category + EnumArray<EnumMap<B, EnumMap<C, Option<DefaultKey>>>>,
    B: Category + EnumArray<EnumMap<C, Option<DefaultKey>>>,
    C: Category + EnumArray<Option<DefaultKey>> {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> {
        
        let mut alphabet = Alphabet::new();

        type Intermediate<A, B, C> = HashMap<DefaultKey, PhonemeQuality<A, B, C>>;

        let intermediate: Intermediate<A, B, C> = HashMap::deserialize(deserializer)?;
        intermediate.into_iter().for_each(|(id, quality)| {
            alphabet.add_phoneme(id, quality)
        });

        Ok(alphabet)
    }
}

impl<A, B, C> Alphabet<A, B, C> where
    A: Category + EnumArray<EnumMap<B, EnumMap<C, Option<DefaultKey>>>>,
    B: Category + EnumArray<EnumMap<C, Option<DefaultKey>>>,
    C: Category + EnumArray<Option<DefaultKey>>  {
    
    pub fn new() -> Self {
        Self {
            query: EnumMap::default(),
            quality: HashMap::new()
        }
    }

    pub fn add_phoneme(&mut self, id: DefaultKey, quality: PhonemeQuality<A, B, C>) {
        for (a, b, c) in quality.clone().into_iter() {
            self.query[a][b][c] = Some(id);
        }

        self.quality.insert(id, quality);
    }

    pub fn get(&self, query: (A, B, C)) -> Option<DefaultKey> {
        let (a, b, c) = query;

        self.query[a][b][c]
    }

    pub fn get_matching(&self) {
        // TODO
    }

    pub fn get_quality(&self, id: DefaultKey) -> Option<PhonemeQuality<A, B, C>> {
        self.quality.get(&id).cloned()
    }

    /// Example:
    /// If `restriction` is (Plosive, [Dental, Alveolar], [Voiced, Voiceless])
    /// And the phoneme tested is 'd' (Plosive, [Alevolar, PostAlveolar], Voiced)
    /// This function returns truthy
    pub fn meets_restrictions(&self, id: DefaultKey, restriction: PhonemeQuality<A, B, C>) -> bool {
        let quality = match self.get_quality(id) {
            Some(quality) => quality,
            None => panic!()
        };
        
        let restrictions: Vec<(A, B, C)> = restriction.into_iter().collect();

        quality.into_iter().any(|item| restrictions.contains(&item))
    }
}