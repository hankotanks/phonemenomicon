use std::{hash, fmt};

use enum_iterator::Sequence;
use enum_map::Enum;
use serde::de;

pub trait Category: //
    Clone + Copy + //
    PartialEq + Eq + hash::Hash + //
    Sequence + Enum + //
    de::DeserializeOwned + serde::Serialize + //
    fmt::Display + 'static { /*  */ }

impl<T: //
    Clone + Copy + //
    PartialEq + Eq + hash::Hash + //
    Sequence + Enum + //
    de::DeserializeOwned + serde::Serialize + //
    fmt::Display + 'static> Category for T { /*  */ }

pub trait CategoryColor {
    fn as_color(&self) -> egui::Color32;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Articulation {
    Plosive,
    Nasal,
    Trill,
    Tap,
    Fricative,
    LateralFricative,
    Approximant,
    LateralApproximant
}

impl fmt::Display for Articulation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Articulation::*;
        write!(f, "{}", match self {
            Plosive => "Plosive",
            Nasal => "Nasal",
            Trill => "Trill",
            Tap => "Tap",
            Fricative => "Fricative",
            LateralFricative => "Lateral Fricative",
            Approximant => "Approximant",
            LateralApproximant => "Lateral Approximant"
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Region {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    PostAlveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Region::*;
        write!(f, "{}", match self {
            Bilabial => "Bilabial",
            Labiodental => "Labiodental",
            Dental => "Dental",
            Alveolar => "Alveolar",
            PostAlveolar => "Post Alveolar",
            Retroflex => "Retroflex",
            Palatal => "Palatal",
            Velar => "Velar",
            Uvular => "Uvular",
            Pharyngeal => "Pharyngeal",
            Glottal => "Glottal"
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Voicing {
    Unvoiced,
    Voiced
}

impl CategoryColor for Voicing {
    fn as_color(&self) -> egui::Color32 {
        match self {
            Voicing::Unvoiced => egui::Color32::GOLD,
            Voicing::Voiced => egui::Color32::RED
        }
    }
}

impl fmt::Display for Voicing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Voicing::*;
        write!(f, "{}", match self {
            Voiced => "Voiced",
            Unvoiced => "Unvoiced"
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Constriction {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open
}

impl fmt::Display for Constriction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Constriction::*;
        write!(f, "{}", match self {
            Close => "Close",
            NearClose => "Near-Close",
            CloseMid => "Close-Mid",
            Mid => "Mid",
            OpenMid => "Open-Mid",
            NearOpen => "Near-Open",
            Open => "Open"
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Place {
    Front,
    Central,
    Back
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Place::*;
        write!(f, "{}", match self {
            Front => "Front",
            Central => "Central",
            Back => "Back"
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Rounding {
    Unrounded,
    Rounded
}

impl CategoryColor for Rounding {
    fn as_color(&self) -> egui::Color32 {
        match self {
            Rounding::Unrounded => egui::Color32::LIGHT_BLUE,
            Rounding::Rounded => egui::Color32::BLUE
        }
    }
}

impl fmt::Display for Rounding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Rounding::*;
        write!(f, "{}", match self {
            Unrounded => "Unrounded",
            Rounded => "Rounded"
        })
    }
}