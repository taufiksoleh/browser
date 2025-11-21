//! Painter - Generates display list from layout tree

use crate::css::{Color, StyleContext, Display};
use crate::dom::Document;
use crate::layout::{LayoutTree, LayoutId, LayoutNode, Rect};
use crate::layout::box_model::BoxType;
use crate::render::{DisplayList, DisplayCommand, DisplayItem};

/// Painter generates display list from layout
pub struct Painter<'a> {
    document: &'a Document,
    styles: &'a StyleContext,
    display_list: DisplayList,
    current_z: i32,
}

impl<'a> Painter<'a> {
    pub fn new(document: &'a Document, styles: &'a StyleContext) -> Self {
        Self {
            document,
            styles,
            display_list: DisplayList::new(),
            current_z: 0,
        }
    }

    /// Paint the entire layout tree
    pub fn paint(&mut self, layout: &LayoutTree) -> DisplayList {
        self.display_list.clear();

        // Paint from root
        let root = layout.root();
        self.paint_node(layout, root);

        // Sort by z-index
        self.display_list.sort();

        std::mem::take(&mut self.display_list)
    }

    /// Paint a single layout node
    fn paint_node(&mut self, layout: &LayoutTree, id: LayoutId) {
        let node = match layout.get_node(id) {
            Some(n) => n,
            None => return,
        };

        let dims = &node.box_model.dimensions;

        // Get style for this node
        let (bg_color, text_color) = if let Some(dom_id) = node.dom_node {
            if let Some(style) = self.styles.get_computed_style(dom_id) {
                (style.background_color(), style.color())
            } else {
                (Color::transparent(), Color::BLACK)
            }
        } else {
            (Color::transparent(), Color::BLACK)
        };

        // Paint background
        if bg_color.a > 0 {
            let bg_rect = dims.padding_box();
            self.display_list.push_rect(bg_rect, bg_color, self.current_z);
        }

        // Paint borders
        self.paint_borders(node);

        // Paint text content
        if let Some(ref text) = node.text {
            if !text.trim().is_empty() {
                self.display_list.push_text(
                    text.clone(),
                    dims.content.x,
                    dims.content.y + 16.0, // Baseline offset
                    text_color,
                    16.0,
                    self.current_z + 1,
                );
            }
        }

        // Paint children
        self.current_z += 1;
        for &child_id in &node.children {
            self.paint_node(layout, child_id);
        }
    }

    /// Paint borders for a node
    fn paint_borders(&mut self, node: &LayoutNode) {
        let dims = &node.box_model.dimensions;
        let border = &dims.border;

        if border.top == 0.0 && border.right == 0.0 && border.bottom == 0.0 && border.left == 0.0 {
            return;
        }

        let border_color = Color::BLACK; // Default border color
        let border_rect = dims.border_box();

        self.display_list.push_border(
            border_rect,
            (border.top, border.right, border.bottom, border.left),
            (border_color, border_color, border_color, border_color),
            self.current_z,
        );
    }
}

/// Paint background with gradient or image (placeholder for future)
pub fn paint_background(rect: Rect, color: Color) -> DisplayCommand {
    DisplayCommand::SolidRect { rect, color }
}
