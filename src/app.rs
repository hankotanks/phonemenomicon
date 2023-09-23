use std::marker;

use enum_map::EnumMap;
use once_cell::sync::Lazy;

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
#[allow(non_upper_case_globals, dead_code)]
const GentiumPlus: marker::PhantomData<fn() -> ()> = marker::PhantomData;

const IPA_FONT_NAME: &str = stringify!(GentiumPlus);
const IPA_FONT_BYTES: &[u8] = include_bytes!(concat!(
    "../assets/fonts/", 
    stringify!(GentiumPlus), 
    ".ttf"
));

pub static IPA_FONT_FAMILY: Lazy<egui::FontFamily> = Lazy::new(|| {
    egui::FontFamily::Name("IPA".into())
});

fn load_fonts() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        IPA_FONT_NAME.to_owned(),
        egui::FontData::from_static(IPA_FONT_BYTES)
    );

    fonts.families.insert(
        IPA_FONT_FAMILY.to_owned(), 
        vec![IPA_FONT_NAME.to_owned()]
    );

    fonts
}

#[allow(dead_code)]
pub static IPA_FONT_ID: Lazy<egui::FontId> = Lazy::new(|| egui::FontId {
    size: 16.,
    family: IPA_FONT_FAMILY.to_owned()
});

impl App {
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_fonts(load_fonts());

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
