mod inventory;

use std::rc;

use egui_extras::Size;
use enum_iterator::cardinality;

use crate::app::FONT_ID;
use crate::pane::Pane;

use crate::types::category::{
    Articulation,
    Region,
    Voicing,
    Constriction,
    Place,
    Rounding
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LanguagePaneRole {
    Inventory,
    Ipa
}

pub struct LanguagePane {
    role: LanguagePaneRole
}

impl LanguagePane {
    pub fn new(role: LanguagePaneRole) -> Self {
        Self {
            role
        }
    }
}

impl Pane for LanguagePane {
    fn title(&self, state: &crate::State) -> std::rc::Rc<str> {
        if matches!(self.role, LanguagePaneRole::Inventory) {
            state.dialects[state.inventory].name.clone()
        } else {
            rc::Rc::from("IPA")
        }
    }

    fn setup<'a, 'b: 'a>(&'a mut self, state: &crate::State, ctx: &egui::Context) -> egui::Window<'b> {
        let spacing = ctx.style().spacing.item_spacing;
        let padding = ctx.style().spacing.button_padding;

        let width = cardinality::<Region>() * cardinality::<Voicing>();
        let width = cardinality::<Place>() * cardinality::<Rounding>() + width;
        let width = (FONT_ID.size + spacing.x) * width as f32;

        let height = cardinality::<Articulation>().max(cardinality::<Constriction>());
        let height = (FONT_ID.size + (spacing.y + padding.y) * 2.) * (height + 2) as f32;
        
        egui::Window::new(self.title(state).as_ref())
            .resizable(true)
            .constrain(true)
            .min_width(width)
            .min_height(height)
    }

    fn show(&mut self, windowed: bool, state: &mut crate::State, ui: &mut egui::Ui) {    
        let inventory = &mut state.dialects[state.inventory];    
        let (mut consonants, mut vowels) = match self.role {
            LanguagePaneRole::Inventory => {
                let consonants = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Display { 
                        inventory: &mut inventory.consonants }
                };

                let vowels = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Display { 
                        inventory: &mut inventory.vowels }
                };

                (consonants, vowels)
            },
            LanguagePaneRole::Ipa => {
                let consonants = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Source { 
                        inventory: &mut inventory.consonants, 
                        phonemes: &state.ipa.consonants 
                    }
                };

                let vowels = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Source { 
                        inventory: &mut inventory.vowels, 
                        phonemes: &state.ipa.vowels
                    }
                };

                (consonants, vowels)
            }
        };

        let columns_c = (cardinality::<Region>() * cardinality::<Voicing>()) as f32;
        let columns_v = (cardinality::<Place>() * cardinality::<Rounding>()) as f32;

        let proportion = columns_c / (columns_c + columns_v);

        egui_extras::StripBuilder::new(ui)
            .size(Size::relative(proportion))
            .size(Size::remainder())
            .horizontal(|mut strip| {
                strip.cell(|ui| {
                    consonants.display(
                        windowed,
                        ui,
                        state.invalid.clone(), 
                        state.space.clone(), 
                        &mut state.phonemes, 
                        &mut state.buffer,
                        state.buffer_state,
                        &state.ipa
                    );
                });

                strip.cell(|ui| {
                    vowels.display(
                        windowed,
                        ui,
                        state.invalid.clone(), 
                        state.space.clone(), 
                        &mut state.phonemes, 
                        &mut state.buffer,
                        state.buffer_state,
                        &state.ipa
                    );
                })
            });        
    }

    fn on_dialect_change(&mut self, _state: &mut crate::State) { /* */ }
}