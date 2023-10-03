use std::mem;

use egui::RichText;
use egui_extras::Size;
use enum_map::EnumMap;
use slotmap::{DefaultKey, SlotMap};

use crate::app::{FONT_ID, STATUS};
use crate::pane::Pane;

use crate::pane::language::LanguagePaneRole;

use crate::types::{Phoneme, CONSONANT, Language};

use crate::pane::util;

#[derive(Clone, Copy, PartialEq, enum_map::Enum)]
enum SoundChangeRequest {
    Src,
    Dst
}

impl SoundChangeRequest {
    fn is_valid_source(&self, source: LanguagePaneRole) -> bool {
        match self {
            SoundChangeRequest::Src => matches!(source, LanguagePaneRole::Inventory),
            SoundChangeRequest::Dst => true
        }
    }
}

pub struct SoundChangePane {
    request: Option<SoundChangeRequest>,
    current: EnumMap<SoundChangeRequest, Option<(DefaultKey, LanguagePaneRole)>>
}

impl SoundChangePane {
    pub fn new() -> Self {
        Self {
            request: None,
            current: EnumMap::default()
        }
    }

    fn sound_change_field(
        &mut self,
        ui: &mut egui::Ui,
        phonemes: &SlotMap<DefaultKey, Phoneme>,
        request: SoundChangeRequest,
        buffer: Option<(DefaultKey, &Language)>,
        buffer_state: &mut bool) {

        let mut toggle_state = if let Some(inner_request) = self.request {
            inner_request == request
        } else {
            false
        };

        let content = match request {
            SoundChangeRequest::Src => "From",
            SoundChangeRequest::Dst => "To"
        };

        let content = RichText::from(content);

        if ui.toggle_value(&mut toggle_state, content).clicked() {
            if toggle_state { 
                let _ = self.request.insert(request);
            } else { 
                let _ = self.request.take();
            };

            *buffer_state = toggle_state;
        }

        egui_extras::StripBuilder::new(ui)
            .size(Size::exact(FONT_ID.size * 2.))
            .horizontal(|mut strip| { strip.cell(|ui| {
                if let Some((id, inventory)) = buffer {
                    let phoneme = &phonemes[id];
        
                    let bg_color = if mem::discriminant(&phoneme.phone) == CONSONANT {
                        util::cell_color(ui, inventory.consonants.get_quality(id))
                    } else {
                        util::cell_color(ui, inventory.vowels.get_quality(id))
                    };
        
                    let content = format!(" {}", phoneme);
                    let content = RichText::new(content)
                        .font(FONT_ID.to_owned())
                        .background_color(egui::Color32::TRANSPARENT);

                    let mut rect = ui.available_rect_before_wrap();
                    
                    *(rect.bottom_mut()) += ui.style().spacing.item_spacing.y;
        
                    ui.painter().rect_filled(rect, 0., bg_color);
                    ui.label(content);
                        
                }
            })});
        
    }
}

impl Pane for SoundChangePane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new("Sound Changes")
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {
        // Collect from the buffer
        if let Some(request) = self.request {
            if let Some(buffer_contents) = state.buffer.take() {
                let (_, source) = buffer_contents;

                let mut advance = true;
                if request.is_valid_source(source) {
                    self.current[request] = Some(buffer_contents);
                } else {
                    // TODO: This else statement assumes that there will never
                    // be another variant of SoundChangeRequest added
                    // I never intend to, but it should be noted.
                    let mut status = STATUS.lock();
                    status.clear();
                    status.push_str("Sound change's source phoneme must be selected from the inventory. Select another phoneme.");

                    advance = false;
                }
                
                // TODO: When not advancing to the next button,
                // Toggle button's don't lose focus after selection

                // TODO: ESC should cancel selection
                if let SoundChangeRequest::Src = request {
                    if advance && self.current[SoundChangeRequest::Dst].is_none() { 
                        self.request = Some(SoundChangeRequest::Dst); 
                    }
                } else if let SoundChangeRequest::Dst = request {
                    self.request = None;
                }
            }
        }

        let panel_height = ui.style().spacing.item_spacing.y * 2.;
        let panel_height = panel_height + FONT_ID.size;

        egui::TopBottomPanel::top(util::new_id())
            .exact_height(panel_height)
            .show_inside(ui, |ui| {

            let mut margin = egui::Margin::default();

            margin.bottom += ui.style().spacing.item_spacing.y * 2.;
            egui::Frame::none().outer_margin(margin).show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    self.sound_change_field(
                        ui, &state.phonemes,
                        SoundChangeRequest::Src,
                        self.current[SoundChangeRequest::Src].map(|(id, role)| match role {
                            LanguagePaneRole::Inventory => (id, &state.inventory),
                            LanguagePaneRole::Ipa => (id, &state.ipa),
                        }), &mut state.buffer_state
                    );
        
                    self.sound_change_field(
                        ui, &state.phonemes,
                        SoundChangeRequest::Dst,
                        self.current[SoundChangeRequest::Dst].map(|(id, role)| match role {
                            LanguagePaneRole::Inventory => (id, &state.inventory),
                            LanguagePaneRole::Ipa => (id, &state.ipa),
                        }), &mut state.buffer_state
                    );
                });
            });
        });
    }
}