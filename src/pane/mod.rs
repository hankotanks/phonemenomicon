mod language;
mod romanization;
mod lexicon;
mod sound_change;

pub use language::LanguagePaneRole;

use language::LanguagePane;
use romanization::RomanizationPane;
use lexicon::LexiconPane;
use sound_change::SoundChangePane;

use enum_map::{Enum, EnumMap, enum_map};

use crate::State;

#[derive(Clone, Copy, Enum)]
pub enum PaneId {
    Inventory,
    Ipa,
    Romanization,
    Lexicon,
    SoundChange
}

pub trait Pane {
    fn setup<'a, 'b: 'a>(&'a mut self, ctx: &egui::Context) -> egui::Window<'b>;
    fn show(&mut self, state: &mut State, ui: &mut egui::Ui);
}

pub fn init_panes() -> EnumMap<PaneId, Box<dyn Pane>> {
    enum_map! {
        PaneId::Inventory => {
            let temp: Box<dyn Pane> = Box::new(LanguagePane::new(LanguagePaneRole::Inventory));
            temp
        },
        PaneId::Ipa => {
            let temp: Box<dyn Pane> = Box::new(LanguagePane::new(LanguagePaneRole::Ipa));
            temp
        },
        PaneId::Romanization => {
            let temp: Box<dyn Pane> = Box::new(RomanizationPane);
            temp
        },
        PaneId::Lexicon => {
            let temp: Box<dyn Pane> = Box::new(LexiconPane);
            temp
        },
        PaneId::SoundChange => {
            let temp: Box<dyn Pane> = Box::new(SoundChangePane { most_recent_buffer: None });
            temp
        }
    }
}