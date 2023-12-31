pub mod diacritics;

use std::mem;

use egui_extras::Column;
use slotmap::SlotMap;

use crate::types::{Alphabet, Phoneme, CONSONANT, VOWEL, Language, PhonemeQuality};
use crate::types::category::{Outer, Inner, Pair};
use crate::types::category::{Articulation, Region, Voicing, Constriction, Place, Rounding};

use crate::app::FONT_ID;

fn show_row_content<A, B, C>(
    mut row: egui_extras::TableRow<'_, '_>, 
    diacritics: &diacritics::Diacritics<A, B, C>,
    phoneme: &mut Phoneme,
    modifier: &str, desc: &str) where 
    A: Outer<B, C>, B: Inner<C>, C: Pair {
    row.col(|ui| {
        let content = if diacritics.prepend_blank {
            format!("\u{25CC}{}", modifier)
        } else {
            format!("{}", modifier)
        };
    
        let content =  egui::RichText::new(content)
            .font(FONT_ID.to_owned());

        match diacritics.behavior {
            diacritics::DiacriticsBehavior::Single { .. } => {

                if ui.button(content).clicked() {
                    (diacritics.change_state)(phoneme, modifier);

                    ui.close_menu();
                }
            },
            diacritics::DiacriticsBehavior::Multiple { 
                contains, remove } => {
                
                let mut state = (contains)(&phoneme, &modifier);
                if ui.toggle_value(&mut state, content).clicked() {
                    if !state {
                        (remove)(phoneme, &modifier);
                    } else {
                        (diacritics.change_state)(phoneme, &modifier);
                    }
                    
                    ui.close_menu();
                }
            },
        }
    });

    row.col(|ui| { ui.label(desc); });
}

fn show_rows<A, B, C>(
    ui: &mut egui::Ui, 
    diacritics: diacritics::Diacritics<A, B, C>,
    quality: PhonemeQuality<A, B, C>,
    phoneme: &mut Phoneme) where 
    A: Outer<B, C>, B: Inner<C>, C: Pair {
    let row_height = FONT_ID.size;
    let row_height = row_height + ui.style().spacing.button_padding.y * 2.;
    let row_height = row_height + ui.style().spacing.item_spacing.y;
    egui_extras::TableBuilder::new(ui)
        .column(Column::initial(FONT_ID.size).at_least(FONT_ID.size))
        .column(Column::remainder())
        .body(|mut body| {
            for (restriction, modifier, desc) in diacritics.contents.iter() {
                if quality.meets_restrictions(restriction.clone()) {
                    
                    body.row(row_height, |row| {
                        show_row_content(row, &diacritics, phoneme, &modifier, &desc);
                    });
                }
            }
        });
}

#[allow(unused_variables)]
fn diacritics_display<A, B, C>(
    ui: &mut egui::Ui, 
    diacritics: diacritics::Diacritics<A, B, C>, 
    quality: PhonemeQuality<A, B, C>,
    phoneme: &mut Phoneme) where A: Outer<B, C>, B: Inner<C>, C: Pair {

    match diacritics.behavior {
        diacritics::DiacriticsBehavior::Single { contains, remove } => {
            if (contains)(&phoneme) {
                let content = format!("Remove {}", diacritics.category);
                if ui.button(content).clicked() {
                    (remove)(phoneme);

                    ui.close_menu();
                }

            } else {
                ui.menu_button(diacritics.category, |ui| {
                    show_rows(ui, diacritics, quality, phoneme);
                });
            }
        },
        diacritics::DiacriticsBehavior::Multiple { .. } => {
            ui.menu_button(diacritics.category, |ui| {
                show_rows(ui, diacritics, quality, phoneme);
            });
        },
    }
}

pub enum Context<'a, A: Outer<B, C>, B: Inner<C>, C: Pair> {
    Bound { inventory: &'a mut Alphabet<A, B, C>, id: slotmap::DefaultKey },
    Free { quality: PhonemeQuality<A, B, C>, phoneme: &'a mut Phoneme }
}

#[allow(unused_variables)]
pub fn cell_context<A: Outer<B, C>, B: Inner<C>, C: Pair>(
    ui: &mut egui::Ui,
    ipa: &Language,
    phonemes: &mut SlotMap<slotmap::DefaultKey, Phoneme>,
    context: Context<'_, A, B, C>) {
    
    match context {
        Context::Bound { inventory, id } => {
            // TODO: There must be a better way
            let quality = inventory.get_quality(id).unwrap();

            type Src<A, B, C> = PhonemeQuality<A, B, C>;
            if mem::discriminant(&phonemes[id].phone) == CONSONANT {
                let quality = unsafe {
                    type Dst = PhonemeQuality<Articulation, Region, Voicing>;
                    mem::transmute::<&Src<A, B, C>, &Dst>(&quality)
                };
        
                for diacritics in diacritics::modifiers_consonants(
                    phonemes, &ipa.consonants, quality.clone()) {
                    
                    diacritics_display(ui, diacritics, quality.clone(), &mut phonemes[id]);
                }
            } else if mem::discriminant(&phonemes[id].phone) == VOWEL {
                let quality = unsafe {
                    type Dst = PhonemeQuality<Constriction, Place, Rounding>;
                    mem::transmute::<&Src<A, B, C>, &Dst>(&quality)
                };
        
                for diacritics in diacritics::modifiers_vowels(
                    phonemes, &ipa.vowels, quality.clone()) {
                    
                    diacritics_display(ui, diacritics, quality.clone(), &mut phonemes[id]);
                }
            } else {
                unreachable!();
            }

            let content = egui::RichText::new("Remove Phoneme").italics();

            if ui.button(content).clicked() {
                phonemes.remove(id);
                inventory.remove_phoneme(id);
        
                ui.close_menu();
            }
        },
        Context::Free { quality, phoneme } => {
            type Src<A, B, C> = PhonemeQuality<A, B, C>;
            if mem::discriminant(&phoneme.phone) == CONSONANT {
                let quality = unsafe {
                    type Dst = PhonemeQuality<Articulation, Region, Voicing>;
                    mem::transmute::<&Src<A, B, C>, &Dst>(&quality)
                };
        
                for diacritics in diacritics::modifiers_consonants(
                    phonemes, &ipa.consonants, quality.clone()) {
                    
                    diacritics_display(ui, diacritics, quality.clone(), phoneme);
                }
            } else if mem::discriminant(&phoneme.phone) == VOWEL {
                let quality = unsafe {
                    type Dst = PhonemeQuality<Constriction, Place, Rounding>;
                    mem::transmute::<&Src<A, B, C>, &Dst>(&quality)
                };
        
                for diacritics in diacritics::modifiers_vowels(
                    phonemes, &ipa.vowels, quality.clone()) {
                    
                    diacritics_display(ui, diacritics, quality.clone(), phoneme);
                }
            } else {
                unreachable!();
            }
        }
    }
}