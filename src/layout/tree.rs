//! Layout Tree
//!
//! Parallel structure to the DOM tree containing layout information.

use crate::dom::{Document, NodeId, NodeType};
use crate::css::{StyleContext, Display, Length, ComputedStyle};
use crate::layout::{BoxModel, Dimensions, EdgeSizes, Rect};
use crate::layout::box_model::BoxType;
use smallvec::SmallVec;

/// Layout node identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayoutId(pub usize);

/// A node in the layout tree
#[derive(Debug)]
pub struct LayoutNode {
    /// Layout ID
    pub id: LayoutId,
    /// Corresponding DOM node (if any)
    pub dom_node: Option<NodeId>,
    /// Box model
    pub box_model: BoxModel,
    /// Children
    pub children: SmallVec<[LayoutId; 8]>,
    /// Parent
    pub parent: Option<LayoutId>,
    /// Text content (for text nodes)
    pub text: Option<String>,
}

impl LayoutNode {
    pub fn new(id: LayoutId, dom_node: Option<NodeId>, box_type: BoxType) -> Self {
        Self {
            id,
            dom_node,
            box_model: BoxModel::new(box_type),
            children: SmallVec::new(),
            parent: None,
            text: None,
        }
    }

    pub fn dimensions(&self) -> &Dimensions {
        &self.box_model.dimensions
    }

    pub fn dimensions_mut(&mut self) -> &mut Dimensions {
        &mut self.box_model.dimensions
    }
}

/// The layout tree
#[derive(Debug)]
pub struct LayoutTree {
    /// All layout nodes
    nodes: Vec<LayoutNode>,
    /// Root node
    root: LayoutId,
    /// Viewport dimensions
    viewport: (f32, f32),
}

impl LayoutTree {
    /// Build layout tree from DOM and styles
    pub fn build(doc: &Document, styles: &StyleContext) -> Self {
        let mut tree = Self {
            nodes: Vec::with_capacity(doc.node_count()),
            root: LayoutId(0),
            viewport: (1920.0, 1080.0), // Default viewport
        };

        // Create root layout node
        let root_id = tree.create_node(None, BoxType::Block);
        tree.root = root_id;

        // Build tree from body element
        if let Some(body_id) = doc.body() {
            tree.build_recursive(doc, styles, body_id, root_id);
        }

        // Perform layout
        tree.layout();

        tree
    }

    /// Set viewport dimensions
    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.viewport = (width, height);
    }

    /// Get viewport dimensions
    pub fn viewport(&self) -> (f32, f32) {
        self.viewport
    }

    /// Create a new layout node
    fn create_node(&mut self, dom_node: Option<NodeId>, box_type: BoxType) -> LayoutId {
        let id = LayoutId(self.nodes.len());
        self.nodes.push(LayoutNode::new(id, dom_node, box_type));
        id
    }

    /// Get a node by ID
    pub fn get_node(&self, id: LayoutId) -> Option<&LayoutNode> {
        self.nodes.get(id.0)
    }

    /// Get a node mutably
    fn get_node_mut(&mut self, id: LayoutId) -> Option<&mut LayoutNode> {
        self.nodes.get_mut(id.0)
    }

    /// Build layout tree recursively
    fn build_recursive(
        &mut self,
        doc: &Document,
        styles: &StyleContext,
        dom_id: NodeId,
        parent_id: LayoutId,
    ) {
        let node = match doc.get_node(dom_id) {
            Some(n) => n,
            None => return,
        };

        // Skip non-rendered elements
        if let Some(style) = styles.get_computed_style(dom_id) {
            if style.display() == Display::None {
                return;
            }
        }

        // Determine box type
        let box_type = match node.node_type() {
            NodeType::Element => {
                if let Some(style) = styles.get_computed_style(dom_id) {
                    match style.display() {
                        Display::Block | Display::Flex | Display::Grid => BoxType::Block,
                        Display::Inline => BoxType::Inline,
                        Display::InlineBlock => BoxType::InlineBlock,
                        Display::None => return,
                        Display::Contents => {
                            // Skip this node but process children
                            for &child_id in &node.children {
                                self.build_recursive(doc, styles, child_id, parent_id);
                            }
                            return;
                        }
                    }
                } else {
                    BoxType::Block
                }
            }
            NodeType::Text => BoxType::Text,
            _ => return,
        };

        // Create layout node
        let layout_id = self.create_node(Some(dom_id), box_type);

        // Set text content for text nodes
        if box_type == BoxType::Text {
            if let Some(text) = node.text_content() {
                if let Some(layout_node) = self.get_node_mut(layout_id) {
                    layout_node.text = Some(text.to_string());
                }
            }
        }

        // Apply dimensions from style
        if let Some(style) = styles.get_computed_style(dom_id) {
            self.apply_style_dimensions(layout_id, style);
        }

        // Add to parent
        if let Some(parent) = self.get_node_mut(parent_id) {
            parent.children.push(layout_id);
        }
        if let Some(layout_node) = self.get_node_mut(layout_id) {
            layout_node.parent = Some(parent_id);
        }

        // Process children
        for &child_id in &node.children.clone() {
            self.build_recursive(doc, styles, child_id, layout_id);
        }
    }

    /// Apply style dimensions to layout node
    fn apply_style_dimensions(&mut self, id: LayoutId, style: &ComputedStyle) {
        let node = match self.get_node_mut(id) {
            Some(n) => n,
            None => return,
        };

        let dims = &mut node.box_model.dimensions;

        // Margins
        let (mt, mr, mb, ml) = style.margin();
        dims.margin = EdgeSizes::new(
            length_to_px(mt),
            length_to_px(mr),
            length_to_px(mb),
            length_to_px(ml),
        );

        // Padding
        let (pt, pr, pb, pl) = style.padding();
        dims.padding = EdgeSizes::new(
            length_to_px(pt),
            length_to_px(pr),
            length_to_px(pb),
            length_to_px(pl),
        );

        // Explicit dimensions
        if let Some(w) = style.width() {
            dims.content.width = length_to_px(w);
        }
        if let Some(h) = style.height() {
            dims.content.height = length_to_px(h);
        }
    }

    /// Perform layout calculations
    pub fn layout(&mut self) {
        let viewport_width = self.viewport.0;

        // Set root dimensions to viewport
        if let Some(root) = self.get_node_mut(self.root) {
            root.box_model.dimensions.content.width = viewport_width;
            root.box_model.dimensions.content.x = 0.0;
            root.box_model.dimensions.content.y = 0.0;
        }

        // Layout children
        let root = self.root;
        self.layout_block(root);
    }

    /// Layout a block node
    fn layout_block(&mut self, id: LayoutId) {
        // Calculate width first
        self.calculate_block_width(id);

        // Layout children
        self.layout_block_children(id);

        // Calculate height based on children
        self.calculate_block_height(id);
    }

    /// Calculate block width
    fn calculate_block_width(&mut self, id: LayoutId) {
        let parent_width = self.get_node(id)
            .and_then(|n| n.parent)
            .and_then(|p| self.get_node(p))
            .map(|p| p.box_model.dimensions.content.width)
            .unwrap_or(self.viewport.0);

        if let Some(node) = self.get_node_mut(id) {
            let dims = &mut node.box_model.dimensions;

            // Auto width fills parent
            if dims.content.width == 0.0 {
                dims.content.width = parent_width
                    - dims.margin.horizontal()
                    - dims.padding.horizontal()
                    - dims.border.horizontal();
            }
        }
    }

    /// Layout block children
    fn layout_block_children(&mut self, id: LayoutId) {
        // Get current y position
        let (start_x, start_y) = {
            let node = match self.get_node(id) {
                Some(n) => n,
                None => return,
            };
            let dims = &node.box_model.dimensions;
            (
                dims.content.x + dims.padding.left,
                dims.content.y + dims.padding.top,
            )
        };

        let mut cursor_y = start_y;

        // Get children IDs
        let children: Vec<_> = self.get_node(id)
            .map(|n| n.children.iter().copied().collect())
            .unwrap_or_default();

        for child_id in children {
            let child_type = self.get_node(child_id)
                .map(|n| n.box_model.box_type)
                .unwrap_or(BoxType::Block);

            match child_type {
                BoxType::Block | BoxType::InlineBlock => {
                    // Position child
                    if let Some(child) = self.get_node_mut(child_id) {
                        let dims = &mut child.box_model.dimensions;
                        dims.content.x = start_x + dims.margin.left;
                        dims.content.y = cursor_y + dims.margin.top;
                    }

                    // Layout child
                    self.layout_block(child_id);

                    // Update cursor
                    if let Some(child) = self.get_node(child_id) {
                        cursor_y += child.box_model.dimensions.total_height();
                    }
                }
                BoxType::Inline | BoxType::Text => {
                    // Simple inline/text positioning
                    if let Some(child) = self.get_node_mut(child_id) {
                        child.box_model.dimensions.content.x = start_x;
                        child.box_model.dimensions.content.y = cursor_y;

                        // Estimate text height
                        if child.text.is_some() {
                            child.box_model.dimensions.content.height = 20.0; // Line height
                            cursor_y += 20.0;
                        }
                    }
                }
                BoxType::Anonymous => {}
            }
        }
    }

    /// Calculate block height
    fn calculate_block_height(&mut self, id: LayoutId) {
        // Get children bounds
        let children: Vec<_> = self.get_node(id)
            .map(|n| n.children.iter().copied().collect())
            .unwrap_or_default();

        let mut max_bottom = 0.0f32;
        for child_id in children {
            if let Some(child) = self.get_node(child_id) {
                let child_bottom = child.box_model.dimensions.margin_box().bottom();
                max_bottom = max_bottom.max(child_bottom);
            }
        }

        if let Some(node) = self.get_node_mut(id) {
            let dims = &mut node.box_model.dimensions;
            if dims.content.height == 0.0 {
                dims.content.height = max_bottom - dims.content.y - dims.padding.vertical();
                if dims.content.height < 0.0 {
                    dims.content.height = 0.0;
                }
            }
        }
    }

    /// Get root layout node
    pub fn root(&self) -> LayoutId {
        self.root
    }

    /// Iterate over all layout nodes
    pub fn nodes(&self) -> impl Iterator<Item = &LayoutNode> {
        self.nodes.iter()
    }
}

/// Convert length to pixels (simplified)
fn length_to_px(length: Length) -> f32 {
    match length {
        Length::Px(v) => v,
        Length::Em(v) => v * 16.0, // Assume 16px base
        Length::Rem(v) => v * 16.0,
        Length::Percent(_) => 0.0, // Handled separately
        Length::Vw(_) => 0.0,
        Length::Vh(_) => 0.0,
        Length::Auto => 0.0,
        Length::Zero => 0.0,
    }
}
