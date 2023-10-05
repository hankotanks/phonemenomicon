use crate::pane::Pane;

pub struct LexiconPane;

impl Pane for LexiconPane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new("Lexicon")
    }

    fn show(&mut self, _windowed: bool, _state: &mut crate::State, _ui: &mut egui::Ui) {
        
    }
}