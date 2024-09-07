use bevy::prelude::*;
use std::{collections::HashMap, string};

use crate::scene::tesselate::resources::PrimitiveType;

#[derive(Resource)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
    pub extended: bool,
    pub primitive_type: PrimitiveType,
    pub transform: bevy::prelude::Transform
}

impl Default for PrimitiveType {
    fn default() -> Self {
        PrimitiveType::None
    }
}

impl TreeNode {
    pub fn new(name: &str, primitive_type: PrimitiveType, transform: bevy::prelude::Transform) -> TreeNode {
        TreeNode {
            name: name.to_string(),
            children: vec![],
            extended: true,
            primitive_type: primitive_type,
            transform: transform
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
        TreeNode {
            name: String::new(),
            children: vec![],
            extended: true,
            ..default()
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
            root: TreeNode::new(root_name, PrimitiveType::None, bevy::prelude::Transform::default()),
        }
    }

    pub fn print(&self) {
        self.root.print(0);
    }
}