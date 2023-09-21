use crate::pane::Pane;

pub struct SoundChangePane;

impl Pane for SoundChangePane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new("Sound Changes")
    }

    fn show(&mut self, _state: &mut crate::State, _ui: &mut egui::Ui) {
        
    }
}