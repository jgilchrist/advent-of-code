pub mod ascii_ocr;
pub mod geometry;
mod hash;
mod heap;
pub mod inputs;
pub mod iters;
pub mod search;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod prelude {
    pub use crate::inputs;

    pub use crate::hash::{Map, MapBuilder, Set, SetBuilder};
    pub use crate::heap::MinHeap;
    pub use std::collections::hash_map::Entry::{Occupied, Vacant};

    pub use itertools::*;
    pub use regex::Regex;
}
