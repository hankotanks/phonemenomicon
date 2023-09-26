mod diacritics;

pub use diacritics::{modifiers_consonants, modifiers_vowels};

use crate::types::{Phoneme, category::{Outer, Pair, Inner}};

use self::diacritics::Diacritics;

#[allow(unused_variables)]
pub fn diacritics_display<A, B, C>(
    ui: &mut egui::Ui, 
    diacritics: Diacritics<A, B, C>, 
    phoneme: &mut Phoneme) where A: Outer<B, C>, B: Inner<C>, C: Pair {

    match diacritics.behavior {
        diacritics::DiacriticsBehavior::Single { contains, remove } => {
            if (contains)(&phoneme) {
                if ui.button(format!("Remove {}", diacritics.category)).clicked() {
                    (remove)(phoneme);
                }
            } else {
                ui.menu_button(diacritics.category, |ui| {
                    for (restriction, modifier, desc) in diacritics.contents.into_iter() {
                        
                    }
                });
            }
        },
        diacritics::DiacriticsBehavior::Multiple { contains, remove } => {
            todo!("Not implemented");
        },
    }
    
    todo!("Unimplemented.")
    // Should create a menu_button if diacritics.submenu_visible
}