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

// WeightedQuickUnion
//
// improvement 1: we saw having tall trees made this ineffecient for find. So always attach the
// root of smaller tree to root of larger tree, not the other way around.
// find is proportional to depths of p and q, union is constant time.
// Depth of any node x is at most lg N (log base 2)
// Proof: depth of x increases by 1 when tree T1 containing x is merged into another tree T2
// Size of tree containing x at least doubles
// size of tree containing x can double at most lg N times
//
// improvement 2: path compression: as you scan up the tree, why not set the parent of each
// examined node to the root. Or more simply, but sligtly less effectively 
// set every node in the path to its grandparent
// it was proved that any sequence of M union-find ops on N nodes will touch the array at most
// N+Mlg*N times. (lg*N is the number of times you have to take the lgN to get 1 - called the
// iterated log function - in practice think of it as 'a number less than 5')
// This means the algorithm is practically linear time in the real world. Not quite in theory -
// there is provably no linear time algo for union find

pub struct WeightedQuickUnion{
    pub parent_of: Vec<usize>,
    sizes: Vec<usize>,
}

impl WeightedQuickUnion {
    pub fn new(n: usize) -> WeightedQuickUnion {
        WeightedQuickUnion{
            parent_of: (0..n).collect(),
            sizes: vec![1; n],
        }
    }

    fn find_top_of_tree(&mut self, p: usize) -> usize {
        let mut current_node = p;

        while self.parent_of[current_node] != current_node {
            // improvement 2 is here
            self.parent_of[current_node] = self.parent_of[self.parent_of[current_node]];
            current_node = self.parent_of[current_node];
        }
        current_node
    }

    pub fn is_connected(&mut self, p: usize, q: usize) -> bool {
        self.find_top_of_tree(p) == self.find_top_of_tree(q)
    }

    pub fn connect(&mut self, p: usize, q: usize) {
        let top_parent_of_p = self.find_top_of_tree(p);
        let top_parent_of_q = self.find_top_of_tree(q);

        // improvement 1 here

        let p_bigger_than_q = self.sizes[top_parent_of_p] > self.sizes[top_parent_of_q];

        // initially I thought you should be checking for depth here, not overall tree size, but
        // that isn't correct. The goal for each union is to minimise the total increase in depth
        // for each node in each tree. The depth for each node will be increasing by 1 for every node in
        // the tree that is attached, and by 0 for each node in the tree that is attached to. So by
        // attaching the tree with the less number of nodes, you are minimising the overall depth
        // increase

        if p_bigger_than_q {
            self.parent_of[top_parent_of_q] = top_parent_of_p;
            self.sizes[top_parent_of_p] = self.sizes[top_parent_of_p] + self.sizes[top_parent_of_q]
        } else {
            self.parent_of[top_parent_of_p] = top_parent_of_q;
            self.sizes[top_parent_of_q] = self.sizes[top_parent_of_q] + self.sizes[top_parent_of_p]
        } 

        println!("connected node {} with node {}", p, q);
    }


    fn number_of_groups(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod TestWeightedQuickUnion {
    use super::*;

    #[test]
    fn is_unconnected_at_initialisaton() {
        let mut qf = WeightedQuickUnion::new(2);
        assert_eq!(qf.is_connected(0, 1), false)
    }

    #[test]
    fn is_reflexive() {
        let mut qf = WeightedQuickUnion::new(2);
        assert_eq!(qf.is_connected(0, 0), true);
        assert_eq!(qf.is_connected(1, 1), true);
    }

    #[test]
    fn can_connect_single_node() {
        let mut qf = WeightedQuickUnion::new(2);
        qf.connect(0, 1);
        assert_eq!(qf.is_connected(0,1), true);
    }

    #[test]
    fn doesnt_connect_everything() {
        let mut qf = WeightedQuickUnion::new(3);
        qf.connect(0, 1);
        assert_eq!(qf.is_connected(0,1), true);
        assert_eq!(qf.is_connected(0,2), false);
        assert_eq!(qf.is_connected(1,2), false);
    }

    #[test]
    fn connects_transitively() {
        let mut qf = WeightedQuickUnion::new(2);
        qf.connect(0,1);
        assert_eq!(qf.is_connected(0,1), true);
        assert_eq!(qf.is_connected(1,0), true);
    }

    #[test]
    fn connects_across_unions() {
        let mut qf = WeightedQuickUnion::new(3);
        qf.connect(0,1);
        qf.connect(1,2);
        assert_eq!(qf.is_connected(0,2), true);
    }

    #[test]
    fn long_test() {
        let mut qf = WeightedQuickUnion::new(10);
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

// summary of dynamic connectivity problem / union find problem:
// Alg          |   worst case time
// Quick Find   |   M N
// Quick Union  |   M N
// Weighted QU  |   N + MlgN
// QU + Path Com|   N + MlgN
// WQU+PC       |   N + Mlg*N
// 
// 10^9 unions on 10^9 objects - QF takes 30 years, WQUPC takes 6 seconds. Picking the right
// algorithm is important
