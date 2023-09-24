use crate::{pane::Pane, app::FONT_ID};

use crate::pane::language::LanguagePaneRole;

pub struct SoundChangePane {
    pub most_recent_buffer: Option<(slotmap::DefaultKey, LanguagePaneRole)>
}

impl Pane for SoundChangePane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new("Sound Changes")
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {
        if let Some(contents) = state.phoneme_buffer.take() {
            let _ = self.most_recent_buffer.insert(contents);
        }

        let contents = match self.most_recent_buffer {
            Some((id, source)) => //
                format!("{} from {}", state.phonemes[id], source),
            None => String::from("Nothing selected.")
        };

        let contents = egui::RichText::new(contents).font(FONT_ID.to_owned());

        ui.label(contents);
    }
}