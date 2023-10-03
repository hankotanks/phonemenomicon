use egui_graphs::{Graph, GraphView};

use crate::pane::Pane;

pub struct DialectPane;

impl DialectPane {
    pub fn new() -> Self {
        Self
    }
}

impl Pane for DialectPane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new("Dialects")
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {
        log::info!("test");
        let mut graph = Graph::from(&state.language_tree);

        let view = GraphView::new(&mut graph);

        ui.add(view);
    }
}