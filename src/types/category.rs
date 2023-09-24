use std::{hash, fmt};

use enum_iterator::Sequence;
use enum_map::{Enum, EnumArray, EnumMap};
use serde::de;

pub trait Category: //
    Clone + Copy + //
    PartialEq + Eq + hash::Hash + //
    Sequence + Enum + //
    de::DeserializeOwned + serde::Serialize + //
    fmt::Display + fmt::Debug + 'static { /*  */ }

impl<T: //
    Clone + Copy + //
    PartialEq + Eq + hash::Hash + //
    Sequence + Enum + //
    de::DeserializeOwned + serde::Serialize + //
    fmt::Display + fmt::Debug + 'static> Category for T { /*  */ }

pub trait Pair: Category + EnumArray<Option<slotmap::DefaultKey>> { /*  */ }

pub trait Inner<A>: Category + //
    EnumArray<EnumMap<A, Option<slotmap::DefaultKey>>> 
    where A: Pair { /*  */ }

pub trait Outer<A, B>: Category + //
    EnumArray<EnumMap<A, EnumMap<B, Option<slotmap::DefaultKey>>>> 
    where A: Inner<B>, B: Pair { /*  */ }

pub trait CategoryColor {
    fn as_color(&self) -> egui::Color32;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Articulation {
    Plosive,
    Nasal,
    Trill,
    Flap,
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
            Flap => "Tap",
            Fricative => "Fricative",
            LateralFricative => "Lateral Fricative",
            Approximant => "Approximant",
            LateralApproximant => "Lateral Approximant"
        })
    }
}

impl Outer<Region, Voicing> for Articulation { /*  */ }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Region {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Post,
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
            Post => "Post Alveolar",
            Retroflex => "Retroflex",
            Palatal => "Palatal",
            Velar => "Velar",
            Uvular => "Uvular",
            Pharyngeal => "Pharyngeal",
            Glottal => "Glottal"
        })
    }
}

impl Inner<Voicing> for Region { /*  */ }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Voicing {
    Voiceless,
    Voiced
}

impl CategoryColor for Voicing {
    fn as_color(&self) -> egui::Color32 {
        match self {
            Voicing::Voiceless => egui::Color32::GOLD,
            Voicing::Voiced => egui::Color32::RED
        }
    }
}

impl fmt::Display for Voicing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Voicing::*;
        write!(f, "{}", match self {
            Voiced => "Voiced",
            Voiceless => "Unvoiced"
        })
    }
}

impl Pair for Voicing { /*  */ }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[derive(Sequence, Enum)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Constriction {
    Close,
    CloseNear,
    CloseMid,
    Mid,
    OpenMid,
    OpenNear,
    Open
}

impl fmt::Display for Constriction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Constriction::*;
        write!(f, "{}", match self {
            Close => "Close",
            CloseNear => "Near-Close",
            CloseMid => "Close-Mid",
            Mid => "Mid",
            OpenMid => "Open-Mid",
            OpenNear => "Near-Open",
            Open => "Open"
        })
    }
}

impl Outer<Place, Rounding> for Constriction { /*  */ }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

impl Inner<Rounding> for Place { /*  */ }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

impl Pair for Rounding { /*  */ }