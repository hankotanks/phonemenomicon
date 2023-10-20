use std::{mem, fmt, vec};
use std::rc::Rc;
use std::borrow::Cow;

use enum_iterator::{Sequence, all};

use crate::types::category::Category;
use crate::app::FONT_ID;

const CONSONANT_: &Phone = &Phone::consonant();
const VOWEL_: &Phone = &Phone::vowel();

pub const CONSONANT: mem::Discriminant<Phone> = mem::discriminant(CONSONANT_);
pub const VOWEL: mem::Discriminant<Phone> = mem::discriminant(VOWEL_);

#[derive(Clone, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Phone {
    Consonant {
        affricated: Option<Rc<str>>,
        regionalized: Option<Rc<str>>
    },
    Vowel
}

impl Phone {
    pub const fn consonant() -> Self {
        Self::Consonant { affricated: None, regionalized: None }
    }

    pub const fn vowel() -> Self {
        Self::Vowel
    }

    pub fn regionalize<'a, C: Into<Cow<'a, str>>>(&mut self, symbol: C) {
        if let Self::Consonant { ref mut regionalized, .. } = self {
            let symbol: Cow<'_, str> = symbol.into();
            let symbol = Rc::from(symbol.as_ref());

            let _ = regionalized.insert(symbol);
        } else {
            panic!();
        }
    }

    pub fn affricate<'a, C: Into<Cow<'a, str>>>(&mut self, symbol: C) {
        if let Self::Consonant { ref mut affricated, .. } = self {
            let symbol: Cow<'_, str> = symbol.into();
            let symbol = Rc::from(symbol.as_ref());

            let _ = affricated.insert(symbol);
        } else {
            panic!();
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Phoneme {
    pub symbol: String,
    pub grapheme: String,
    pub phone: Phone,
    id: slotmap::DefaultKey,
    id_state: bool
}

impl Phoneme {
    pub fn new<'a, C: Into<Cow<'a, str>>>(symbol: C, phone: Phone) -> Self {
        let symbol: Cow<'_, str> = symbol.into();

        Self {
            symbol: symbol.to_string(),
            grapheme: String::new(),
            phone,
            id: slotmap::DefaultKey::default(),
            id_state: false
        }
    }

    pub fn set_id(&mut self, id: slotmap::DefaultKey) {
        if self.id_state { panic!(); }

        self.id = id;
        self.id_state = true;
    }

    pub fn id(&self) -> slotmap::DefaultKey {
        if !self.id_state { panic!(); }

        self.id
    }
}

impl fmt::Display for Phoneme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.phone {
            Phone::Consonant { affricated, regionalized } => {
                write!(f, "{}{}{}", 
                    self.symbol,
                    regionalized.as_ref().unwrap_or(&Rc::from("")),
                    affricated.as_ref().map_or(String::from(""), |affricate| {
                        format!("\u{0361}{}", affricate)
                    })
                )
            },
            Phone::Vowel => write!(f, "{}", self.symbol),
        }
    }
}

impl Phoneme {
    pub fn add_diacritic<'a, C: Into<Cow<'a, str>>>(&mut self, diacritic: C) {
        let diacritic: Cow<'_, str> = diacritic.into();

        self.symbol.push_str(diacritic.as_ref());
    }
}

impl Into<egui::RichText> for Phoneme {
    fn into(self) -> egui::RichText {
        egui::RichText::from(format!("{}", self)).font(FONT_ID.to_owned())
    }
}

fn into_raw(quality: Rc<[impl Category]>) -> Rc<[usize]> {
    let raw = quality.iter().map(|q| q.into_usize()).collect::<Vec<_>>();
    Rc::from(raw.as_slice())
}

fn from_raw<T: Category>(raw: Rc<[usize]>) -> Rc<[T]> {
    let quality = raw.iter().map(|r| T::from_usize(*r)).collect::<Vec<_>>();
    Rc::from(quality.as_slice())
}

#[derive(Clone, Debug)]
pub struct PhonemeQuality<A, B, C>(pub Rc<[A]>, pub Rc<[B]>, pub Rc<[C]>)
    where A: Category, B: Category, C: Category;

impl<A, B, C> PhonemeQuality<A, B, C> 
    where A: Category, B: Category, C: Category {

    pub fn blank() -> Self {
        (&[][..], &[][..], &[][..]).into()
    }

    pub fn into_raw(&self) -> (Rc<[usize]>, Rc<[usize]>, Rc<[usize]>) {
        (into_raw(self.0.clone()), into_raw(self.1.clone()), into_raw(self.2.clone()))
    }

    pub fn from_raw(raw: (Rc<[usize]>, Rc<[usize]>, Rc<[usize]>)) -> Self {
        let (a, b, c) = raw;

        Self(from_raw::<A>(a), from_raw::<B>(b), from_raw::<C>(c))
    }

    /// Example:
    /// If `restriction` is (Plosive, [Dental, Alveolar], [Voiced, Voiceless])
    /// And the phoneme tested is 'd' (Plosive, [Alevolar, PostAlveolar], Voiced)
    /// This function returns truthy
    /// Empty slices are shorthand for 'all variants'
    /// Because you would never search for an 
    pub fn meets_restrictions(&self, mut restriction: PhonemeSelector<A, B, C>) -> bool {
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

        // TODO: This clone sucks
        self.clone().into_iter().any(|item| restrictions.contains(&item))
    }
}

impl<A, B, C> IntoIterator for PhonemeQuality<A, B, C>
    where A: Category, B: Category, C: Category {

    type Item = (A, B, C);
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut queries = Vec::new();
        for a in self.0.into_iter() {
            for b in self.1.into_iter() {
                for c in self.2.into_iter() {
                    queries.push((*a, *b, *c));
                }
            }
        }

        queries.into_iter()
    }
}

impl<A, B, C> serde::Serialize for PhonemeQuality<A, B, C>
    where A: Category, B: Category, C: Category {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {

        #[derive(serde::Serialize)]
        struct Intermediate<A, B, C> { a: Vec<A>, b: Vec<B>, c: Vec<C> }

        let intermediate = Intermediate {
            a: self.0.to_vec(),
            b: self.1.to_vec(),
            c: self.2.to_vec()
        };

        intermediate.serialize(serializer)
    }
}

impl<'de, A, B, C> serde::Deserialize<'de> for PhonemeQuality<A, B, C>
    where A: Category, B: Category, C: Category {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> {
        
            #[derive(serde::Deserialize)]
            struct Intermediate<A, B, C> { a: Vec<A>, b: Vec<B>, c: Vec<C> }

            let intermediate = Intermediate::deserialize(deserializer)?;
            let Intermediate { a, b, c } = intermediate;

            let quality = Self(
                Rc::from(a.as_slice()), 
                Rc::from(b.as_slice()), 
                Rc::from(c.as_slice())
            );

            Ok(quality)
    }
}

// TODO: There must be a better way to implement all of these Into's
// Maybe by having an intermediate type that implements Into for both A and &[A]

impl<A, B, C> From<(A, B, C)> for PhonemeQuality<A, B, C> 
    where A: Category, B: Category, C: Category {

    fn from(value: (A, B, C)) -> Self {
        Self(Rc::from([value.0]), Rc::from([value.1]), Rc::from([value.2]))
    }
}

impl<A, B, C> From<(&[A], B, C)> for PhonemeQuality<A, B, C> 
    where A: Category, B: Category, C: Category {

    fn from(value: (&[A], B, C)) -> Self {
        Self(Rc::from(value.0), Rc::from([value.1]), Rc::from([value.2]))
    }
}

impl<A, B, C> From<(A, &[B], C)> for PhonemeQuality<A, B, C> 
    where A: Category, B: Category, C: Category {

    fn from(value: (A, &[B], C)) -> Self {
        Self(Rc::from([value.0]), Rc::from(value.1), Rc::from([value.2]))
    }
}

impl<A, B, C> From<(A, B, &[C])> for PhonemeQuality<A, B, C> 
    where A: Category, B: Category, C: Category {

    fn from(value: (A, B, &[C])) -> Self {
        Self(Rc::from([value.0]), Rc::from([value.1]), Rc::from(value.2))
    }
}

impl<A, B, C> From<(&[A], &[B], C)> for PhonemeQuality<A, B, C> 
    where A: Category, B: Category, C: Category {

    fn from(value: (&[A], &[B], C)) -> Self {
        Self(Rc::from(value.0), Rc::from(value.1), Rc::from([value.2]))
    }
}

impl<A, B, C> From<(&[A], B, &[C])> for PhonemeQuality<A, B, C> 
    where A: Category, B: Category, C: Category {

    fn from(value: (&[A], B, &[C])) -> Self {
        Self(Rc::from(value.0), Rc::from([value.1]), Rc::from(value.2))
    }
}

impl<A, B, C> From<(A, &[B], &[C])> for PhonemeQuality<A, B, C> 
    where A: Category, B: Category, C: Category {

    fn from(value: (A, &[B], &[C])) -> Self {
        Self(Rc::from([value.0]), Rc::from(value.1), Rc::from(value.2))
    }
}

impl<A, B, C> From<(&[A], &[B], &[C])> for PhonemeQuality<A, B, C> 
    where A: Category, B: Category, C: Category {

    fn from(value: (&[A], &[B], &[C])) -> Self {
        Self(Rc::from(value.0), Rc::from(value.1), Rc::from(value.2))
    }
}


pub type PhonemeSelector<A, B, C> = PhonemeQuality<A, B, C>;