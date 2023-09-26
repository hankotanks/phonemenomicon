mod diacritics;

pub use diacritics::{modifiers_consonants, modifiers_vowels};

use crate::types::{Phoneme, category::{Outer, Pair, Inner}};

use self::diacritics::Diacritics;

#[allow(unused_variables)]
pub fn diacritics_display<A, B, C>(
    ui: &mut egui::Ui, 
    diacritics: Diacritics<A, B, C>, 
    phoneme: &mut Phoneme) where A: Outer<B, C>, B: Inner<C>, C: Pair {
    
    todo!("Unimplemented.")
    // Should create a menu_button if diacritics.submenu_visible
}