use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct BiGraph<V> {
    edges: HashMap<V, HashSet<V>>,
}

impl<V: Hash + Eq + Copy> BiGraph<V> {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, a: V, b: V) {
        self.edges.entry(a).or_default().insert(b);
        self.edges.entry(b).or_default().insert(a);
    }

    pub fn neighbors(&self, v: &V) -> impl Iterator<Item = &V> {
        self.edges.get(v).into_iter().flat_map(|set| set.iter())
    }

    pub fn contains_edge(&self, a: V, b: V) -> bool {
        self.edges.get(&a).map_or(false, |set| set.contains(&b))
    }

    pub fn vertices(&self) -> impl Iterator<Item = &V> {
        self.edges.keys()
    }
}
