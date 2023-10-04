use std::rc;

use egui_extras::Size;
use petgraph::stable_graph::{NodeIndex, StableGraph};

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

fn show_dialect_internal(
    ui: &mut egui::Ui, 
    state: &mut crate::State,
    id: NodeIndex<u32>,
    renaming: &mut Option<NodeIndex<u32>>,
    current: &mut String) -> egui::Pos2 {

    let dialect_name = state.dialects[state.language_tree[id]].name.clone();

    let dialect_text_id = egui::Id::from("dialect-view-rename");

    match renaming {
        Some(active_id) if *active_id == id => {
            let dialect_text = egui::TextEdit::singleline(current).id(dialect_text_id);
    
            let response = ui.add(dialect_text);

            if response.lost_focus() {
                let _ = renaming.take();
    
                state.dialects[state.language_tree[id]].name = //
                    rc::Rc::from(current.as_str());
    
                current.clear();
            }

            response.rect.center()
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

            let dialect_button_center = response.rect.center();
        
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

            dialect_button_center
        }
    }
}

fn leaves(language_tree: &StableGraph<slotmap::DefaultKey, (), petgraph::Directed>, id: NodeIndex) -> usize {
    if language_tree.neighbors_directed(id, petgraph::Outgoing).count() == 0 {
        return 1;
    }

    let mut leaf_count = 0;
    for child in language_tree.neighbors_directed(id, petgraph::Outgoing) {
        leaf_count += leaves(language_tree, child);
    }

    leaf_count
}

fn show_connection(ui: &mut egui::Ui, from: egui::Pos2, to: egui::Pos2) {
    ui.painter().line_segment([from, to], {
        let mut stroke = ui.visuals().window_stroke;

        stroke.width += 0.5;
        stroke
    })
}

fn show_dialect(
    ui: &mut egui::Ui, 
    state: &mut crate::State,
    id: NodeIndex<u32>,
    renaming: &mut Option<NodeIndex<u32>>,
    current: &mut String,
    cell_width: f32) -> egui::Pos2 {

    let row_height = ui.style().text_styles[&egui::TextStyle::Button].size;
    let row_height = row_height + ui.style().spacing.item_spacing.y * 2.;

    let mut builder = egui_extras::StripBuilder::new(ui);

    let mut count = 0;
    for child in state.language_tree.neighbors_directed(id, petgraph::Outgoing) {
        let size = row_height * leaves(&state.language_tree, child) as f32;
        builder = builder.size(Size::initial(size).at_least(size));

        count += 1;
    }

    if count == 0 {
        builder = builder.size(Size::initial(row_height).at_least(row_height));
    }

    let mut pos = egui::Pos2::default();
    builder.vertical(|mut strip| {
        let neighbors: Vec<NodeIndex<u32>> = state.language_tree
            .neighbors_directed(id, petgraph::Outgoing)
            .collect();

        strip.cell(|ui| {
            pos = match neighbors.first() {
                Some(child) => {
                    let mut temp = egui::Pos2::default();
                    egui_extras::StripBuilder::new(ui)
                        .sizes(Size::initial(cell_width).at_least(cell_width), 2)
                        .horizontal(|mut strip| {
                            strip.cell(|ui| { 
                                temp = show_dialect_internal(ui, state, id, renaming, current);
                            });

                            strip.cell(|ui| {
                                let to = show_dialect(ui, state, *child, renaming, current, cell_width); 
                                show_connection(ui, temp, to);
                            });
                        });

                    temp
                },
                None => show_dialect_internal(ui, state, id, renaming, current)
            };
        });

        for child in neighbors.into_iter().skip(1) {
            strip.cell(|ui| {
                egui_extras::StripBuilder::new(ui)
                    .sizes(Size::remainder(), 2)
                    .horizontal(|mut strip| {
                        strip.empty();

                        strip.cell(|ui| { 
                            let to = show_dialect(ui, state, child, renaming, current, cell_width); 
                            show_connection(ui, pos, to);
                        });
                    });
            });
        }
    });

    pos
}

fn depth(
    language_tree: &StableGraph<slotmap::DefaultKey, (), petgraph::Directed>, 
    id: NodeIndex<u32>) -> usize {

    if language_tree.neighbors_directed(id, petgraph::Outgoing).count() == 0 {
        return 1;
    }

    let mut max_height = 0;
    for child in language_tree.neighbors_directed(id, petgraph::Outgoing) {
        max_height = max_height.max(depth(language_tree, child));
    }

    max_height + 1
}

impl Pane for DialectPane {
    fn setup<'a, 'b: 'a>(&'a mut self, _ctx: &egui::Context) -> egui::Window<'b> {
        egui::Window::new("Dialects").constrain(true)
    }

    fn show(&mut self, state: &mut crate::State, ui: &mut egui::Ui) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            let depth = depth(&state.language_tree, state.root);
            let cell_width = ui.available_width() / depth as f32;

            show_dialect(ui, state, state.root,
                &mut self.renaming, &mut self.current, cell_width);
        });

        
    }
}