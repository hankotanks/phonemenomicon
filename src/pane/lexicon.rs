use crate::pane::Pane;

pub struct LexiconPane;

impl Pane for LexiconPane {
    fn title(&self, _state: &crate::State) -> std::rc::Rc<str> {
        std::rc::Rc::from("Lexicon")
    }
    
    fn setup<'a, 'b: 'a>(&'a mut self, state: &crate::State, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new(self.title(state).as_ref())
    }

    fn show(&mut self, _windowed: bool, _state: &mut crate::State, _ui: &mut egui::Ui) {
        
    }
}