use std::rc;

use egui_extras::Size;
use petgraph::stable_graph::NodeIndex;

use crate::{pane::Pane, types::Language};

pub struct DialectPane {
    current: String,
    renaming: Option<NodeIndex<u32>>
}

impl DialectPane {
    pub fn new() -> Self {
        Self {
            current: String::from(""),
            renaming: None
        }
    }
}

fn show_dialect(
    ui: &mut egui::Ui, 
    state: &mut crate::State,
    id: NodeIndex<u32>,
    renaming: &mut Option<NodeIndex<u32>>,
    current: &mut String) {

    let dialect_name = state.dialects[state.language_tree[id]].name.clone();

    let dialect_text_id = egui::Id::from("dialect-view-rename");

    match renaming {
        Some(active_id) if *active_id == id => {
            let dialect_text = egui::TextEdit::singleline(current).id(dialect_text_id);

            if ui.add(dialect_text).lost_focus() {
                let _ = renaming.take();

                state.dialects[state.language_tree[id]].name = //
                    rc::Rc::from(current.as_str());

                current.clear();
            }
        },
        _ => {
            let mut dialect_button = egui::Button::new(dialect_name.as_ref());

            if state.inventory == state.language_tree[id] {
                let dialect_button_color = ui.style().visuals.hyperlink_color;
                dialect_button = dialect_button.fill(dialect_button_color);
            }

            let response = ui.add(dialect_button);

            let dialect_button_id = response.id;
        
            if response.clicked() {
                state.inventory = state.language_tree[id];
            }
        
            response.context_menu(|ui| {
                if ui.button("Rename").clicked() {
                    let _ = renaming.insert(id);
        
                    current.push_str(dialect_name.as_ref());

                    ui.close_menu();

                    ui.memory_mut(|mem| {
                        mem.surrender_focus(dialect_button_id);
                        mem.request_focus(dialect_text_id);
                    });
                }

                if ui.button("Add Dialect").clicked() {
                    let new_id = state.dialects.insert(Language::default());
                    let new_id = state.language_tree.add_node(new_id);

                    state.language_tree.add_edge(id, new_id, ());

                    ui.close_menu();

                    // Clear text box target
                    let _ = renaming.take();
                    
                    // Clear current string
                    current.clear();
                }
            });
        }
    }

    let count = state.language_tree
        .neighbors_directed(id, petgraph::Outgoing)
        .count();
    
    let subdialects: Vec<NodeIndex<u32>> = state.language_tree
        .neighbors_directed(id, petgraph::Outgoing)
        .collect();

    egui_extras::StripBuilder::new(ui)
        .sizes(Size::remainder(), count)
        .horizontal(|mut strip| {
            for id in subdialects.into_iter() {
                strip.cell(|ui| {
                    show_dialect(ui, state, id, 
                        renaming, current);
                });
            }
        });
}

impl Pane for DialectPane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new("Dialects")
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {
        log::info!("test");

        ui.vertical_centered_justified(|ui| {
            show_dialect(ui, state, state.root, 
                &mut self.renaming, &mut self.current);
        });

        
    }
}