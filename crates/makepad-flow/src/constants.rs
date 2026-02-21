//! Constants for FlowCanvas configuration
//!
//! This module centralizes all magic numbers used throughout the flow canvas,
//! making it easier to maintain consistent styling and behavior.

/// Node layout constants
pub mod node {
    /// Default node width in pixels
    pub const WIDTH: f64 = 180.0;
    /// Minimum node height in pixels
    pub const MIN_HEIGHT: f64 = 80.0;
    /// Camera node height in pixels
    pub const CAMERA_HEIGHT: f64 = 80.0;
    /// Default node height in pixels
    pub const DEFAULT_HEIGHT: f64 = 100.0;
    /// Node header height in pixels
    pub const HEADER_HEIGHT: f64 = 32.0;
    /// Height per port row in pixels
    pub const PORT_HEIGHT: f64 = 20.0;
    /// Padding below ports in pixels
    pub const BODY_PADDING: f64 = 16.0;
    /// Corner radius for rounded nodes
    pub const CORNER_RADIUS: f64 = 8.0;
    /// Default border width in pixels
    pub const BORDER_WIDTH: f32 = 2.0;
    /// Legacy Y offset for single-port nodes
    pub const LEGACY_PORT_Y_OFFSET: f64 = 54.0;
}

/// Port rendering constants
pub mod port {
    /// Port circle radius in pixels
    pub const RADIUS: f64 = 5.0;
    /// Port hit area size for click detection
    pub const HIT_SIZE: f64 = 18.0;
    /// Port hit area X offset (for output ports)
    pub const HIT_OFFSET_X: f64 = 12.0;
    /// Port hit area Y offset
    pub const HIT_OFFSET_Y: f64 = 6.0;
    /// Gap between port circle and label
    pub const LABEL_GAP: f64 = 4.0;
    /// Vertical offset for port label text
    pub const LABEL_Y_OFFSET: f64 = 5.0;
}

/// Edge rendering constants
pub mod edge {
    /// Default edge line width in pixels
    pub const WIDTH: f64 = 2.0;
    /// Default edge line width as f32 (for EdgeConnection struct)
    pub const WIDTH_F32: f32 = 2.0;
    /// Number of segments for bezier curve rendering
    pub const BEZIER_SEGMENTS: usize = 100;
    /// Distance threshold for edge hit detection
    pub const HIT_DISTANCE: f64 = 8.0;
    /// Dash length for dashed lines
    pub const DASH_LENGTH: f64 = 12.0;
    /// Gap between dashes
    pub const DASH_GAP: f64 = 8.0;
    /// Spacing between dots for dotted lines
    pub const DOT_SPACING: f64 = 12.0;
    /// Arrow marker size multiplier (relative to line thickness)
    pub const ARROW_SIZE_MULTIPLIER: f64 = 4.0;
    /// Particle size multiplier for animated edges
    pub const PARTICLE_SIZE_MULTIPLIER: f64 = 2.0;
}

/// Canvas interaction constants
pub mod canvas {
    /// Minimum zoom level
    pub const MIN_ZOOM: f64 = 0.25;
    /// Maximum zoom level
    pub const MAX_ZOOM: f64 = 4.0;
    /// Padding around content for fit_view
    pub const FIT_VIEW_PADDING: f64 = 50.0;
    /// Maximum undo history size
    pub const UNDO_STACK_SIZE: usize = 50;
}

/// Context menu constants
pub mod menu {
    /// Small padding for menus
    pub const PADDING_SMALL: f64 = 4.0;
    /// Standard padding for menus
    pub const PADDING: f64 = 8.0;
    /// Text left margin in menu items
    pub const TEXT_MARGIN: f64 = 8.0;
    /// Indented text margin (for sub-items)
    pub const TEXT_INDENT: f64 = 12.0;
    /// Checkbox/indicator Y offset
    pub const INDICATOR_Y_OFFSET: f64 = 2.0;
}

/// Edge label constants
pub mod label {
    /// Approximate character width for label sizing
    pub const CHAR_WIDTH: f64 = 6.0;
    /// Label background horizontal padding
    pub const BG_PADDING_X: f64 = 4.0;
    /// Label background vertical padding
    pub const BG_PADDING_Y: f64 = 8.0;
    /// Label height
    pub const HEIGHT: f64 = 16.0;
    /// Label text Y offset
    pub const TEXT_Y_OFFSET: f64 = 6.0;
}

/// Shape rendering constants (for Round and Diamond shapes)
pub mod shape {
    /// Number of segments for circular shapes
    pub const CIRCLE_SEGMENTS: usize = 72;
    /// Pixel step size for shape filling
    pub const FILL_STEP: f64 = 4.0;
    /// Small pixel size for shape rendering
    pub const PIXEL_SIZE: f64 = 4.0;
    /// Line step for diamond border
    pub const LINE_STEP: f64 = 2.0;
    /// Steps divisor for diamond border
    pub const BORDER_STEPS_DIVISOR: f64 = 3.0;
}
