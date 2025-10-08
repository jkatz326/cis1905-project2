#[allow(unused_imports)]
use std::{cmp::Ord, mem};

#[derive(Clone, Debug)]
pub enum TreeNode<T: Ord> {
    Leaf,
    Node(T, Box<TreeNode<T>>, Box<TreeNode<T>>),
}

// Provided functions
impl<T: Ord> TreeNode<T> {
    pub fn height(&self) -> usize {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => 1 + std::cmp::max(left.height(), right.height()),
        }
    }

    /// Verifies that the tree is a binary search tree
    fn is_bst(&self) -> bool {
        fn is_bst_helper<T: Ord>(tree: &TreeNode<T>, min: Option<&T>, max: Option<&T>) -> bool {
            match tree {
                TreeNode::Leaf => true,
                TreeNode::Node(value, left, right) => {
                    match min {
                        Some(min) => {
                            if value <= min {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    match max {
                        Some(max) => {
                            if value >= max {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    is_bst_helper(left, min, Some(value)) && is_bst_helper(right, Some(value), max)
                }
            }
        }
        is_bst_helper(self, None, None)
    }

    /// Verifies that the tree is balanced
    pub fn is_balanced(&self) -> bool {
        match self {
            TreeNode::Leaf => true,
            TreeNode::Node(_, left, right) => {
                let left_height = left.height();
                let right_height = right.height();
                let diff = (left_height as i32 - right_height as i32).abs();
                diff <= 1 && left.is_balanced() && right.is_balanced()
            }
        }
    }

    /// Verifies that the tree is a valid balanced binary search tree
    pub fn validate(&self) -> bool {
        self.is_bst() && self.is_balanced()
    }
}

// Required functions
impl<T: Ord> TreeNode<T> {
    /// Creates a new `TreeNode<T>` with value `value` and children `left` and `right`
    pub fn node(value: T, left: TreeNode<T>, right: TreeNode<T>) -> TreeNode<T> {
        TreeNode::Node(value, Box::new(left), Box::new(right))
    }

    /// Creates a new `TreeNode<T>` with no children
    pub fn new() -> TreeNode<T> {
        TreeNode::Leaf
    }

    /// Inserts a new node with value `value` into the tree. If the value already exists in the tree,
    /// the function does nothing.
    ///
    /// After insertion, the tree is rebalanced if necessary
    pub fn insert(&mut self, value: T) {
        match self {
            TreeNode::Leaf => *self = TreeNode::node(value, TreeNode::new(), TreeNode::new()),
            TreeNode::Node(node_val, left, right) => {
                match value.cmp(node_val) {
                    std::cmp::Ordering::Equal => return,
                    std::cmp::Ordering::Less => left.insert(value),
                    std::cmp::Ordering::Greater => right.insert(value),
                }
            }
        }
        self.rebalance();
    }

    /// Computes the balance factor of the tree (the difference between the height of the left and right subtrees)
    fn balance_factor(&self) -> i32 {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => (left.height() as i32) - (right.height() as i32)
        }
    }

    /// Performs a left rotation on the tree
    pub fn left_rotate(&mut self) {
        let root = mem::replace(self, TreeNode::Leaf);
        match root {
            TreeNode::Leaf => return,
            TreeNode::Node(root_value, left, right) => {
                match *right {
                    TreeNode::Leaf => *self = TreeNode::node(root_value, *left, TreeNode::Leaf),
                    TreeNode::Node(pivot_value, pivot_left, pivot_right) => {
                        *self = TreeNode::node(pivot_value, TreeNode::node(root_value, *left, *pivot_left), *pivot_right)
                    }
                }
            }
        }
    }
    /// Performs a right rotation on the tree
    pub fn right_rotate(&mut self) {
        let root = mem::replace(self, TreeNode::Leaf);
        match root {
            TreeNode::Leaf => return,
            TreeNode::Node(root_value, left, right) => {
                match *left {
                    TreeNode::Leaf => *self = TreeNode::node(root_value, TreeNode::Leaf, *right),
                    TreeNode::Node(pivot_value, pivot_left, pivot_right) => {
                        *self = TreeNode::node(pivot_value, *pivot_left, TreeNode::node(root_value, *pivot_right, *right))
                    }
                }
            }
        }
    }

    /// Rebalances the tree using either a single or double rotation, as specified in the AVL tree
    /// rebalancing algorithm.
    fn rebalance(&mut self) {
        let balance_factor : i32 = self.balance_factor();
        match self {
                TreeNode::Node(value, left, right) => {
                    if balance_factor == -2 {
                        if right.balance_factor() == 1 {
                            right.right_rotate();
                        }
                        self.left_rotate();
                    } else if balance_factor == 2 {
                        if left.balance_factor() == -1 {
                            left.left_rotate();
                        }
                        self.right_rotate();
                    }
                }
                TreeNode::Leaf => {}
            }
    }
}

// Implement `Default` for `TreeNode<T>`
impl<T: Ord> Default for TreeNode<T> {
    fn default() -> Self {
        TreeNode::Leaf
    }
}

// Implement `PartialEq` for `TreeNode<T>`
impl<T: Ord> PartialEq for TreeNode<T> where T: PartialEq{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TreeNode::Leaf, TreeNode::Leaf) => true,
            (TreeNode::Node(val1, left1,right1), TreeNode::Node(val2, left2, right2)) => {
                val1 == val2 && left1.eq(left2) && right1.eq(right2)
            },
            _ => false
        }
    }
}

// Implement `Eq` for `TreeNode<T>`
impl<T: Ord> Eq for TreeNode<T> where T: Eq{}

// Implement `From<Vec<T>>` for `TreeNode<T>`
impl<T: Ord> From<Vec<T>> for TreeNode<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut tree : TreeNode<T> = TreeNode::new();
        for x in vec.into_iter() {
            tree.insert(x);
        }
        tree
    }
}

// Implement `From<TreeNode<T>>` for `Vec<T>`
impl<T: Ord> From<TreeNode<T>> for Vec<T> {
    fn from(value: TreeNode<T>) -> Self {
        todo!()
    }
}
