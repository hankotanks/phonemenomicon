use std::sync::atomic::{AtomicUsize, Ordering};

use crate::app::FONT_ID;
use crate::types::{PhonemeQuality, Phoneme};
use crate::types::category::{Outer, Inner, Pair, CategoryColor};

pub fn cell_color<A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor>(
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

pub fn new_id() -> egui::Id { 
    static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

    let value = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    
    egui::Id::new(value)
}

pub fn draw_phoneme_cell<A, B, C>(
    ui: &mut egui::Ui, 
    phoneme: &Phoneme, 
    quality: Option<PhonemeQuality<A, B, C>>) where 
    A: Outer<B, C>, B: Inner<C>, C: Pair + CategoryColor {

    ui.painter().rect_filled(
        { 
            let mut rect = ui.available_rect_before_wrap();

            rect.set_bottom(rect.bottom() - ui.style().spacing.item_spacing.y);
            rect
        }, 
        0., cell_color(ui, quality));

    let content = egui::RichText::new(format!("{}", phoneme))
        .font(FONT_ID.to_owned())
        .background_color(egui::Color32::TRANSPARENT);
    ui.label(content); 
}