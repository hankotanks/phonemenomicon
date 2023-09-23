mod inventory;

use std::fmt;

use crate::pane::Pane;

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
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new(format!("{}", self.role))
            .resizable(true)
            .constrain(true)
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {
        let (mut consonants, mut vowels) = match self.role {
            LanguagePaneRole::Inventory => {
                let consonants = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Display { inventory: &state.inventory.consonants }
                };

                let vowels = inventory::InventoryPane {
                    role: inventory::InventoryPaneRole::Display { inventory: &state.inventory.vowels }
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

        ui.vertical(|ui| {
            consonants.display(&state.phonemes, ui);
            //vowels.display(&state.phonemes, ui);
        });

        
    }
}