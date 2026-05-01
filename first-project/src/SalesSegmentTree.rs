struct SalesSegmentTree {
    tree: Vec<i32>,
    sales: Vec<i32>,
    n: usize,
}

impl SalesSegmentTree {
    fn new(sales: Vec<i32>) -> Self {
        let n = sales.len();
        let mut segment_tree = SalesSegmentTree {
            tree: vec![0; 4 * n],
            sales,
            n,
        };

        segment_tree.build(1, 0, n - 1);
        segment_tree
    }

    fn build(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = self.sales[start];
            return;
        }

        let mid = (start + end) / 2;

        self.build(node * 2, start, mid);
        self.build(node * 2 + 1, mid + 1, end);

        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    fn query(&self, left: usize, right: usize) -> i32 {
        self.query_internal(1, 0, self.n - 1, left, right)
    }

    fn query_internal(
        &self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
    ) -> i32 {
        // out
        if right < start || end < left {
            return 0;
        }

        // inside
        if left <= start && end <= right {
            return self.tree[node];
        }

        let mid = (start + end) / 2;

        let sum_left = self.query_internal(node * 2, start, mid, left, right);
        let sum_right = self.query_internal(node * 2 + 1, mid + 1, end, left, right);

        sum_left + sum_right
    }

    fn update(&mut self, index: usize, new_value: i32) {
        self.update_internal(1, 0, self.n - 1, index, new_value);
    }

    fn update_internal(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        index: usize,
        new_value: i32,
    ) {
        if start == end {
            self.sales[index] = new_value;
            self.tree[node] = new_value;
            return;
        }

        let mid = (start + end) / 2;

        if index <= mid {
            self.update_internal(node * 2, start, mid, index, new_value);
        } else {
            self.update_internal(node * 2 + 1, mid + 1, end, index, new_value);
        }

        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }
}

fn main() {
    let sales = vec![10, 15, 7, 20, 13, 9, 18];

    let mut segment_tree = SalesSegmentTree::new(sales);

    println!("=== Sales System ===");

    println!(
        "Sales Amount day 2 to 5: {}",
        segment_tree.query(1, 4)
    );

    println!(
        "Amount total: {}",
        segment_tree.query(0, 6)
    );

    // segment_tree.update(3, 25);
    // println!("Updated total: {}", segment_tree.query(0, 6));
}
