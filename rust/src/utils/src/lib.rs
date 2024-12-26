pub mod ascii_ocr;
pub mod geometry;
pub mod inputs;
pub mod iters;
pub mod search;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod prelude {
    pub use crate::inputs;

    pub use hashbrown::HashMap;
    pub use hashbrown::HashSet;
    pub use std::collections::hash_map::Entry::{Occupied, Vacant};

    pub use fancy_regex::Regex;
    pub use itertools::*;
}
