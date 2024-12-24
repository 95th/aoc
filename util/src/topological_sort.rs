use std::{collections::HashSet, hash::Hash};

use crate::DiGraph;

impl<T> DiGraph<T>
where
    T: Eq + Hash + Copy,
{
    pub fn topological_sort(&self) -> Vec<T> {
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();

        for node in self.vertices() {
            if visited.insert(node) {
                dfs(self, &mut visited, &mut sorted, node);
            }
        }

        sorted
    }
}

fn dfs<T: Eq + Hash + Copy>(
    graph: &DiGraph<T>,
    visited: &mut HashSet<T>,
    sorted: &mut Vec<T>,
    node: T,
) {
    visited.insert(node);
    for neighbor in graph.neighbors(node) {
        if visited.insert(neighbor) {
            dfs(graph, visited, sorted, neighbor);
        }
    }
    sorted.push(node);
}
