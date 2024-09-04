use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(name: &str) -> TreeNode {
        TreeNode {
            name: name.to_string(),
            children: vec![],
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

#[derive(Resource)]
pub struct Tree {
    pub root: TreeNode,
}

impl Tree {
    pub fn new(root_name: &str) -> Self {
        Tree {
            root: TreeNode::new(root_name),
        }
    }

    pub fn print(&self) {
        self.root.print(0);
    }
}