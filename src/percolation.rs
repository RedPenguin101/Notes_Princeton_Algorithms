use super::union_find;

pub struct Grid {
    n: usize,
    qu: union_find::WeightedQuickUnion,
    unblocked: Vec<bool>
}

impl Grid {
    pub fn new(n: usize) -> Grid {
        let mut unbl = vec![false; n.pow(2)+2];
        unbl[0] = true;
        unbl[n.pow(2)+1] = true;

        Grid {
            n,
            qu: union_find::WeightedQuickUnion::new(n.pow(2)+2),
            unblocked: unbl
        }
    }

    fn unblock_box(&mut self, row: usize, column: usize) {
        let node = self.box_to_array_position(row, column);
        let side_length = self.n;

        self.unblocked[node] = true;

        if row == 0 {
            self.connect(node, 0)
        } if row == side_length-1 {
            self.connect(node, side_length.pow(2)+1);
        }

        if row != 0 {
            self.connect(node, node - side_length);
        } if row != self.n-1 {
            self.connect(node, node + side_length);
        } if column != 0 {
            self.connect(node, node - 1);
        } if column != self.n-1 {
            self.connect(node, node + 1);
        }
    }

    fn connect(&mut self, node: usize, partner: usize) {
        if self.unblocked[node] && self.unblocked[partner] {
            self.qu.connect(node, partner);
        }
    }

    fn box_to_array_position(&self, row: usize, column: usize) -> usize {
        column + (self.n * row) + 1
    }


    pub fn percolates(&mut self) -> bool {
        self.qu.is_connected(0, self.n.pow(2)+1)
    }
}

#[cfg(test)]
mod grid {
    use super::*;

    #[test]
    fn with_1x1_unblocked_percolates() {
        let mut grid = Grid::new(1);
        grid.unblock_box(0,0);
        assert_eq!(grid.percolates(), true);
    }

    #[test]
    fn with_1x1_blocked_doesnt_percolate() {
        let mut grid = Grid::new(1);
        assert_eq!(grid.percolates(), false);
    }

    #[test]
    fn with_2x2_blocked_doesnt_percolate() {
        let mut grid = Grid::new(2);
        grid.unblock_box(0,0);
        grid.unblock_box(0,1);
        assert_eq!(grid.percolates(), false);
    }

    #[test]
    fn with_2x2_unblocked_percolate() {
        let mut grid = Grid::new(2);
        grid.unblock_box(0,0);
        grid.unblock_box(1,0);
        assert_eq!(grid.percolates(), true);
    }
}
