mod context;
mod inventory;

use std::fmt;

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

impl fmt::Display for LanguagePaneRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            LanguagePaneRole::Inventory => "Inventory",
            LanguagePaneRole::Ipa => "IPA",
        })
    }
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
    fn setup<'a, 'b: 'a>(&'a mut self, ctx: &egui::Context) -> egui::Window<'b> {
        let spacing = ctx.style().spacing.item_spacing;
        let padding = ctx.style().spacing.button_padding;

        let width = cardinality::<Region>() * cardinality::<Voicing>();
        let width = cardinality::<Place>() * cardinality::<Rounding>() + width;
        let width = (FONT_ID.size + spacing.x) * width as f32;

        let height = cardinality::<Articulation>().max(cardinality::<Constriction>());
        let height = (FONT_ID.size + (spacing.y + padding.y) * 2.) * (height + 2) as f32;
        
        egui::Window::new(format!("{}", self.role))
            .resizable(true)
            .constrain(true)
            .min_width(width)
            .min_height(height)
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {        
        let (mut consonants, mut vowels) = match self.role {
            LanguagePaneRole::Inventory => {
                let consonants = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Display { inventory: &mut state.inventory.consonants }
                };

                let vowels = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Display { inventory: &mut state.inventory.vowels }
                };

                (consonants, vowels)
            },
            LanguagePaneRole::Ipa => {
                let consonants = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Source { 
                        inventory: &mut state.inventory.consonants, 
                        phonemes: &state.ipa.consonants 
                    }
                };

                let vowels = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Source { 
                        inventory: &mut state.inventory.vowels, 
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
                        ui,
                        state.invalid.clone(), 
                        state.space.clone(), 
                        &mut state.phonemes, 
                        &mut state.phoneme_buffer,
                        &state.ipa
                    );
                });

                strip.cell(|ui| {
                    vowels.display(
                        ui,
                        state.invalid.clone(), 
                        state.space.clone(), 
                        &mut state.phonemes, 
                        &mut state.phoneme_buffer,
                        &state.ipa
                    );
                })
            });        
    }
}