use std::{collections::HashSet, hash::Hash};

use crate::BiGraph;

impl<T> BiGraph<T>
where
    T: Eq + Hash + Copy,
{
    /// Find max clique using Bron-Kerbosch algorithm
    pub fn max_clique(&self) -> Vec<T> {
        let mut max_clique = Vec::new();
        self.find_max_clique(
            Vec::new(),
            self.vertices().collect(),
            HashSet::new(),
            &mut max_clique,
        );
        max_clique
    }

    fn find_max_clique(
        &self,
        r: Vec<T>,
        mut p: Vec<T>,
        mut x: HashSet<T>,
        max_clique: &mut Vec<T>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > max_clique.len() {
                *max_clique = r;
            }
            return;
        }

        let pivot = *p.iter().chain(x.iter()).next().unwrap();
        let mut p_dash = p.clone();
        p_dash.retain(|&n| !self.contains_edge(pivot, n));
        while let Some(v) = p_dash.pop() {
            {
                let mut r = r.clone();
                let mut p = p.clone();
                let mut x = x.clone();
                r.push(v);
                p.retain(|&n| self.contains_edge(v, n));
                x.retain(|&n| self.contains_edge(v, n));
                self.find_max_clique(r, p, x, max_clique);
            }
            x.insert(v);
            p.retain(|&n| n != v);
        }
    }
}
