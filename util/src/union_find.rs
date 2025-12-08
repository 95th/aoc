/// Disjoint set UNION
/// Returns true if items were not already connected.
pub fn union(connections: &mut [usize], a: usize, b: usize) -> bool {
    let root_a = find(connections, a);
    let root_b = find(connections, b);
    if root_a == root_b {
        return false;
    }
    connections[root_a] = root_b;
    for i in 0..connections.len() {
        if connections[i] == root_a {
            connections[i] = root_b;
        }
    }
    true
}

/// Disjoint set FIND
pub fn find(connections: &[usize], a: usize) -> usize {
    if connections[a] == a {
        a
    } else {
        find(connections, connections[a])
    }
}
