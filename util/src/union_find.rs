pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a == root_b {
            return false;
        }
        self.parent[root_a] = root_b;
        for i in 0..self.parent.len() {
            if self.parent[i] == root_a {
                self.parent[i] = root_b;
            }
        }
        true
    }

    pub fn find(&mut self, a: usize) -> usize {
        if self.parent[a] == a {
            a
        } else {
            self.find(self.parent[a])
        }
    }
}
