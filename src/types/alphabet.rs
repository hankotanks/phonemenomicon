use enum_map::EnumMap;
use slotmap::DefaultKey;

use crate::types::category::{
    Category,
    Articulation,
    Region,
    Voicing,
    Constriction,
    Place,
    Rounding
};

pub struct Alphabet<A: Category, B: Category, C: Category> {
    query: EnumMap<A, EnumMap<B, EnumMap<C, Option<DefaultKey>>>>,
    // TODO
}