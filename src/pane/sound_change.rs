use std::mem;

use egui::RichText;
use egui_extras::Size;
use enum_map::EnumMap;

use crate::app::{FONT_ID, STATUS};

use crate::pane;
use crate::pane::language::LanguagePaneRole;

use crate::state::Selection;
use crate::types::{CONSONANT, VOWEL, PhonemeQuality};

use crate::types::category::{Articulation, Region, Voicing};
use crate::types::category::{Constriction, Place, Rounding};

use crate::pane::util;

use super::context::Context;

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
    current: EnumMap<SoundChangeRequest, Option<Selection>>
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
        request: SoundChangeRequest,
        buffer: Option<Selection>,
        buffer_state: &mut bool) -> Option<egui::Response> {

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

        let mut response = None;
        egui_extras::StripBuilder::new(ui)
            .size(Size::exact(FONT_ID.size * 2.))
            .horizontal(|mut strip| { strip.cell(|ui| {
                if let Some(selection) = buffer {
                    let Selection { phoneme, quality, .. } = selection;
        
                    let bg_color = if mem::discriminant(&phoneme.phone) == CONSONANT {
                        let quality: PhonemeQuality<Articulation, Region, Voicing> = PhonemeQuality::from_raw(quality);

                        util::cell_color(ui, Some(quality))
                    } else if mem::discriminant(&phoneme.phone) == VOWEL {
                        let quality: PhonemeQuality<Constriction, Place, Rounding> = PhonemeQuality::from_raw(quality);

                        util::cell_color(ui, Some(quality))
                    } else {
                        unreachable!();
                    };
        
                    let content = format!("{}", phoneme);
                    let content = RichText::new(content)
                        .font(FONT_ID.to_owned())
                        .background_color(egui::Color32::TRANSPARENT);

                    let mut rect = ui.available_rect_before_wrap();
                    
                    *(rect.bottom_mut()) += ui.style().spacing.item_spacing.y;
        
                    ui.painter().rect_filled(rect, 0., bg_color);
                    ui.vertical_centered(|ui| {
                        let content = egui::Label::new(content)
                            .wrap(false)
                            .sense(egui::Sense::click());

                        let _ = response.insert(ui.add(content));
                    });
                }
            })});

        response
    }
}

impl pane::Pane for SoundChangePane {
    fn title(&self, _state: &crate::State) -> std::rc::Rc<str> {
        std::rc::Rc::from("Sound Changes")
    }

    fn setup<'a, 'b: 'a>(&'a mut self, state: &crate::State, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new(self.title(state).as_ref())
    }

    fn show(&mut self, _windowed: bool, state: &mut crate::State, ui: &mut egui::Ui) {
        // Collect from the buffer
        if let Some(request) = self.request {
            if let Some(buffer_contents) = state.buffer.take() {
                let Selection { ref source, .. } = buffer_contents;

                let mut advance = true;
                if request.is_valid_source(*source) {
                    self.current[request] = Some(buffer_contents);
                } else {
                    // TODO: This else statement assumes that there will never
                    // be another variant of SoundChangeRequest added
                    // I never intend to, but it should be noted.
                    let mut status = STATUS.lock();
                    status.clear();
                    status.push_str("Sound change's source must be selected from the inventory. Select another phoneme.");

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
                    // TODO: The clones in this section are not great...

                    self.sound_change_field(
                        ui,
                        SoundChangeRequest::Src,
                        self.current[SoundChangeRequest::Src].clone(), 
                        &mut state.buffer_state
                    );
        
                    let response = self.sound_change_field(
                        ui,
                        SoundChangeRequest::Dst,
                        self.current[SoundChangeRequest::Dst].clone(), 
                        &mut state.buffer_state
                    );

                    if let Some(selection) = &mut self.current[SoundChangeRequest::Dst] {
                        let Selection { phoneme, quality, .. } = selection;

                        response.unwrap().context_menu(|ui| {
                            if mem::discriminant(&phoneme.phone) == CONSONANT {
                                let quality: PhonemeQuality<Articulation, Region, Voicing> = PhonemeQuality::from_raw(quality.clone());
                                let context = Context::Free { quality, phoneme };
        
                                pane::context::cell_context(ui, &state.ipa, &mut state.phonemes, context);
                            } else if mem::discriminant(&phoneme.phone) == VOWEL {
                                let quality: PhonemeQuality<Constriction, Place, Rounding> = PhonemeQuality::from_raw(quality.clone());
                                let context = Context::Free { quality, phoneme };
    
                                pane::context::cell_context(ui, &state.ipa, &mut state.phonemes, context);
                            } else {
                                unreachable!();
                            };
                        });
                    }
                });
            });
        });
    }
}