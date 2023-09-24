use std::collections::{HashSet, HashMap};
use std::ops;

use egui_extras::{StripBuilder, Size, Strip};
use enum_iterator::{cardinality, all};
use slotmap::SlotMap;

use crate::app::FONT_ID;
use crate::pane::Pane;
use crate::types::{Alphabet, Phoneme, PhonemeQuality};
use crate::types::category::{Outer, Inner, Pair, CategoryColor};
use crate::pane::language::LanguagePaneRole;

fn cell_color<A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor>(
    ui: &egui::Ui,
    quality: Option<PhonemeQuality<A, B, C>>) -> egui::Color32 {

    let background = ui.visuals().window_fill;

    match quality {
        Some(quality) => {
            let PhonemeQuality(_, _, c) = quality;

            if c.len() > 1 {
                egui::Color32::LIGHT_GRAY
            } else {
                use egui::Rgba;

                let color = c[0].as_color();
                egui::lerp(Rgba::from(color)..=Rgba::from(background), 0.6).into()
            }
        },
        None => background
    }
}

#[allow(unused_variables)]
fn cell_context<A: Outer<B, C>, B: Inner<C>, C: Pair>(
    ui: &mut egui::Ui,
    inventory: &Alphabet<A, B, C>,
    phoneme: Phoneme) {

    todo!("Need to implement inventory context menu.");
}

fn cell_populated<A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor>(
    ui: &mut egui::Ui,
    role: &mut InventoryPaneRole<'_, '_, A, B, C>,
    phoneme: Phoneme) {

    let inventory = match role {
        InventoryPaneRole::Source { phonemes, .. } => phonemes,
        InventoryPaneRole::Display { inventory } => inventory,
    };

    let quality = inventory.get_quality(phoneme.id());

    ui.painter().rect_filled(
        ui.available_rect_before_wrap(), 
        0., cell_color(ui, quality));

    #[allow(unused_variables)]
    let (response, source) = match role {
        InventoryPaneRole::Source { inventory, phonemes } => {
            let button_content: egui::RichText = phoneme.clone().into();
            let button = egui::Button::new(button_content)
                .fill(egui::Color32::TRANSPARENT)
                .min_size(ui.available_size_before_wrap())
                .wrap(false);
            
            let response = ui.add(button);

            if response.clicked() {
                // TODO: I think this unwrap is safe, should double check
                let quality = phonemes.get_quality(phoneme.id()).unwrap();

                inventory.add_phoneme(phoneme.id(), quality);
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
                cell_context(ui, inventory, phoneme.clone());
            });

            (response, LanguagePaneRole::Inventory)
        },
    };

    if response.hovered() && ui.input(|input| 
        input.key_pressed(egui::Key::Space)) {

        todo!("Add `phoneme` and `source` to buffer (not yet implemented).")
    }
}

fn cell<A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor>(
    strip: &mut Strip<'_, '_>, 
    role: &mut InventoryPaneRole<'_, '_, A, B, C>,
    occurrence: Option<Phoneme>) {
    
    match occurrence {
        Some(symbol) => strip.cell(|ui| {
            cell_populated(ui, role, symbol);
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
    Display { inventory: &'a Alphabet<A, B, C> }
}

pub struct InventoryPane<'a, 'b, A: Outer<B, C>, B: Inner<C>, C: Pair> {
    pub role: InventoryPaneRole<'a, 'b, A, B, C>
}

impl<'a, 'b, A, B, C> InventoryPane<'a, 'b, A, B, C>
    where A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor {

    pub fn display(
        &mut self, 
        invalid: Phoneme, 
        space: Phoneme, 
        phonemes: &SlotMap<slotmap::DefaultKey, Phoneme>, 
        ui: &mut egui::Ui) {

        let original_spacing = ui.style().spacing.clone();

        {
            let spacing = &mut ui.style_mut().spacing;

            spacing.item_spacing = [0., 0.].into();
            spacing.window_margin = egui::Vec2::from([0., 0.]).into();
        }

        let cell_column_count = cardinality::<B>() * cardinality::<C>();

        let cell_proportion = (cell_column_count as f32).recip();

        StripBuilder::new(ui)
            .sizes(Size::remainder(), cardinality::<A>() + 1)
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
                        let inventory = match &self.role {
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
                                    cell(&mut strip, &mut self.role, occurrence.0);
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

    fn setup<'c, 'd: 'c>(&'c mut self, _ctx: &egui::Context) -> egui::Window<'d> {
        egui::Window::new(egui::RichText::default())
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {
        self.display(
            state.invalid.clone(), 
            state.space.clone(), 
            &state.phonemes,
            ui
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