pub mod diacritics;

use egui_extras::Column;

use crate::types::{Phoneme, Alphabet};
use crate::types::category::{Outer, Pair, Inner};
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
    inventory: &Alphabet<A, B, C>,
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
                if inventory.meets_restrictions(phoneme.id(), restriction.clone()) {
                    
                    body.row(row_height, |row| {
                        show_row_content(row, &diacritics, phoneme, &modifier, &desc);
                    });
                }
            }
        });
}

#[allow(unused_variables)]
pub fn diacritics_display<A, B, C>(
    ui: &mut egui::Ui, 
    diacritics: diacritics::Diacritics<A, B, C>, 
    inventory: &Alphabet<A, B, C>,
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
                    show_rows(ui, diacritics, inventory, phoneme);
                });
            }
        },
        diacritics::DiacriticsBehavior::Multiple { .. } => {
            ui.menu_button(diacritics.category, |ui| {
                show_rows(ui, diacritics, inventory, phoneme);
            });
        },
    }
}