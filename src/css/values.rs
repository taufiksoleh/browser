//! CSS Value types

/// CSS Color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn transparent() -> Self {
        Self { r: 0, g: 0, b: 0, a: 0 }
    }

    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 128, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);

    /// Parse named color
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "black" => Some(Self::BLACK),
            "white" => Some(Self::WHITE),
            "red" => Some(Self::RED),
            "green" => Some(Self::GREEN),
            "blue" => Some(Self::BLUE),
            "transparent" => Some(Self::transparent()),
            _ => None,
        }
    }

    /// Parse hex color
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
                Some(Self::rgb(r, g, b))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Self::rgba(r, g, b, a))
            }
            _ => None,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

/// CSS Length unit
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Px(f32),
    Em(f32),
    Rem(f32),
    Percent(f32),
    Vw(f32),
    Vh(f32),
    Auto,
    Zero,
}

impl Length {
    /// Convert to pixels given context
    pub fn to_px(&self, parent_font_size: f32, root_font_size: f32, viewport: (f32, f32)) -> f32 {
        match self {
            Length::Px(v) => *v,
            Length::Em(v) => v * parent_font_size,
            Length::Rem(v) => v * root_font_size,
            Length::Percent(v) => v, // Percentage handled by layout
            Length::Vw(v) => v * viewport.0 / 100.0,
            Length::Vh(v) => v * viewport.1 / 100.0,
            Length::Auto => 0.0,
            Length::Zero => 0.0,
        }
    }

    /// Parse length from string
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if s == "auto" {
            return Some(Length::Auto);
        }
        if s == "0" {
            return Some(Length::Zero);
        }

        if let Some(v) = s.strip_suffix("px") {
            return v.parse().ok().map(Length::Px);
        }
        if let Some(v) = s.strip_suffix("em") {
            return v.parse().ok().map(Length::Em);
        }
        if let Some(v) = s.strip_suffix("rem") {
            return v.parse().ok().map(Length::Rem);
        }
        if let Some(v) = s.strip_suffix('%') {
            return v.parse().ok().map(Length::Percent);
        }
        if let Some(v) = s.strip_suffix("vw") {
            return v.parse().ok().map(Length::Vw);
        }
        if let Some(v) = s.strip_suffix("vh") {
            return v.parse().ok().map(Length::Vh);
        }

        // Try parsing as number (assume px)
        s.parse().ok().map(Length::Px)
    }
}

impl Default for Length {
    fn default() -> Self {
        Length::Zero
    }
}

/// CSS Display value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    Grid,
    None,
    Contents,
}

impl Display {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "block" => Some(Display::Block),
            "inline" => Some(Display::Inline),
            "inline-block" => Some(Display::InlineBlock),
            "flex" => Some(Display::Flex),
            "grid" => Some(Display::Grid),
            "none" => Some(Display::None),
            "contents" => Some(Display::Contents),
            _ => None,
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Display::Inline
    }
}

/// Generic CSS value
#[derive(Debug, Clone, PartialEq)]
pub enum CssValue {
    Keyword(String),
    Color(Color),
    Length(Length),
    Number(f32),
    String(String),
    Inherit,
    Initial,
    Unset,
}

impl CssValue {
    pub fn as_color(&self) -> Option<Color> {
        match self {
            CssValue::Color(c) => Some(*c),
            CssValue::Keyword(s) => Color::from_name(s).or_else(|| Color::from_hex(s)),
            _ => None,
        }
    }

    pub fn as_length(&self) -> Option<Length> {
        match self {
            CssValue::Length(l) => Some(*l),
            CssValue::Number(n) if *n == 0.0 => Some(Length::Zero),
            CssValue::Keyword(s) => Length::parse(s),
            _ => None,
        }
    }
}
