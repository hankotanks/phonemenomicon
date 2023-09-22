use std::collections::HashMap;
use std::rc::Rc;

use enum_iterator::{all, Sequence};
use enum_map::{EnumMap, EnumArray};
use slotmap::DefaultKey;

use crate::types::category::Category;
use crate::types::{PhonemeQuality, PhonemeSelector};

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

    pub fn select_phonemes(&self, query: PhonemeSelector<A, B, C>) -> impl Iterator<Item = DefaultKey> + '_ {
        self.quality
            .keys()
            .filter(move |&id| self.meets_restrictions(*id, query.clone()))
            .cloned()
    }

    pub fn get_quality(&self, id: DefaultKey) -> Option<PhonemeQuality<A, B, C>> {
        self.quality.get(&id).cloned()
    }

    /// Example:
    /// If `restriction` is (Plosive, [Dental, Alveolar], [Voiced, Voiceless])
    /// And the phoneme tested is 'd' (Plosive, [Alevolar, PostAlveolar], Voiced)
    /// This function returns truthy
    /// Empty slices are shorthand for 'all variants'
    /// Because you would never search for an 
    pub fn meets_restrictions(&self, id: DefaultKey, mut restriction: PhonemeSelector<A, B, C>) -> bool {
        let quality = match self.get_quality(id) {
            Some(quality) => quality,
            None => panic!()
        };

        if restriction.0.is_empty() //
            && restriction.1.is_empty() //
            && restriction.2.is_empty() {

            return true;
        }

        fn every_variant<T: Sequence>() -> Rc<[T]> {
            let variants = all::<T>().collect::<Vec<_>>();
            Rc::from(variants)
        }

        if restriction.0.is_empty() { restriction.0 = every_variant::<A>(); }
        if restriction.1.is_empty() { restriction.1 = every_variant::<B>(); }
        if restriction.2.is_empty() { restriction.2 = every_variant::<C>(); }
        
        let restrictions: Vec<(A, B, C)> = restriction.into_iter().collect();

        quality.into_iter().any(|item| restrictions.contains(&item))
    }
}