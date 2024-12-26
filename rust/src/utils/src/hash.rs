/// Hashing code taken from <https://github.com/maneatingape/advent-of-code-rust>
/// to avoid an external dependency.
///
/// Extends the built-in `HashSet` and `HashMap` with an alternative (but simplified)
/// hasher implementation based on `FxHasher`.
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::BitXor as _;

pub type Set<V> = HashSet<V, BuildFxHasher>;

pub trait SetBuilder<T> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn from_array<const N: usize>(array: [T; N]) -> Self;
}

#[allow(clippy::implicit_hasher)]
impl<T: Eq + Hash> SetBuilder<T> for Set<T> {
    fn new() -> Self {
        Self::with_hasher(BuildFxHasher)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildFxHasher)
    }

    fn from_array<const N: usize>(array: [T; N]) -> Self {
        let mut set = Self::new();
        set.extend(array);
        set
    }
}

pub type Map<K, V> = HashMap<K, V, BuildFxHasher>;

pub trait MapBuilder<K, V> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn from_array<const N: usize>(array: [(K, V); N]) -> Self;
}

#[allow(clippy::implicit_hasher)]
impl<K: Eq + Hash, V> MapBuilder<K, V> for Map<K, V> {
    fn new() -> Self {
        Self::with_hasher(BuildFxHasher)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildFxHasher)
    }

    fn from_array<const N: usize>(array: [(K, V); N]) -> Self {
        let mut map = Self::new();
        map.extend(array);
        map
    }
}

/// If you want an instance of [`FxHasher`] then this has you covered.
#[derive(Clone, Copy, Default)]
pub struct BuildFxHasher;

impl BuildHasher for BuildFxHasher {
    type Hasher = FxHasher;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        FxHasher { hash: 0 }
    }
}

const K: u64 = 0x517cc1b727220a95;

pub struct FxHasher {
    hash: u64,
}

impl FxHasher {
    #[inline]
    fn add(&mut self, i: u64) {
        self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(K);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 8 {
            self.add(u64::from_ne_bytes(bytes[..8].try_into().unwrap()));
            bytes = &bytes[8..];
        }
        if bytes.len() >= 4 {
            self.add(u64::from(u32::from_ne_bytes(
                bytes[..4].try_into().unwrap(),
            )));
            bytes = &bytes[4..];
        }
        if bytes.len() >= 2 {
            self.add(u64::from(u16::from_ne_bytes(
                bytes[..2].try_into().unwrap(),
            )));
            bytes = &bytes[2..];
        }
        if !bytes.is_empty() {
            self.add(u64::from(bytes[0]));
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add(u64::from(i));
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add(u64::from(i));
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add(u64::from(i));
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }
}
