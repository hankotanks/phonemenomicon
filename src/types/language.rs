
use std::rc;

use crate::types::Alphabet;

use crate::types::category::{
    Articulation, 
    Region, 
    Voicing, 
    Constriction, 
    Place,
    Rounding
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Language {
    pub name: rc::Rc<str>,
    pub consonants: Alphabet<Articulation, Region, Voicing>,
    pub vowels: Alphabet<Constriction, Place, Rounding>
}

impl Default for Language {
    fn default() -> Self {
        Self { 
            name: rc::Rc::from("Untitled"),
            consonants: Alphabet::new(), 
            vowels: Alphabet::new()
        }
    }
}