use enum_map::EnumMap;

use crate::{State, pane::{PaneId, Pane, init_panes}, types::{Phoneme, Phone}};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    state: State,
    #[serde(skip)]
    panes: EnumMap<PaneId, Box<dyn Pane>>
}

impl Default for App {
    fn default() -> Self {
        let mut state = State::default();

        let panes = init_panes();

        Self { 
            state, 
            panes
        }
    }
}

impl App {
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut phoneme = Phoneme::new("d", Phone::consonant());
        phoneme.phone = Phone::Consonant { affricated: Some("v".into()), regionalized: None };
        phoneme.phone.regionalize("ʷ");
        phoneme.add_diacritic("\u{0324}");
        log::info!("{}", phoneme);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY)
                .unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    // Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for (_, pane) in self.panes.iter_mut() {
            pane.setup(ctx).show(ctx, |ui| {
                pane.show(&mut self.state, ui);
            });
        }
    }
}
