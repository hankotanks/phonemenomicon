use crate::pane::Pane;

pub struct InventoryPane;

impl Pane for InventoryPane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new(egui::RichText::default())
    }

    fn show(&mut self, _state: &mut crate::State, _ui: &mut egui::Ui) {
        
    }
}