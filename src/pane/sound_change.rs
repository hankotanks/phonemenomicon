use std::mem;

use egui::RichText;
use egui_extras::{Size, Column};
use enum_map::EnumMap;
use petgraph::stable_graph::NodeIndex;
use slotmap::SlotMap;

use crate::app::{FONT_ID, STATUS};

use crate::pane;
use crate::pane::language::LanguagePaneRole;

use crate::state::Selection;
use crate::types::{CONSONANT, VOWEL, PhonemeQuality, SoundChange, SoundChangeContext, Language, Phoneme};

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
    current: EnumMap<SoundChangeRequest, Option<Selection>>,
    dialect: Option<NodeIndex<u32>>
}

impl SoundChangePane {
    pub fn new() -> Self {
        Self {
            request: None,
            current: EnumMap::default(),
            dialect: None
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

fn show_sound_change(
    mut row: egui_extras::TableRow<'_, '_>, 
    phonemes: &SlotMap<slotmap::DefaultKey, Phoneme>,
    parent: &Language, 
    child: &Language, 
    sound_change: &SoundChange) -> egui::Response {

    let SoundChange { src, dst, .. } = sound_change;

    row.col(|ui| {
        let phoneme = &phonemes[*src];
        
        let discriminant = mem::discriminant(&phoneme.phone);

        let cell_color = if discriminant == CONSONANT {
            let quality = parent.consonants.get_quality(*src);
            util::cell_color(ui, quality)
        } else if discriminant == VOWEL {
            let quality = parent.vowels.get_quality(*src);
            util::cell_color(ui, quality)
        } else {
            unreachable!();
        };
        
        ui.painter().rect_filled(
            { 
                let mut rect = ui.available_rect_before_wrap();

                rect.set_bottom(rect.bottom() - ui.style().spacing.item_spacing.y);
                rect
            },
            0., cell_color);

        let content = egui::RichText::new(format!("{}", phoneme))
            .font(FONT_ID.to_owned())
            .background_color(egui::Color32::TRANSPARENT);

        ui.label(content); 
    });

    row.col(|ui| {
        let phoneme = &phonemes[*dst];
        
        let discriminant = mem::discriminant(&phoneme.phone);

        let cell_color = if discriminant == CONSONANT {
            let quality = child.consonants.get_quality(*dst);
            util::cell_color(ui, quality)
        } else if discriminant == VOWEL {
            let quality = child.vowels.get_quality(*dst);
            util::cell_color(ui, quality)
        } else {
            unreachable!();
        };
        
        ui.painter().rect_filled(
            { 
                let mut rect = ui.available_rect_before_wrap();

                rect.set_bottom(rect.bottom() - ui.style().spacing.item_spacing.y);
                rect
            },
            0., cell_color);

        let content = egui::RichText::new(format!("{}", phoneme))
            .font(FONT_ID.to_owned())
            .background_color(egui::Color32::TRANSPARENT);

        ui.label(content); 
    });

    let mut response = None;
    row.col(|ui| {
        response = Some(ui.button("Delete"));
    });

    response.unwrap()
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

                if request.is_valid_source(*source) {
                    self.current[request] = Some(buffer_contents);

                    // TODO: ESC should cancel selection
                    if let SoundChangeRequest::Src = request {
                        self.request = Some(SoundChangeRequest::Dst);
                    } else if let SoundChangeRequest::Dst = request {
                        self.request = None;
                    }
                } else {
                    // TODO: This else statement assumes that there will never
                    // be another variant of SoundChangeRequest added
                    // I never intend to, but it should be noted.
                    let mut status = STATUS.lock();
                    status.clear();
                    status.push_str("Sound change's source must be selected from the inventory. Select another phoneme.");
                }
            }
        }

        let panel_height = ui.style().spacing.item_spacing.y * 2.;
        let panel_height = panel_height + FONT_ID.size;

        egui::TopBottomPanel::top(util::new_id())
            .exact_height(panel_height)
            .show_inside(ui, |ui| {

            ui.horizontal(|ui| {
                let mut dialects = state.language_tree
                    .neighbors_directed(state.inventory_index, petgraph::Outgoing)
                    .peekable();
                
                let content = match dialects.peek() {
                    Some(..) => "Select target dialect",
                    None => "Selected language must have at least one dialect to apply sound changes",
                };

                ui.label(content);

                for child in dialects {
                    let content = state.dialects[state.language_tree[child]].name.clone();

                    ui.selectable_value(&mut self.dialect, Some(child), content.as_ref());
                }
            });

            let mut margin = egui::Margin::default();

            margin.bottom += ui.style().spacing.item_spacing.y * 2.;
            egui::Frame::none().outer_margin(margin).show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    // TODO: The clones in this section are not great...

                    let buffer = self.current[SoundChangeRequest::Src]
                        .clone()
                        .map(|mut selection| {
                        selection.phoneme = state.phonemes[selection.phoneme.id()].clone();
                        selection
                    });

                    self.sound_change_field(
                        ui,
                        SoundChangeRequest::Src,
                        buffer, 
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
                                let quality: PhonemeQuality<Articulation, Region, Voicing> = //
                                    PhonemeQuality::from_raw(quality.clone());

                                let context = Context::Free { quality, phoneme };
        
                                pane::context::cell_context(ui, &state.ipa, &mut state.phonemes, context);
                            } else if mem::discriminant(&phoneme.phone) == VOWEL {
                                let quality: PhonemeQuality<Constriction, Place, Rounding> = //
                                    PhonemeQuality::from_raw(quality.clone());
                                    
                                let context = Context::Free { quality, phoneme };
    
                                pane::context::cell_context(ui, &state.ipa, &mut state.phonemes, context);
                            } else {
                                unreachable!();
                            };
                        });
                    }

                    ui.separator();
                    
                    match self.dialect {
                        Some(dialect) if ui.button("Add").clicked() && 
                            self.current.iter().fold(true, |a, (_, b)| a && b.is_some())=> {
                            let sound_change = SoundChange {
                                src: self.current[SoundChangeRequest::Src].as_ref().unwrap().phoneme.id(),
                                dst: {
                                    let Selection { phoneme, quality, .. } = //
                                        self.current[SoundChangeRequest::Dst].as_ref().unwrap();

                                    // TODO: This could be a utility function, similar to `add_symbol_to_alphabet`
                                    let id = state.phonemes.insert(phoneme.clone());

                                    state.phonemes[id].set_id(id);

                                    let quality = quality.clone();

                                    let discriminant = mem::discriminant(&state.phonemes[id].phone);
                                    if discriminant == CONSONANT {
                                        let quality = PhonemeQuality::from_raw(quality);
                                        state.dialects[state.language_tree[dialect]].consonants.add_phoneme(id, quality);
                                    } else if discriminant == VOWEL {
                                        let quality = PhonemeQuality::from_raw(quality);
                                        state.dialects[state.language_tree[dialect]].vowels.add_phoneme(id, quality);
                                    } else {
                                        unreachable!();
                                    }

                                    id                                    
                                },
                                context: (SoundChangeContext::Unrestricted, SoundChangeContext::Unrestricted)
                            };

                            self.current[SoundChangeRequest::Src] = None;
                            self.current[SoundChangeRequest::Dst] = None;
                            
                            state.dialects[state.language_tree[dialect]].sound_changes.push(sound_change);
                        },
                        _ => { /*  */ },
                    }
                });
            });
        });

        if let Some(id) = self.dialect {
            let dialect = &state.dialects[state.language_tree[id]];

            let row_height = FONT_ID.size;
            let row_height = row_height + ui.style().spacing.item_spacing.y * 4.;
            let row_height = row_height + ui.style().spacing.button_padding.y * 2.;

            let mut deletion_queue = Vec::new();
            ui.vertical_centered(|ui| {
                egui_extras::TableBuilder::new(ui)
                    .columns(Column::remainder(), 3)
                    .vscroll(true)
                    .body(|body| {

                    body.rows(row_height, dialect.sound_changes.len(), 
                        |idx, row| {
                            let parent = &state.dialects[state.inventory];

                            let response = show_sound_change(row, &state.phonemes, parent, dialect, &dialect.sound_changes[idx]);

                            if response.clicked() {
                                deletion_queue.insert(0, idx);
                            }
                        }
                    );
                });
            });

            let dialect = &mut state.dialects[state.language_tree[id]];

            for idx in deletion_queue.drain(0..) {
                let id = dialect.sound_changes[idx].dst;

                let discriminant = mem::discriminant(&state.phonemes[id].phone);
                if discriminant == CONSONANT {
                    dialect.consonants.remove_phoneme(id);
                } else if discriminant == VOWEL {
                    dialect.vowels.remove_phoneme(id);
                } else {
                    unreachable!();
                }

                state.phonemes.remove(id);

                dialect.sound_changes.remove(idx);
            }
        }
    }

    fn on_dialect_change(&mut self, _state: &mut crate::State) {
        self.dialect = None;
    }
}