use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// An Undirected graph.
pub struct BiGraph<T> {
    edges: HashMap<T, HashSet<T>>,
}

impl<T: Hash + Eq + Copy> BiGraph<T> {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, a: T, b: T) {
        self.edges.entry(a).or_default().insert(b);
        self.edges.entry(b).or_default().insert(a);
    }

    pub fn neighbors(&self, v: T) -> impl Iterator<Item = T> + '_ {
        self.edges
            .get(&v)
            .into_iter()
            .flat_map(|set| set.iter().copied())
    }

    pub fn contains_edge(&self, a: T, b: T) -> bool {
        self.edges.get(&a).map_or(false, |set| set.contains(&b))
    }

    pub fn vertices(&self) -> impl Iterator<Item = T> + '_ {
        self.edges.keys().copied()
    }
}

impl<T> FromIterator<(T, T)> for BiGraph<T>
where
    T: Hash + Eq + Copy,
{
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
        let mut graph = BiGraph::new();
        for (a, b) in iter {
            graph.add_edge(a, b);
        }
        graph
    }
}

/// An Undirected graph.
pub struct DiGraph<T> {
    edges: HashMap<T, HashSet<T>>,
}

impl<T: Hash + Eq + Copy> DiGraph<T> {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, a: T, b: T) {
        self.edges.entry(a).or_default().insert(b);
    }

    pub fn neighbors(&self, v: T) -> impl Iterator<Item = T> + '_ {
        self.edges
            .get(&v)
            .into_iter()
            .flat_map(|set| set.iter().copied())
    }

    pub fn contains_edge(&self, a: T, b: T) -> bool {
        self.edges.get(&a).map_or(false, |set| set.contains(&b))
    }

    pub fn vertices(&self) -> impl Iterator<Item = T> + '_ {
        self.edges.keys().copied()
    }
}

impl<T> FromIterator<(T, T)> for DiGraph<T>
where
    T: Hash + Eq + Copy,
{
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
        let mut graph = DiGraph::new();
        for (a, b) in iter {
            graph.add_edge(a, b);
        }
        graph
    }
}
