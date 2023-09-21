use crate::pane::Pane;

pub struct RomanizationPane;

impl Pane for RomanizationPane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new("Romanization")
    }

    fn show(&mut self, _state: &mut crate::State, _ui: &mut egui::Ui) {
        
    }
}