// UnionFind

// QuickFind: uses n-length array as network model. elements p and q are connected if they share
// the same array value
// It is SLOW: it takes a quadratic (N^2) array accesses to produce produce N unions of N nodes.

struct QuickFind(Vec<u32>);

impl QuickFind {
    fn new(n: u32) -> QuickFind {
        QuickFind((0..n).collect())
    }

    fn is_connected(&self, p: usize, q: usize) -> bool {
        println!("{:?}", self.0);
        self.0[p] == self.0[q]
    }

    fn connect(&mut self, p: usize, q: usize) {
        let mut new_vec: Vec<u32> = vec!();

        let target_group = self.0[p];
        let replacement_group = self.0[q];

        for current_group_id in &self.0 {
            if current_group_id == &target_group {
                new_vec.push(replacement_group);
            } else {
                new_vec.push(*current_group_id)
            }
        }
        self.0 = new_vec;
    }

    fn number_of_groups(&self) -> usize {
        let mut x = self.0.clone();
        x.sort();
        x.dedup();
        x.len()
    }
}

#[cfg(test)]
mod TestQuickFind {
    use super::*;

    #[test]
    fn is_unconnected_at_initialisaton() {
        let qf = QuickFind::new(2);
        assert_eq!(qf.is_connected(0, 1), false)
    }

    #[test]
    fn is_reflexive() {
        let qf = QuickFind::new(2);
        assert_eq!(qf.is_connected(0, 0), true);
        assert_eq!(qf.is_connected(1, 1), true);
    }

    #[test]
    fn can_connect_single_node() {
        let mut qf = QuickFind::new(2);
        qf.connect(0, 1);
        assert_eq!(qf.is_connected(0,1), true);
    }

    #[test]
    fn doesnt_connect_everything() {
        let mut qf = QuickFind::new(3);
        qf.connect(0, 1);
        assert_eq!(qf.is_connected(0,1), true);
        assert_eq!(qf.is_connected(0,2), false);
        assert_eq!(qf.is_connected(1,2), false);
    }

    #[test]
    fn connects_transitively() {
        let mut qf = QuickFind::new(2);
        qf.connect(0,1);
        assert_eq!(qf.is_connected(0,1), true);
        assert_eq!(qf.is_connected(1,0), true);
    }

    #[test]
    fn connects_across_unions() {
        let mut qf = QuickFind::new(3);
        qf.connect(0,1);
        qf.connect(1,2);
        assert_eq!(qf.is_connected(0,2), true);
    }

    #[test]
    fn can_count_unconnected_groups() {
        let mut qf = QuickFind::new(3);
        assert_eq!(qf.number_of_groups(), 3);
    }

    #[test]
    fn can_count_connected_groups() {
        let mut qf = QuickFind::new(3);
        assert_eq!(qf.number_of_groups(), 3);
        qf.connect(0,1);
        assert_eq!(qf.number_of_groups(), 2);
        qf.connect(0,2);
        assert_eq!(qf.number_of_groups(), 1);
    }

    #[test]
    fn long_test() {
        let mut qf = QuickFind::new(10);
        qf.connect(4,3);
        qf.connect(3,8);
        qf.connect(6,5);
        qf.connect(9,4);
        qf.connect(2,1);
        assert_eq!(qf.is_connected(0,7), false);
        assert_eq!(qf.is_connected(8,9), true);
        qf.connect(5,0);
        qf.connect(7,2);
        qf.connect(6,1);
        qf.connect(1,0);
        assert_eq!(qf.is_connected(0,7), true);
    }
}

// QuickUnion
// 'lazy' approach
// same struct, different interpretation, a forest interpretation. Each node, represented by its array position, contains a
// reference to its parent node
// Faster than quickfind but still too slow. The trees get too tall and then find gets too
// expensive

struct QuickUnion(Vec<usize>);

impl QuickUnion {
    fn new(n: usize) -> QuickUnion {
        QuickUnion((0..n).collect())
    }

    fn is_connected(&self, p: usize, q: usize) -> bool {
        self.find_top_of_tree(p) == self.find_top_of_tree(q)
    }

    fn connect(&mut self, p: usize, q: usize) {
        let top_parent_of_p = self.find_top_of_tree(p);
        self.0[top_parent_of_p] = self.find_top_of_tree(q);
    }

    fn find_top_of_tree(&self, p: usize) -> usize {
        let mut current_node = p;
        while self.0[current_node] != current_node {
            current_node = self.0[current_node];
        }
        current_node
    }

    fn number_of_groups(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod TestQuickUnion {
    use super::*;

    #[test]
    fn is_unconnected_at_initialisaton() {
        let qf = QuickUnion::new(2);
        assert_eq!(qf.is_connected(0, 1), false)
    }

    #[test]
    fn is_reflexive() {
        let qf = QuickUnion::new(2);
        assert_eq!(qf.is_connected(0, 0), true);
        assert_eq!(qf.is_connected(1, 1), true);
    }

    #[test]
    fn can_connect_single_node() {
        let mut qf = QuickUnion::new(2);
        qf.connect(0, 1);
        assert_eq!(qf.is_connected(0,1), true);
    }

    #[test]
    fn doesnt_connect_everything() {
        let mut qf = QuickUnion::new(3);
        qf.connect(0, 1);
        assert_eq!(qf.is_connected(0,1), true);
        assert_eq!(qf.is_connected(0,2), false);
        assert_eq!(qf.is_connected(1,2), false);
    }

    #[test]
    fn connects_transitively() {
        let mut qf = QuickUnion::new(2);
        qf.connect(0,1);
        assert_eq!(qf.is_connected(0,1), true);
        assert_eq!(qf.is_connected(1,0), true);
    }

    #[test]
    fn connects_across_unions() {
        let mut qf = QuickUnion::new(3);
        qf.connect(0,1);
        qf.connect(1,2);
        assert_eq!(qf.is_connected(0,2), true);
    }

    //#[test]
    fn can_count_unconnected_groups() {
        let mut qf = QuickUnion::new(3);
        assert_eq!(qf.number_of_groups(), 3);
    }

    //#[test]
    fn can_count_connected_groups() {
        let mut qf = QuickUnion::new(3);
        assert_eq!(qf.number_of_groups(), 3);
        qf.connect(0,1);
        assert_eq!(qf.number_of_groups(), 2);
        qf.connect(0,2);
        assert_eq!(qf.number_of_groups(), 1);
    }

    #[test]
    fn long_test() {
        let mut qf = QuickUnion::new(10);
        qf.connect(4,3);
        qf.connect(3,8);
        qf.connect(6,5);
        qf.connect(9,4);
        qf.connect(2,1);
        assert_eq!(qf.is_connected(0,7), false);
        assert_eq!(qf.is_connected(8,9), true);
        qf.connect(5,0);
        qf.connect(7,2);
        qf.connect(6,1);
        qf.connect(1,0);
        assert_eq!(qf.is_connected(0,7), true);
    }
}

// QuickUnionImproved
// improvement 1: we saw having tall trees made this ineffecient for find. So always attach the
// root of smaller tree to root of larger tree, not the other way around.

struct QuickUnionImproved(Vec<usize>);

impl QuickUnionImproved {
    fn new(n: usize) -> QuickUnionImproved {
        QuickUnionImproved((0..n).collect())
    }

    fn is_connected(&self, p: usize, q: usize) -> bool {
        self.find_top_of_tree(p) == self.find_top_of_tree(q)
    }

    fn connect(&mut self, p: usize, q: usize) {
        let top_parent_of_p = self.find_top_of_tree(p);
        let top_parent_of_q = self.find_top_of_tree(q);
        self.0[top_parent_of_p] = self.find_top_of_tree(q);
    }

    fn find_number_of_children(&self, p: usize) -> {
        let mut counter = 0;
        for children 
    }

    fn find_top_of_tree(&self, p: usize) -> usize {
        let mut current_node = p;
        while self.0[current_node] != current_node {
            current_node = self.0[current_node];
        }
        current_node
    }

    fn number_of_groups(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod TestQuickUnionImproved {
    use super::*;

    #[test]
    fn is_unconnected_at_initialisaton() {
        let qf = QuickUnionImproved::new(2);
        assert_eq!(qf.is_connected(0, 1), false)
    }

    #[test]
    fn is_reflexive() {
        let qf = QuickUnionImproved::new(2);
        assert_eq!(qf.is_connected(0, 0), true);
        assert_eq!(qf.is_connected(1, 1), true);
    }

    #[test]
    fn can_connect_single_node() {
        let mut qf = QuickUnionImproved::new(2);
        qf.connect(0, 1);
        assert_eq!(qf.is_connected(0,1), true);
    }

    #[test]
    fn doesnt_connect_everything() {
        let mut qf = QuickUnionImproved::new(3);
        qf.connect(0, 1);
        assert_eq!(qf.is_connected(0,1), true);
        assert_eq!(qf.is_connected(0,2), false);
        assert_eq!(qf.is_connected(1,2), false);
    }

    #[test]
    fn connects_transitively() {
        let mut qf = QuickUnionImproved::new(2);
        qf.connect(0,1);
        assert_eq!(qf.is_connected(0,1), true);
        assert_eq!(qf.is_connected(1,0), true);
    }

    #[test]
    fn connects_across_unions() {
        let mut qf = QuickUnionImproved::new(3);
        qf.connect(0,1);
        qf.connect(1,2);
        assert_eq!(qf.is_connected(0,2), true);
    }

    //#[test]
    fn can_count_unconnected_groups() {
        let mut qf = QuickUnionImproved::new(3);
        assert_eq!(qf.number_of_groups(), 3);
    }

    //#[test]
    fn can_count_connected_groups() {
        let mut qf = QuickUnionImproved::new(3);
        assert_eq!(qf.number_of_groups(), 3);
        qf.connect(0,1);
        assert_eq!(qf.number_of_groups(), 2);
        qf.connect(0,2);
        assert_eq!(qf.number_of_groups(), 1);
    }

    #[test]
    fn long_test() {
        let mut qf = QuickUnionImproved::new(10);
        qf.connect(4,3);
        qf.connect(3,8);
        qf.connect(6,5);
        qf.connect(9,4);
        qf.connect(2,1);
        assert_eq!(qf.is_connected(0,7), false);
        assert_eq!(qf.is_connected(8,9), true);
        qf.connect(5,0);
        qf.connect(7,2);
        qf.connect(6,1);
        qf.connect(1,0);
        assert_eq!(qf.is_connected(0,7), true);
    }
}

