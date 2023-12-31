mod language;
mod romanization;
mod lexicon;
mod sound_change;
mod dialect_view;
mod context;

pub mod util;

use std::rc;

pub use language::LanguagePaneRole;

use language::LanguagePane;
use romanization::RomanizationPane;
use lexicon::LexiconPane;
use sound_change::SoundChangePane;
use dialect_view::DialectPane;

use enum_map::{Enum, EnumMap, enum_map};

use crate::State;

#[derive(Clone, Copy, Enum, Debug)]
pub enum PaneId {
    Inventory,
    Ipa,
    Romanization,
    Lexicon,
    SoundChange,
    Dialects
}

pub trait Pane {
    fn title(&self, state: &State) -> rc::Rc<str>;
    fn setup<'a, 'b: 'a>(&'a mut self, state: &State, ctx: &egui::Context) -> egui::Window<'b>;
    fn show(&mut self, windowed: bool, state: &mut State, ui: &mut egui::Ui);
    fn on_dialect_change(&mut self, state: &mut State);
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
            let temp: Box<dyn Pane> = Box::new(SoundChangePane::new());
            temp
        },
        PaneId::Dialects => {
            let temp: Box<dyn Pane> = Box::new(DialectPane::new());
            temp
        }
    }
}