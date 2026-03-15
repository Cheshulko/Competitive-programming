const N: usize = 100_000;

#[derive(Clone, Copy, Debug)]
struct Node {
    mult: usize,
    add: usize,
}

impl Default for Node {
    fn default() -> Self {
        Self { mult: 1, add: 0 }
    }
}

impl Node {
    const MOD: usize = 1_000_000_000 + 7;

    fn new(value: usize) -> Self {
        Self::add(value)
    }

    fn mult(value: usize) -> Self {
        Self {
            mult: value,
            add: 0,
        }
    }

    fn add(value: usize) -> Self {
        Self {
            mult: 1,
            add: value,
        }
    }

    // self <*> node
    fn apply(&mut self, node: Node) {
        self.mult *= node.mult;
        self.mult %= Node::MOD;

        self.add *= node.mult;
        self.add += node.add;
        self.add %= Node::MOD;
    }
}

struct Fancy {
    tree: [Node; 2 * N],
    len: usize,
}

use std::ops::Range;

impl Fancy {
    fn new() -> Self {
        Self {
            tree: [Node::default(); 2 * N],
            len: 0,
        }
    }

    fn update_recursive(
        &mut self,
        idx: usize,
        element_range: Range<usize>,
        update_range: Range<usize>,
        node: Node,
    ) {
        if element_range.start >= update_range.end || element_range.end <= update_range.start {
            return;
        }
        if element_range.end - element_range.start == 1 {
            self.apply(idx, node);
            return;
        }
        if element_range.start >= update_range.start && element_range.end <= update_range.end {
            self.apply(idx, node);
            return;
        }

        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        self.propagate(idx);
        self.update_recursive(
            idx * 2 + 1,
            element_range.start..mid,
            update_range.clone(),
            node,
        );
        self.update_recursive(idx * 2 + 2, mid..element_range.end, update_range, node);
        self.tree[idx] = Node::default();
    }

    fn apply(&mut self, idx: usize, node: Node) {
        self.tree[idx].apply(node);
    }

    fn propagate(&mut self, idx: usize) {
        self.apply(2 * idx + 1, self.tree[idx]);
        self.apply(2 * idx + 2, self.tree[idx]);
        self.tree[idx] = Node::default();
    }

    fn query_recursive(
        &mut self,
        idx: usize,
        element_range: Range<usize>,
        query_range: Range<usize>,
    ) -> Option<Node> {
        if element_range.start >= query_range.end || element_range.end <= query_range.start {
            return None;
        }
        if element_range.start >= query_range.start && element_range.end <= query_range.end {
            return Some(self.tree[idx]);
        }

        let mid = element_range.start + (element_range.end - element_range.start) / 2;
        self.propagate(idx);

        let left = self.query_recursive(idx * 2 + 1, element_range.start..mid, query_range.clone());
        let right = self.query_recursive(idx * 2 + 2, mid..element_range.end, query_range);
        match (left, right) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            _ => unreachable!(),
        }
    }

    fn add_all(&mut self, inc: i32) {
        let inc = inc as usize;

        self.update_recursive(0, 0..N, 0..self.len, Node::add(inc));
    }

    fn mult_all(&mut self, m: i32) {
        let m = m as usize;

        self.update_recursive(0, 0..N, 0..self.len, Node::mult(m));
    }

    fn append(&mut self, val: i32) {
        let val = val as usize;

        self.update_recursive(0, 0..N, self.len..self.len + 1, Node::add(val));
        self.len += 1;
    }

    fn get_index(&mut self, idx: i32) -> i32 {
        let idx = idx as usize;

        if idx < self.len {
            self.query_recursive(0, 0..N, idx..(idx + 1))
                .map(|node| node.add as i32)
                .unwrap()
        } else {
            -1
        }
    }
}
