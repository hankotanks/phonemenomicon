use crate::types::{Phoneme, PhonemeQuality};
use crate::types::category::{Outer, Inner, Pair};

pub type Modifier<A, B, C> = (PhonemeQuality<A, B, C>, String, String);

pub struct Diacritics<A: Outer<B, C>, B: Inner<C>, C: Pair> {
    pub contents: Vec<Modifier<A, B, C>>,
    pub change_state: fn(&mut Phoneme),
    pub prepend_blank: bool,
    pub submenu_visible: bool
}