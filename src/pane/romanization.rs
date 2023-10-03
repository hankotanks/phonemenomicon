use egui_extras::{Size, Column};
use slotmap::SlotMap;

use crate::pane::Pane;
use crate::pane::util;
use crate::types::PhonemeQuality;
use crate::types::category::CategoryColor;
use crate::types::category::{Outer, Inner, Pair};
use crate::types::{Phoneme, Alphabet};
use crate::app::FONT_ID;

pub struct RomanizationPane;

impl Pane for RomanizationPane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new("Romanization")
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {
        egui_extras::StripBuilder::new(ui) 
            .sizes(Size::remainder(), 2)
            .horizontal(|mut strip: egui_extras::Strip<'_, '_>| {
                strip.cell(|ui| { ui.push_id(
                    "romanization-pane-consonant-list", |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Consonants");
                            show_graphemes(ui, 
                                &state.dialects[state.inventory].consonants, 
                                &mut state.phonemes);
                        });
                    });
                });

                strip.cell(|ui| { ui.push_id(
                    "romanization-pane-vowel-list", |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Vowels");
                            show_graphemes(ui, 
                                &state.dialects[state.inventory].vowels, 
                                &mut state.phonemes);
                        });
                        
                    });
                });
            });
    }
}

fn show_grapheme_row<A, B, C>(
    mut row: egui_extras::TableRow<'_, '_>, 
    phoneme: &mut Phoneme, 
    quality: PhonemeQuality<A, B, C>) where 
    A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor {

    row.col(|ui| {
        ui.painter().rect_filled(
            { 
                let mut rect = ui.available_rect_before_wrap();

                rect.set_bottom(rect.bottom() - ui.style().spacing.item_spacing.y);
                rect
            }, 
            0., util::cell_color(ui, Some(quality)));

        let content = egui::RichText::new(format!("{}", phoneme))
            .font(FONT_ID.to_owned())
            .background_color(egui::Color32::TRANSPARENT);
        ui.label(content); 
    });

    row.col(|ui| {
        let grapheme_editor = //
            egui::TextEdit::singleline(&mut phoneme.grapheme)
                .font(FONT_ID.to_owned());

        ui.add(grapheme_editor);
    });
}

fn show_graphemes<A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor>(
    ui: &mut egui::Ui, 
    inventory: &Alphabet<A, B, C>,
    phonemes: &mut SlotMap<slotmap::DefaultKey, Phoneme>) {

    let row_height = FONT_ID.size;
    let row_height = row_height + ui.style().spacing.item_spacing.y * 4.;
    let row_height = row_height + ui.style().spacing.button_padding.y * 2.;
    
    egui_extras::TableBuilder::new(ui)
        .columns(Column::remainder(), 2)
        .vscroll(true)
        .body(|mut body| {
            for (id, quality) in inventory.phoneme_qualities() {
                if let Some(phoneme) = phonemes.get_mut(id) {
                    body.row(row_height, |row| {
                        show_grapheme_row(row, phoneme, quality);
                    });
                }
            }
        });
}