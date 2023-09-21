mod inventory;

use std::fmt;

use crate::pane::Pane;

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
    }

    fn show(&mut self, _state: &mut crate::State, _ui: &mut egui::Ui) {
        
    }
}