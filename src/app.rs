use std::{fmt, io};

use egui::mutex::Mutex;
use enum_map::EnumMap;
use include_dir::Dir;
use once_cell::sync::Lazy;

use crate::State;
use crate::pane::util::new_id;
use crate::pane::{PaneId, Pane, init_panes};

pub static STATUS: Lazy<Mutex<String>> = Lazy::new(|| 
    Mutex::new(String::from("")));

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

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Font {
    GentiumPlus,
    Andika,
    CharisSIL,
    DoulosSIL
}

impl Font {
    const fn as_filename(&self) -> &str {
        // NOTE: These filenames MUST match the contents of /assets/fonts
        match self {
            Font::GentiumPlus => "GentiumPlus.ttf",
            Font::Andika => "Andika.ttf",
            Font::CharisSIL => "CharisSIL.ttf",
            Font::DoulosSIL => "DoulosSIL.ttf"
        }
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Font::GentiumPlus => "Gentium Plus",
            Font::Andika => "Andika",
            Font::CharisSIL => "Charis SIL",
            Font::DoulosSIL => "Doulos SIL"
        })
    }
}

static FONT_DATA: Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets/fonts");

fn load_font_data(selection: Font) -> anyhow::Result<Vec<u8>> {
    let glob = format!("**/{}", selection.as_filename());

    let error = io::Error::new(
        io::ErrorKind::NotFound, 
        "Unable to load required fonts."
    );

    let file = FONT_DATA.find(glob.as_str())?.next().unwrap();
    let file = file.as_file().ok_or(error)?;

    Ok(file.contents().to_vec())
}

static FONT_FAMILY: Lazy<egui::FontFamily> = Lazy::new(|| {
    egui::FontFamily::Name("IPA".into())
});

#[allow(dead_code)]
pub static FONT_ID: Lazy<egui::FontId> = Lazy::new(|| egui::FontId {
    size: 16.,
    family: FONT_FAMILY.to_owned()
});

fn load_fonts(selection: Font) -> anyhow::Result<egui::FontDefinitions> {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        format!("{}", FONT_SELECTION),
        egui::FontData::from_owned(load_font_data(selection)?)
    );

    fonts.families.insert(
        FONT_FAMILY.to_owned(), 
        vec![format!("{}", FONT_SELECTION)]
    );

    Ok(fonts)
}

const FONT_SELECTION: Font = Font::GentiumPlus;

impl App {
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        match load_fonts(FONT_SELECTION) {
            Ok(fonts) => cc.egui_ctx.set_fonts(fonts),
            Err(error) => {
                log::error!("{}", error);

                // TODO: Ensure that all Drop occur as they should
                // Handle this termination behavior better
                panic!();
            }
        }
        
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

        egui::TopBottomPanel::bottom(new_id()).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(STATUS.lock().as_str());
            });
        });
    }
}
