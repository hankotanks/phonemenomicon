use egui_extras::{StripBuilder, Size};
use enum_iterator::{cardinality, all};
use slotmap::SlotMap;

use crate::pane::Pane;
use crate::types::{Alphabet, Phoneme};
use crate::types::category::{Outer, Inner, Pair};

pub enum InventoryPaneRole<'a, 'b, A: Outer<B, C>, B: Inner<C>, C: Pair> {
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
    where A: Outer<B, C>, B: Inner<C>, C: Pair {

    pub fn display(&mut self, phonemes: &SlotMap<slotmap::DefaultKey, Phoneme>, ui: &mut egui::Ui) {
        let original_spacing = ui.style().spacing.clone();

        {
            let spacing = &mut ui.style_mut().spacing;

            spacing.item_spacing = [0., 0.].into();
            spacing.window_margin = egui::Vec2::from([0., 0.]).into();
        }

        let inventory = match &self.role {
            InventoryPaneRole::Source { phonemes, .. } => phonemes,
            InventoryPaneRole::Display { inventory } => inventory,
        };

        let cell_column_count = cardinality::<B>() * cardinality::<C>();

        let cell_width = ui.available_width() / cell_column_count as f32;
        let cell_width_size = Size::remainder(); // Size::initial(cell_width).at_least(cell_width);

        // TODO: cell_height should be fixed, to the height of the IPA font's size
        // So, set up the font loader first
        let cell_height = ui.available_height() / cardinality::<A>() as f32;
        let cell_height_size = Size::remainder(); // Size::initial(cell_height).at_least(cell_height);

        StripBuilder::new(ui)
            .sizes(cell_height_size, cardinality::<A>())
            .vertical(|mut strip| {
                for a in all::<A>() {
                    strip.cell(|ui| {
                        StripBuilder::new(ui)
                            .sizes(cell_width_size, cell_column_count)
                            .horizontal(|mut strip| {
                                for b in all::<B>() {
                                    for c in all::<C>() {
                                        match inventory.get((a, b, c)) {
                                            Some(id) => {
                                                strip.cell(|ui| {
                                                    let content = format!("{}", phonemes[id]);
                                                    let _ = ui.button(content);
                                                });
                                            },
                                            None => strip.empty(),
                                        }
                                    }
                                }
                            });
                    })
                    
                }
            });

        ui.style_mut().spacing = original_spacing;
    }
}

impl<'a, 'b, A, B, C> Pane for InventoryPane<'a, 'b, A, B, C> 
    where A: Outer<B, C>, B: Inner<C>, C: Pair {

    fn setup<'c, 'd: 'c>(&'c mut self, _ctx: &egui::Context) -> egui::Window<'d> {
        egui::Window::new(egui::RichText::default())
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {
        self.display(&state.phonemes, ui);
    }
}