use bevy::prelude::*;
mod resources;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::Tree {
            root: resources::TreeNode {
                name: "Root".to_string(),
                children: vec![
                    resources::TreeNode {
                        name: "Child 1".to_string(),
                        children: vec![],
                    },
                    resources::TreeNode {
                        name: "Child 2".to_string(),
                        children: vec![],
                    },
                ],
            },
        });
    }
}
