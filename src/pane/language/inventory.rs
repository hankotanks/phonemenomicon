use std::collections::{HashSet, HashMap};
use std::{ops, mem};

use egui_extras::{StripBuilder, Size, Strip};
use enum_iterator::{cardinality, all};
use slotmap::SlotMap;

use crate::app::FONT_ID;
use crate::pane::Pane;
use crate::pane::language::context;
use crate::types::{Alphabet, Phoneme, CONSONANT, VOWEL, Language};
use crate::types::category::{Outer, Inner, Pair, CategoryColor};
use crate::types::category::{Articulation, Region, Voicing, Constriction, Place, Rounding};
use crate::pane::language::LanguagePaneRole;
use crate::pane::util;

#[allow(unused_variables)]
fn cell_context<A: Outer<B, C>, B: Inner<C>, C: Pair>(
    ui: &mut egui::Ui,
    inventory: &mut Alphabet<A, B, C>,
    ipa: &Language,
    phonemes: &mut SlotMap<slotmap::DefaultKey, Phoneme>,
    phoneme: Phoneme) {

    if mem::discriminant(&phoneme.phone) == CONSONANT {
        let inventory = unsafe {
            type Dst = Alphabet<Articulation, Region, Voicing>;
            mem::transmute::<&Alphabet<A, B, C>, &Dst>(inventory)
        };

        let quality = inventory.get_quality(phoneme.id()).unwrap();
        for diacritics in context::diacritics::modifiers_consonants(
            phonemes, &ipa.consonants, quality) {
            
            let phoneme = &mut phonemes[phoneme.id()];
            context::diacritics_display(ui, diacritics, inventory, phoneme);
        }
    } else if mem::discriminant(&phoneme.phone) == VOWEL {
        let inventory = unsafe {
            type Dst = Alphabet<Constriction, Place, Rounding>;
            mem::transmute::<&Alphabet<A, B, C>, &Dst>(inventory)
        };

        let quality = inventory.get_quality(phoneme.id()).unwrap();
        for diacritics in context::diacritics::modifiers_vowels(
            phonemes, &ipa.vowels, quality) {
            
            let phoneme = &mut phonemes[phoneme.id()];
            context::diacritics_display(ui, diacritics, inventory, phoneme);
        }
    } else {
        unreachable!();
    }

    let content = egui::RichText::new("Remove Phoneme").italics();

    if ui.button(content).clicked() {
        phonemes.remove(phoneme.id());

        inventory.remove_phoneme(phoneme.id());

        ui.close_menu();
    }
}

fn cell_populated<A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor>(
    windowed: bool,
    ui: &mut egui::Ui,
    role: &mut InventoryPaneRole<'_, '_, A, B, C>,
    ipa: &Language, 
    phonemes: &mut SlotMap<slotmap::DefaultKey, Phoneme>,
    buffer: &mut Option<(slotmap::DefaultKey, LanguagePaneRole)>,
    buffer_state: bool,
    phoneme: Phoneme) {

    let inventory: &Alphabet<A, B, C> = match role {
        InventoryPaneRole::Source { phonemes, .. } => phonemes,
        InventoryPaneRole::Display { inventory } => inventory,
    };

    let quality = inventory.get_quality(phoneme.id());

    ui.painter().rect_filled(
        if windowed { 
            ui.available_rect_before_wrap() 
        } else {
            let mut rect = ui.available_rect_before_wrap();
            (*rect.bottom_mut()) += ui.style().spacing.item_spacing.y;
            rect
        }, 
        0., util::cell_color(ui, quality));

    #[allow(unused_variables)]
    let (response, source) = match role {
        InventoryPaneRole::Source { inventory, phonemes: source } => {
            let button_content: egui::RichText = phoneme.clone().into();
            let button = egui::Button::new(button_content)
                .fill(egui::Color32::TRANSPARENT)
                .min_size(ui.available_size_before_wrap())
                .wrap(false);
            
            let response = ui.add(button);

            if response.clicked() && !buffer_state {
                // TODO: I think this unwrap is safe, should double check
                let quality = source.get_quality(phoneme.id()).unwrap();

                let phoneme = Phoneme::new(
                    String::from(phoneme.symbol.as_str()), 
                    phoneme.phone.clone()
                );

                let id = phonemes.insert(phoneme);

                phonemes[id].set_id(id);

                inventory.add_phoneme(id, quality);
            }

            (response, LanguagePaneRole::Ipa)
        },
        InventoryPaneRole::Display { inventory } => {
            let contents = format!("{}", phoneme);
            let contents = if !phoneme.grapheme.is_empty() && contents != phoneme.grapheme {
                format!("{} [{}]", contents, phoneme.grapheme)
            } else {
                contents
            };

            let contents = egui::RichText::new(contents)
                .font(FONT_ID.to_owned());

            let button = egui::Button::new(contents)
                .fill(egui::Color32::TRANSPARENT)
                .min_size(ui.available_size_before_wrap())
                .wrap(false);

            let response = ui.add(button).context_menu(|ui| {
                cell_context::<A, B, C>(ui, inventory, ipa, phonemes, phoneme.clone());
            });

            (response, LanguagePaneRole::Inventory)
        },
    };

    if response.clicked() && buffer_state {
        let _ = buffer.insert((phoneme.id(), source));
    }
}

fn cell<A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor>(
    windowed: bool,
    strip: &mut Strip<'_, '_>, 
    role: &mut InventoryPaneRole<'_, '_, A, B, C>,
    ipa: &Language,
    phonemes: &mut SlotMap<slotmap::DefaultKey, Phoneme>,
    buffer: &mut Option<(slotmap::DefaultKey, LanguagePaneRole)>,
    buffer_state: bool,
    occurrence: Option<Phoneme>) {
    
    match occurrence {
        Some(symbol) => strip.cell(|ui| {
            cell_populated(windowed, ui, role, 
                ipa, phonemes, buffer, buffer_state, symbol);
        }),
        None => strip.empty()
    }
}

pub enum InventoryPaneRole<'a: 'b, 'b: 'a, A: Outer<B, C>, B: Inner<C>, C: Pair> {
     // Writes to inventory
     Source {
        inventory: &'a mut Alphabet<A, B, C>,
        phonemes: &'b Alphabet<A, B, C>
    },
    // Reads from inventory
    Display { inventory: &'a mut Alphabet<A, B, C> }
}

pub struct InventoryPane<'a, 'b, A: Outer<B, C>, B: Inner<C>, C: Pair> {
    pub role: InventoryPaneRole<'a, 'b, A, B, C>
}

impl<'a, 'b, A, B, C> InventoryPane<'a, 'b, A, B, C>
    where A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor {

    pub fn display(
        &mut self, 
        windowed: bool,
        ui: &mut egui::Ui,
        invalid: Phoneme, 
        space: Phoneme, 
        phonemes: &mut SlotMap<slotmap::DefaultKey, Phoneme>, 
        buffer: &mut Option<(slotmap::DefaultKey, LanguagePaneRole)>,
        buffer_state: bool,
        ipa: &Language) {

        let original_spacing = ui.style().spacing.clone();

        if windowed {
            let spacing = &mut ui.style_mut().spacing;

            spacing.item_spacing = [0., 0.].into();
            spacing.window_margin = egui::Vec2::from([0., 0.]).into();
        } else {
            let spacing = &mut ui.style_mut().spacing;

            spacing.item_spacing.x = 0.;
        }

        let cell_column_count = cardinality::<B>() * cardinality::<C>();

        let cell_proportion = (cell_column_count as f32).recip();

        let size = if windowed { 
            Size::remainder()
        } else {
            let size = FONT_ID.size;
            let size = size + ui.style().spacing.button_padding.y * 2.;
            let size = size + ui.style().spacing.item_spacing.y * 2.;

            Size::exact(size)
        };

        StripBuilder::new(ui)
            .sizes(size, cardinality::<A>() + 1)
            .vertical(|mut strip| {
                strip.strip(|builder| {
                    builder
                        .sizes(Size::remainder(), cardinality::<B>())
                        .horizontal(|mut strip| {
                            for b in all::<B>() {
                                strip.cell(|ui| {
                                    ui.label(format!("{}", b));
                                });
                            }
                        });
                });

                for a in all::<A>() {
                    let occurrences = {
                        let inventory: &Alphabet<A, B, C> = match &self.role {
                            InventoryPaneRole::Source { phonemes, .. } => phonemes,
                            InventoryPaneRole::Display { inventory } => inventory,
                        };

                        occurrences(invalid.clone(), space.clone(), phonemes, inventory, a)
                    };

                    strip.strip(|mut builder| {
                        let mut sum = 0;
                        for (_, count) in occurrences.iter() {
                            sum += count;

                            let size = if sum == cell_column_count {
                                Size::remainder()
                            } else {
                                let cell_proportion = cell_proportion * *count as f32;
                                Size::relative(cell_proportion)
                            };

                            builder = builder.size(size);
                        }

                        builder.horizontal(|mut strip| {
                            occurrences
                                .into_iter()
                                .for_each(|occurrence| {
                                    cell(windowed, &mut strip, &mut self.role, ipa, 
                                        phonemes, buffer, buffer_state, occurrence.0);
                                });
                        });
                    });
                }
            });

        ui.style_mut().spacing = original_spacing;
    }
}

impl<'a, 'b, A, B, C> Pane for InventoryPane<'a, 'b, A, B, C> 
    where A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor {

    fn title(&self, _state: &crate::State) -> std::rc::Rc<str> {
        // TODO: I would like this to read "Consonants" or "Vowels" depending on the generics
        std::rc::Rc::from("Inventory")
    }
    
    fn setup<'c, 'd: 'c>(&'c mut self, state: &crate::State, _ctx: &egui::Context) -> egui::Window<'d> {
        egui::Window::new(self.title(state).as_ref())
    }

    fn show(&mut self, windowed: bool, state: &mut crate::State, ui: &mut egui::Ui) {
        self.display(
            windowed,
            ui,
            state.invalid.clone(), 
            state.space.clone(), 
            &mut state.phonemes,
            &mut state.buffer,
            state.buffer_state,
            &state.ipa
        );
    }
}

pub fn repeating_replace(
    symbols: &str, 
    phoneme: &str, 
    substring_bounds: ops::Range<usize>) -> String {
    let mut temp = String::new();

    let ops::Range { start, end } = substring_bounds;

    temp.push_str(&symbols[0..start]);
    temp.push_str(&phoneme.repeat(end - start));
    temp.push_str(&symbols[end..]);
    temp
}

pub fn repeating_replace_in_place(
    symbols: &mut String,
    phoneme: &str,
    substring_bounds: ops::Range<usize>) {
    let temp = repeating_replace(symbols, phoneme, substring_bounds);
    
    symbols.clear();
    symbols.push_str(temp.as_str());
}


fn occurrences<A: Outer<B, C>, B: Inner<C>, C: Pair>(
    invalid: Phoneme,
    space: Phoneme,
    phonemes: &SlotMap<slotmap::DefaultKey, Phoneme>, 
    alphabet: &Alphabet<A, B, C>, 
    a: A) -> Vec<(Option<Phoneme>, usize)> {
    
    let mut sounds = HashSet::new();
    let mut symbol_chart = String::with_capacity(cardinality::<B>() * cardinality::<C>());

    for b in all::<B>() {
        for c in all::<C>() {
            let id = alphabet.get((a, b, c));

            let symbol = match id.map(|id| phonemes[id].clone()) {
                Some(symbol) => {
                    sounds.insert(symbol.clone());
                    symbol
                },
                None => space.clone()
            };

            symbol_chart.push_str(format!("{}", symbol).as_str());
        }
    }

    for phoneme in sounds.iter() {
        let re_fmt = format!("{}( +){}", phoneme, phoneme);
        let re = regex::Regex::new(&re_fmt).unwrap();

        let mut locs = re.capture_locations();
        while let Some(_capture) = re.captures_read(&mut locs, &symbol_chart) {
            if let Some((start, end)) = locs.get(1) {
                repeating_replace_in_place(
                    &mut symbol_chart, 
                    format!("{}", phoneme).as_str(), 
                    start..end);
            } else {
                break;
            }
        }
    }

    sounds.insert(space.clone());

    let mut temp_symbol_occurrence = HashMap::new();

    for phoneme in sounds.iter() {
        let re_fmt = format!("(({})+)", phoneme);
        let re = regex::Regex::new(&re_fmt).unwrap();

        let mut locs = re.capture_locations();
        while let Some(capture) = re.captures_read_at(&mut locs, &symbol_chart, 0) {
            if let Some((start, end)) = locs.get(1) {
                let capture = capture.as_str();
                let symbol = if capture.starts_with(' ') {
                    None
                } else { 
                    Some(phoneme.clone()) 
                };

                // NOTE: This should always divide cleanly, 
                // don't worry about integer arithmetic
                let span = capture.len() / format!("{}", phoneme).len();
                let occurrence = (symbol, end, span);
                temp_symbol_occurrence.insert(start, occurrence);

                let temp = repeating_replace(
                    &symbol_chart, 
                    format!("{}", invalid).as_str(), 
                    start..end);

                symbol_chart.clear();
                symbol_chart.push_str(temp.as_str());
            } else {
                break;
            }
        }
    }

    let mut symbol_occurrence = Vec::new();

    let mut curr = 0;
    while let Some((symbol, end, span)) = temp_symbol_occurrence.get(&curr) {
        symbol_occurrence.push((symbol.clone(), *span, ));

        curr = *end;
    }

    symbol_occurrence
}