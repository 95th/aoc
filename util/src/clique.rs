use std::{collections::HashSet, hash::Hash};

use crate::BiGraph;

/// Find max clique using Bron-Kerbosch algorithm
pub fn max_clique<T>(graph: &BiGraph<T>) -> Vec<T>
where
    T: Eq + Hash + Copy,
{
    let mut max_clique = Vec::new();
    find_max_clique(
        graph,
        Vec::new(),
        graph.vertices().collect(),
        HashSet::new(),
        &mut max_clique,
    );
    max_clique
}

fn find_max_clique<T>(
    graph: &BiGraph<T>,
    r: Vec<T>,
    mut p: Vec<T>,
    mut x: HashSet<T>,
    max_clique: &mut Vec<T>,
) where
    T: Eq + Hash + Copy,
{
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r;
        }
        return;
    }

    let pivot = *p.iter().chain(x.iter()).next().unwrap();
    let mut p_dash = p.clone();
    p_dash.retain(|&n| !graph.contains_edge(pivot, n));
    while let Some(v) = p_dash.pop() {
        {
            let mut r = r.clone();
            let mut p = p.clone();
            let mut x = x.clone();
            r.push(v);
            p.retain(|&n| graph.contains_edge(v, n));
            x.retain(|&n| graph.contains_edge(v, n));
            find_max_clique(graph, r, p, x, max_clique);
        }
        x.insert(v);
        p.retain(|&n| n != v);
    }
}
