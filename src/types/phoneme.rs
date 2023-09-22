use std::{borrow::Cow, mem, fmt, rc::Rc, vec};

use crate::types::category::Category;

const CONSONANT_: &Phone = &Phone::consonant();
const VOWEL_: &Phone = &Phone::vowel();

pub const CONSONANT: mem::Discriminant<Phone> = mem::discriminant(CONSONANT_);
pub const VOWEL: mem::Discriminant<Phone> = mem::discriminant(VOWEL_);

#[derive(Clone)]
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

#[derive(Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Phoneme {
    pub symbol: String,
    pub grapheme: String,
    pub phone: Phone
}

impl Phoneme {
    pub fn new<'a, C: Into<Cow<'a, str>>>(symbol: C, phone: Phone) -> Self {
        let symbol: Cow<'_, str> = symbol.into();

        Self {
            symbol: symbol.to_string(),
            grapheme: String::new(),
            phone
        }
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

#[derive(Clone)]
pub struct PhonemeQuality<A, B, C>(pub Rc<[A]>, pub Rc<[B]>, pub Rc<[C]>)
    where A: Category, B: Category, C: Category;

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