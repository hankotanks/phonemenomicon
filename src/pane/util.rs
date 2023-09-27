use crate::types::PhonemeQuality;
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