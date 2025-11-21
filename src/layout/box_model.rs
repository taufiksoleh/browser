//! CSS Box Model implementation

/// Rectangle with position and size
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    /// Check if point is inside rectangle
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    /// Expand rectangle by amounts on each side
    pub fn expand(&self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            x: self.x - left,
            y: self.y - top,
            width: self.width + left + right,
            height: self.height + top + bottom,
        }
    }

    /// Get right edge
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Get bottom edge
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

/// Edge sizes (margin, padding, border)
#[derive(Debug, Clone, Copy, Default)]
pub struct EdgeSizes {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeSizes {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }

    pub fn uniform(size: f32) -> Self {
        Self::new(size, size, size, size)
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn horizontal(&self) -> f32 {
        self.left + self.right
    }

    pub fn vertical(&self) -> f32 {
        self.top + self.bottom
    }
}

/// Complete box model dimensions
#[derive(Debug, Clone, Default)]
pub struct Dimensions {
    /// Content area
    pub content: Rect,
    /// Padding
    pub padding: EdgeSizes,
    /// Border
    pub border: EdgeSizes,
    /// Margin
    pub margin: EdgeSizes,
}

impl Dimensions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get padding box (content + padding)
    pub fn padding_box(&self) -> Rect {
        self.content.expand(
            self.padding.top,
            self.padding.right,
            self.padding.bottom,
            self.padding.left,
        )
    }

    /// Get border box (content + padding + border)
    pub fn border_box(&self) -> Rect {
        let padding = self.padding_box();
        padding.expand(
            self.border.top,
            self.border.right,
            self.border.bottom,
            self.border.left,
        )
    }

    /// Get margin box (content + padding + border + margin)
    pub fn margin_box(&self) -> Rect {
        let border = self.border_box();
        border.expand(
            self.margin.top,
            self.margin.right,
            self.margin.bottom,
            self.margin.left,
        )
    }

    /// Total width including margins
    pub fn total_width(&self) -> f32 {
        self.content.width
            + self.padding.horizontal()
            + self.border.horizontal()
            + self.margin.horizontal()
    }

    /// Total height including margins
    pub fn total_height(&self) -> f32 {
        self.content.height
            + self.padding.vertical()
            + self.border.vertical()
            + self.margin.vertical()
    }
}

/// Box type for layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxType {
    Block,
    Inline,
    InlineBlock,
    Anonymous,
    Text,
}

/// Complete box model
#[derive(Debug, Clone)]
pub struct BoxModel {
    /// Box type
    pub box_type: BoxType,
    /// Dimensions
    pub dimensions: Dimensions,
}

impl BoxModel {
    pub fn new(box_type: BoxType) -> Self {
        Self {
            box_type,
            dimensions: Dimensions::new(),
        }
    }

    pub fn block() -> Self {
        Self::new(BoxType::Block)
    }

    pub fn inline() -> Self {
        Self::new(BoxType::Inline)
    }

    pub fn text() -> Self {
        Self::new(BoxType::Text)
    }
}
