use enum_map::EnumMap;

use crate::State;
use crate::pane::{PaneId, Pane, init_panes};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    state: State,
    #[serde(skip)]
    panes: EnumMap<PaneId, Box<dyn Pane>>
}

impl Default for App {
    fn default() -> Self {
        Self { 
            state: State::default(), 
            panes: init_panes()
        }
    }
}

impl App {
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
