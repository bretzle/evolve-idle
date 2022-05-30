use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Deref, DerefMut, IndexMut};
use std::{collections::HashMap, ops::Index};

pub struct MutMap<K, V>(HashMap<K, V>);

impl<K: Debug, V: Debug> std::fmt::Debug for MutMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_tuple("MutMap").field(&self.0).finish()
        write!(f, "{:?}", self.0)
    }
}

impl<K, V> MutMap<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<K, V> Index<K> for MutMap<K, V>
where
    K: Eq + Hash,
{
    type Output = V;

    #[inline]
    fn index(&self, key: K) -> &V {
        self.0.get(&key).expect("no entry found for key")
    }
}

impl<K, V> IndexMut<K> for MutMap<K, V>
where
    K: Eq + Hash,
{
    #[inline]
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.0.entry(key).or_insert(unsafe { std::mem::zeroed() })
    }
}

impl<K, V> Deref for MutMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for MutMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//////////////////////////////

#[derive(Debug, Clone, Copy)]
pub struct Bounded {
    pub cur: f64,
    pub max: f64,
}

impl Bounded {
    pub fn new<A: Into<f64>, B: Into<f64>>(cur: A, max: B) -> Self {
        Self {
            cur: cur.into(),
            max: max.into(),
        }
    }

    pub fn get(&self) -> f64 {
        self.cur
    }

    pub fn modify<F: Fn(&mut f64)>(&mut self, f: F) {
        f(&mut self.cur);
        self.cur = self.cur.min(self.max);
    }
}

impl PartialEq<f64> for Bounded {
    fn eq(&self, other: &f64) -> bool {
        self.cur == *other
    }
}

impl PartialOrd<f64> for Bounded {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.cur.partial_cmp(other)
    }
}

///////////////////////////////////

pub struct Rng(fastrand::Rng);

impl Rng {
    pub const fn new() -> Self {
        Self(gemstone::mem::zero())
    }
}

impl Deref for Rng {
    type Target = fastrand::Rng;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Sync for Rng {}
