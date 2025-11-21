//! CSS Property definitions

/// CSS Property identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyId {
    // Display & Box Model
    Display,
    Position,
    Float,
    Clear,

    // Dimensions
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,

    // Margins
    Margin,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,

    // Padding
    Padding,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,

    // Borders
    BorderWidth,
    BorderTopWidth,
    BorderRightWidth,
    BorderBottomWidth,
    BorderLeftWidth,
    BorderColor,
    BorderTopColor,
    BorderRightColor,
    BorderBottomColor,
    BorderLeftColor,
    BorderStyle,
    BorderRadius,

    // Colors
    Color,
    BackgroundColor,
    Background,

    // Text
    FontFamily,
    FontSize,
    FontWeight,
    FontStyle,
    LineHeight,
    TextAlign,
    TextDecoration,
    TextTransform,
    LetterSpacing,
    WordSpacing,
    WhiteSpace,

    // Flexbox
    FlexDirection,
    FlexWrap,
    JustifyContent,
    AlignItems,
    AlignContent,
    FlexGrow,
    FlexShrink,
    FlexBasis,

    // Grid
    GridTemplateColumns,
    GridTemplateRows,
    GridColumn,
    GridRow,
    Gap,

    // Positioning
    Top,
    Right,
    Bottom,
    Left,
    ZIndex,

    // Other
    Overflow,
    OverflowX,
    OverflowY,
    Visibility,
    Opacity,
    Cursor,
    BoxSizing,

    // Unknown/Custom
    Unknown,
}

impl PropertyId {
    /// Parse property name to ID
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().replace('_', "-").as_str() {
            "display" => Self::Display,
            "position" => Self::Position,
            "float" => Self::Float,
            "clear" => Self::Clear,

            "width" => Self::Width,
            "height" => Self::Height,
            "min-width" => Self::MinWidth,
            "min-height" => Self::MinHeight,
            "max-width" => Self::MaxWidth,
            "max-height" => Self::MaxHeight,

            "margin" => Self::Margin,
            "margin-top" => Self::MarginTop,
            "margin-right" => Self::MarginRight,
            "margin-bottom" => Self::MarginBottom,
            "margin-left" => Self::MarginLeft,

            "padding" => Self::Padding,
            "padding-top" => Self::PaddingTop,
            "padding-right" => Self::PaddingRight,
            "padding-bottom" => Self::PaddingBottom,
            "padding-left" => Self::PaddingLeft,

            "border-width" => Self::BorderWidth,
            "border-top-width" => Self::BorderTopWidth,
            "border-right-width" => Self::BorderRightWidth,
            "border-bottom-width" => Self::BorderBottomWidth,
            "border-left-width" => Self::BorderLeftWidth,
            "border-color" => Self::BorderColor,
            "border-top-color" => Self::BorderTopColor,
            "border-right-color" => Self::BorderRightColor,
            "border-bottom-color" => Self::BorderBottomColor,
            "border-left-color" => Self::BorderLeftColor,
            "border-style" => Self::BorderStyle,
            "border-radius" => Self::BorderRadius,

            "color" => Self::Color,
            "background-color" => Self::BackgroundColor,
            "background" => Self::Background,

            "font-family" => Self::FontFamily,
            "font-size" => Self::FontSize,
            "font-weight" => Self::FontWeight,
            "font-style" => Self::FontStyle,
            "line-height" => Self::LineHeight,
            "text-align" => Self::TextAlign,
            "text-decoration" => Self::TextDecoration,
            "text-transform" => Self::TextTransform,
            "letter-spacing" => Self::LetterSpacing,
            "word-spacing" => Self::WordSpacing,
            "white-space" => Self::WhiteSpace,

            "flex-direction" => Self::FlexDirection,
            "flex-wrap" => Self::FlexWrap,
            "justify-content" => Self::JustifyContent,
            "align-items" => Self::AlignItems,
            "align-content" => Self::AlignContent,
            "flex-grow" => Self::FlexGrow,
            "flex-shrink" => Self::FlexShrink,
            "flex-basis" => Self::FlexBasis,

            "grid-template-columns" => Self::GridTemplateColumns,
            "grid-template-rows" => Self::GridTemplateRows,
            "grid-column" => Self::GridColumn,
            "grid-row" => Self::GridRow,
            "gap" => Self::Gap,

            "top" => Self::Top,
            "right" => Self::Right,
            "bottom" => Self::Bottom,
            "left" => Self::Left,
            "z-index" => Self::ZIndex,

            "overflow" => Self::Overflow,
            "overflow-x" => Self::OverflowX,
            "overflow-y" => Self::OverflowY,
            "visibility" => Self::Visibility,
            "opacity" => Self::Opacity,
            "cursor" => Self::Cursor,
            "box-sizing" => Self::BoxSizing,

            _ => Self::Unknown,
        }
    }

    /// Check if property is inherited by default
    pub fn is_inherited(&self) -> bool {
        matches!(
            self,
            Self::Color
                | Self::FontFamily
                | Self::FontSize
                | Self::FontWeight
                | Self::FontStyle
                | Self::LineHeight
                | Self::TextAlign
                | Self::TextDecoration
                | Self::TextTransform
                | Self::LetterSpacing
                | Self::WordSpacing
                | Self::WhiteSpace
                | Self::Visibility
                | Self::Cursor
        )
    }
}
