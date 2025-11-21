//! Block Layout Algorithm
//!
//! Implements CSS block formatting context.

use crate::layout::{Dimensions, LayoutId, LayoutTree};

/// Block layout context
pub struct BlockContext<'a> {
    tree: &'a mut LayoutTree,
    containing_block: Dimensions,
}

impl<'a> BlockContext<'a> {
    pub fn new(tree: &'a mut LayoutTree, containing_block: Dimensions) -> Self {
        Self {
            tree,
            containing_block,
        }
    }

    /// Layout a block element
    pub fn layout(&mut self, id: LayoutId) {
        self.calculate_width(id);
        self.calculate_position(id);
        self.layout_children(id);
        self.calculate_height(id);
    }

    /// Calculate block width based on containing block
    fn calculate_width(&mut self, id: LayoutId) {
        let containing_width = self.containing_block.content.width;

        if let Some(node) = self.tree.get_node_mut(id) {
            let dims = &mut node.box_model.dimensions;

            // Calculate auto margins
            let total_used = dims.margin.horizontal()
                + dims.border.horizontal()
                + dims.padding.horizontal()
                + dims.content.width;

            let available = containing_width - total_used;

            // Distribute remaining space to auto margins
            if available > 0.0 {
                // For now, just use the specified width
                if dims.content.width == 0.0 {
                    dims.content.width = containing_width
                        - dims.margin.horizontal()
                        - dims.border.horizontal()
                        - dims.padding.horizontal();
                }
            }
        }
    }

    /// Calculate block position
    fn calculate_position(&mut self, id: LayoutId) {
        let (container_x, container_y) = (
            self.containing_block.content.x,
            self.containing_block.content.y,
        );

        if let Some(node) = self.tree.get_node_mut(id) {
            let dims = &mut node.box_model.dimensions;
            dims.content.x = container_x + dims.margin.left + dims.border.left + dims.padding.left;
        }
    }

    /// Layout child elements
    fn layout_children(&mut self, _id: LayoutId) {
        // Implementation deferred to LayoutTree
    }

    /// Calculate block height based on content
    fn calculate_height(&mut self, _id: LayoutId) {
        // Height is already calculated in layout_block_children
    }
}

/// Margin collapsing utilities
pub mod margin {
    /// Collapse two margins according to CSS rules
    pub fn collapse(margin1: f32, margin2: f32) -> f32 {
        if margin1 >= 0.0 && margin2 >= 0.0 {
            margin1.max(margin2)
        } else if margin1 < 0.0 && margin2 < 0.0 {
            margin1.min(margin2)
        } else {
            margin1 + margin2
        }
    }
}
