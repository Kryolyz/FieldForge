use bevy::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

use std::hash::{Hash, Hasher};

use crate::scene::tesselate::resources::PrimitiveType;

#[derive(Resource)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
    pub collapsed: bool,
    pub primitive_type: PrimitiveType,
    pub transform: bevy::prelude::Transform,
    pub id: u64,
    pub marked_for_deletion: bool, // New field
}

impl TreeNode {
    pub fn new(name: &str, primitive_type: PrimitiveType, transform: bevy::prelude::Transform) -> TreeNode {
        pub static NODE_ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        TreeNode {
            name: name.to_string(),
            children: vec![],
            collapsed: false,
            primitive_type: primitive_type,
            transform: transform,
            id: NODE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            marked_for_deletion: false
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    pub fn print(&self, depth: usize) {
        println!("{}{}", " ".repeat(depth * 2), self.name);
        for child in &self.children {
            child.print(depth + 1);
        }
    }
}

impl Default for TreeNode {
    fn default() -> Self {
        return TreeNode::new("", PrimitiveType::default(), Transform::default());
    }
}

impl Hash for TreeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.primitive_type.hash(state);
        state.write_u64(self.id);
    }
}

#[derive(Resource)]
pub struct Tree {
    pub root: TreeNode,
}

impl Tree {
    pub fn new(root_name: &str) -> Self {
        Tree {
            root: TreeNode::new(root_name, PrimitiveType::None, bevy::prelude::Transform::default()),
        }
    }

    pub fn print(&self) {
        self.root.print(0);
    }
}