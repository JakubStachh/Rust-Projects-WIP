#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn is_balanced(root: Option<Box<TreeNode>>) -> bool {
    fn height(node: Option<&Box<TreeNode>>) -> i32 {
        if node.is_none() {
            return 0;
        }
        let left_height = height(node.unwrap().left.as_ref());
        let right_height = height(node.unwrap().right.as_ref());

        if left_height == -1 || right_height == -1 || (left_height - right_height).abs() > 1 {
            return -1;
        }
        
        1 + left_height.max(right_height)
    }

    height(root.as_ref()) != -1
}

fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Box::new(TreeNode::new(2)));
    root.right = Some(Box::new(TreeNode::new(3)));
    root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
    
    println!("Is tree balanced? {}", is_balanced(Some(Box::new(root)))); // Output: true
}
