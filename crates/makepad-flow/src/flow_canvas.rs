use makepad_widgets::*;
use std::collections::HashSet;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Custom rounded rectangle shader using SDF (all corners rounded)
    DrawRoundedRect = {{DrawRoundedRect}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let inset = self.border_width * 0.5;
            sdf.box(
                inset, inset,
                self.rect_size.x - inset * 2.0,
                self.rect_size.y - inset * 2.0,
                max(0.0, self.radius - inset)
            );
            sdf.fill(self.color);
            if self.border_width > 0.5 {
                sdf.stroke(self.border_color, self.border_width);
            }
            return sdf.result;
        }
    }

    // Rounded TOP corners only (for header)
    DrawRoundedTopRect = {{DrawRoundedTopRect}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let r = self.radius;
            let w = self.rect_size.x;
            let h = self.rect_size.y;

            // Main body rect - full width, starts slightly above where circles end
            sdf.rect(0.0, r - 1.0, w, h - r + 1.0);
            sdf.fill_keep(self.color);

            // Top-left corner circle
            sdf.circle(r, r, r);
            sdf.fill_keep(self.color);

            // Top-right corner circle
            sdf.circle(w - r, r, r);
            sdf.fill_keep(self.color);

            // Top edge rect between corners - overlaps with circles
            sdf.rect(r - 1.0, 0.0, w - r * 2.0 + 2.0, r + 1.0);
            sdf.fill(self.color);

            return sdf.result;
        }
    }

    // Rounded BOTTOM corners only (for body below header)
    DrawRoundedBottomRect = {{DrawRoundedBottomRect}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let r = self.radius;
            let w = self.rect_size.x;
            let h = self.rect_size.y;

            // Main body rect - full width, ends slightly below where circles start
            sdf.rect(0.0, 0.0, w, h - r + 1.0);
            sdf.fill_keep(self.color);

            // Bottom-left corner circle
            sdf.circle(r, h - r, r);
            sdf.fill_keep(self.color);

            // Bottom-right corner circle
            sdf.circle(w - r, h - r, r);
            sdf.fill_keep(self.color);

            // Bottom edge rect between corners - overlaps with circles
            sdf.rect(r - 1.0, h - r - 1.0, w - r * 2.0 + 2.0, r + 1.0);
            sdf.fill(self.color);

            return sdf.result;
        }
    }

    // Node templates (light theme)
    NodeCamera = <RoundedView> {
        width: 180, height: Fit, flow: Down
        draw_bg: { color: #ffffff, border_radius: 8.0, border_size: 1.0, border_color: #e0e0e0 }
        cursor: Hand

        header = <RoundedView> {
            width: Fill, height: 32
            padding: { left: 10, right: 10 }, align: { y: 0.5 }
            draw_bg: { color: #f8f8f8, border_radius: 8.0 }
            <Label> { draw_text: { text_style: { font_size: 11.0 }, color: #333333 }, text: "Camera" }
        }
        <View> {
            width: Fill, height: Fit
            padding: 8, flow: Right, align: { x: 1.0, y: 0.5 }, spacing: 6
            <Label> { draw_text: { text_style: { font_size: 10.0 }, color: #666666 }, text: "image" }
            output_port = <RoundedView> { width: 12, height: 12, draw_bg: { color: #22c55e, border_radius: 6.0 } }
        }
    }

    NodeProcessor = <RoundedView> {
        width: 180, height: Fit, flow: Down
        draw_bg: { color: #ffffff, border_radius: 8.0, border_size: 1.0, border_color: #e0e0e0 }
        cursor: Hand

        header = <RoundedView> {
            width: Fill, height: 32
            padding: { left: 10, right: 10 }, align: { y: 0.5 }
            draw_bg: { color: #f8f8f8, border_radius: 8.0 }
            title = <Label> { draw_text: { text_style: { font_size: 11.0 }, color: #333333 }, text: "Processor" }
        }
        <View> {
            width: Fill, height: Fit
            padding: 8, flow: Down, spacing: 4
            <View> {
                width: Fill, height: 20, flow: Right, align: { y: 0.5 }, spacing: 6
                input_port = <RoundedView> { width: 12, height: 12, draw_bg: { color: #3b82f6, border_radius: 6.0 } }
                <Label> { draw_text: { text_style: { font_size: 10.0 }, color: #666666 }, text: "input" }
            }
            <View> {
                width: Fill, height: 20, flow: Right, align: { x: 1.0, y: 0.5 }, spacing: 6
                <Label> { draw_text: { text_style: { font_size: 10.0 }, color: #666666 }, text: "output" }
                output_port = <RoundedView> { width: 12, height: 12, draw_bg: { color: #22c55e, border_radius: 6.0 } }
            }
        }
    }

    // Flow canvas (light theme)
    pub FlowCanvas = {{FlowCanvas}} {
        width: Fill, height: Fill
        flow: Overlay
        show_bg: true
        draw_bg: { color: #f0f0f0 }
        cursor: Arrow

        draw_text: {
            text_style: <THEME_FONT_REGULAR>{ font_size: 11.0 }
            color: #333333
        }

        // Configurable properties with defaults
        line_width: 2.0
        line_style: 0.0
        default_zoom: 1.0
        selection_color: #4A90D9
        edge_color: #8CBFFF
        edge_selected_color: #FFD966
        animate_edges: true
    }
}

// Custom rounded rectangle draw shader (all corners)
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawRoundedRect {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub border_color: Vec4,
    #[live] pub border_width: f32,
    #[live] pub radius: f32,
}

// Rounded TOP corners only (for header)
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawRoundedTopRect {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub radius: f32,
}

// Rounded BOTTOM corners only (for body)
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawRoundedBottomRect {
    #[deref] pub draw_super: DrawQuad,
    #[live] pub color: Vec4,
    #[live] pub radius: f32,
}

// Node shape types
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum NodeShape {
    #[default]
    RoundedRect,
    DoubleRoundedRect,
    Rectangle,
    Round,
    Diamond,
}

// Node category for coloring
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum NodeCategory {
    #[default]
    Default,
    MaaS,       // LLM clients (blue)
    TTS,        // Text-to-speech (green)
    Bridge,     // Message routing (orange)
    Controller, // Orchestration (purple)
    MoFA,       // Dynamic UI widgets (cyan)
    Segmenter,  // Text processing (yellow)
}

impl NodeCategory {
    pub fn color(&self) -> Vec4 {
        match self {
            NodeCategory::Default => vec4(0.24, 0.24, 0.36, 1.0),
            NodeCategory::MaaS => vec4(0.2, 0.4, 0.7, 1.0),
            NodeCategory::TTS => vec4(0.2, 0.6, 0.3, 1.0),
            NodeCategory::Bridge => vec4(0.7, 0.5, 0.2, 1.0),
            NodeCategory::Controller => vec4(0.5, 0.3, 0.7, 1.0),
            NodeCategory::MoFA => vec4(0.2, 0.6, 0.7, 1.0),
            NodeCategory::Segmenter => vec4(0.7, 0.7, 0.2, 1.0),
        }
    }

    pub fn header_color(&self) -> Vec4 {
        match self {
            NodeCategory::Default => vec4(0.30, 0.30, 0.45, 1.0),
            NodeCategory::MaaS => vec4(0.25, 0.45, 0.75, 1.0),
            NodeCategory::TTS => vec4(0.25, 0.65, 0.35, 1.0),
            NodeCategory::Bridge => vec4(0.75, 0.55, 0.25, 1.0),
            NodeCategory::Controller => vec4(0.55, 0.35, 0.75, 1.0),
            NodeCategory::MoFA => vec4(0.25, 0.65, 0.75, 1.0),
            NodeCategory::Segmenter => vec4(0.75, 0.75, 0.25, 1.0),
        }
    }
}

// Port definition
#[derive(Clone, Debug)]
pub struct Port {
    pub id: String,
    pub label: String,
}

impl Port {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            label: id.to_string(),
        }
    }
}

// Node types
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeType {
    Camera,
    Detector,
    Tracker,
    Visualizer,
    Custom,
}

impl NodeType {
    pub fn title(&self) -> &'static str {
        match self {
            NodeType::Camera => "Camera",
            NodeType::Detector => "Object Detector",
            NodeType::Tracker => "Tracker",
            NodeType::Visualizer => "Visualizer",
            NodeType::Custom => "Processor",
        }
    }

    pub fn has_input(&self) -> bool {
        !matches!(self, NodeType::Camera)
    }

    pub fn has_output(&self) -> bool {
        !matches!(self, NodeType::Visualizer)
    }
}

// Node data
#[derive(Clone, Debug)]
pub struct FlowNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub node_type: NodeType,
    pub title: String,
    pub shape: NodeShape,
    pub border_width: f32,
    pub category: NodeCategory,
    pub input_ports: Vec<Port>,
    pub output_ports: Vec<Port>,
}

impl FlowNode {
    pub fn new(x: f64, y: f64, node_type: NodeType) -> Self {
        let height = if node_type == NodeType::Camera { 80.0 } else { 100.0 };
        let (input_ports, output_ports) = match node_type {
            NodeType::Camera => (vec![], vec![Port::new("image")]),
            NodeType::Visualizer => (vec![Port::new("input")], vec![]),
            _ => (vec![Port::new("input")], vec![Port::new("output")]),
        };
        Self {
            id: format!("node_{}", x as i32 + y as i32),
            x, y,
            width: 180.0,
            height,
            node_type,
            title: node_type.title().to_string(),
            shape: NodeShape::RoundedRect,
            border_width: 2.0,
            category: NodeCategory::Default,
            input_ports,
            output_ports,
        }
    }

    pub fn new_dataflow(id: &str, x: f64, y: f64, title: &str, category: NodeCategory, inputs: Vec<Port>, outputs: Vec<Port>) -> Self {
        let port_count = inputs.len().max(outputs.len());
        let header_height = 32.0;
        let port_height = 20.0;
        let padding = 16.0;
        let height = header_height + (port_count as f64 * port_height) + padding;

        Self {
            id: id.to_string(),
            x, y,
            width: 180.0,
            height: height.max(80.0),
            node_type: NodeType::Custom,
            title: title.to_string(),
            shape: NodeShape::DoubleRoundedRect,
            border_width: 2.0,
            category,
            input_ports: inputs,
            output_ports: outputs,
        }
    }

    pub fn contains(&self, pos: DVec2) -> bool {
        pos.x >= self.x && pos.x <= self.x + self.width &&
        pos.y >= self.y && pos.y <= self.y + self.height
    }

    // Get position for a specific input port by index
    pub fn input_port_pos(&self, index: usize) -> DVec2 {
        let header_height = 32.0;
        let port_height = 20.0;
        let y = self.y + header_height + (index as f64 * port_height) + port_height / 2.0;
        DVec2 { x: self.x, y }
    }

    // Get position for a specific output port by index
    pub fn output_port_pos(&self, index: usize) -> DVec2 {
        let header_height = 32.0;
        let port_height = 20.0;
        let y = self.y + header_height + (index as f64 * port_height) + port_height / 2.0;
        DVec2 { x: self.x + self.width, y }
    }

    // Legacy: first output port position
    pub fn output_pos(&self) -> DVec2 {
        if self.output_ports.is_empty() {
            let y_offset = 54.0;
            DVec2 { x: self.x + self.width, y: self.y + y_offset }
        } else {
            self.output_port_pos(0)
        }
    }

    // Legacy: first input port position
    pub fn input_pos(&self) -> DVec2 {
        if self.input_ports.is_empty() {
            DVec2 { x: self.x, y: self.y + 54.0 }
        } else {
            self.input_port_pos(0)
        }
    }

    pub fn output_port_rect(&self) -> Rect {
        let pos = self.output_pos();
        Rect { pos: DVec2 { x: pos.x - 12.0, y: pos.y - 6.0 }, size: DVec2 { x: 18.0, y: 18.0 } }
    }

    pub fn input_port_rect(&self) -> Rect {
        let pos = self.input_pos();
        Rect { pos: DVec2 { x: pos.x - 6.0, y: pos.y - 6.0 }, size: DVec2 { x: 18.0, y: 18.0 } }
    }

    // Find port index by id
    pub fn input_port_index(&self, port_id: &str) -> Option<usize> {
        self.input_ports.iter().position(|p| p.id == port_id)
    }

    pub fn output_port_index(&self, port_id: &str) -> Option<usize> {
        self.output_ports.iter().position(|p| p.id == port_id)
    }
}

// Edge marker type
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum EdgeMarker {
    #[default]
    None,
    Arrow,
    ArrowFilled,
    Circle,
}

// Edge connection with per-edge properties
#[derive(Clone, Debug)]
pub struct EdgeConnection {
    pub from_node: usize,
    pub to_node: usize,
    pub from_port: String,  // output port id
    pub to_port: String,    // input port id
    pub style: f32,         // 0=solid, 1=dashed, 2=dotted
    pub width: f32,         // line width
    pub animated: bool,     // animation on/off
    pub label: String,      // edge label text
    pub marker_end: EdgeMarker, // marker at end
}

impl EdgeConnection {
    pub fn new(from_node: usize, to_node: usize) -> Self {
        Self {
            from_node,
            to_node,
            from_port: String::new(),
            to_port: String::new(),
            style: 0.0,      // solid by default
            width: 2.0,      // 2px default
            animated: true,  // animated by default
            label: String::new(),
            marker_end: EdgeMarker::Arrow, // arrow by default
        }
    }

    pub fn new_with_ports(from_node: usize, from_port: &str, to_node: usize, to_port: &str) -> Self {
        Self {
            from_node,
            to_node,
            from_port: from_port.to_string(),
            to_port: to_port.to_string(),
            style: 0.0,
            width: 2.0,
            animated: false,  // dataflow edges not animated by default
            label: String::new(),
            marker_end: EdgeMarker::Arrow,
        }
    }
}

// Bezier curve helper - computes points along the curve
pub struct BezierCurve;

impl BezierCurve {
    /// Get point at parameter t on cubic bezier curve
    pub fn point_at(t: f64, p0: DVec2, c0: DVec2, c1: DVec2, p1: DVec2) -> DVec2 {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        DVec2 {
            x: mt3 * p0.x + 3.0 * mt2 * t * c0.x + 3.0 * mt * t2 * c1.x + t3 * p1.x,
            y: mt3 * p0.y + 3.0 * mt2 * t * c0.y + 3.0 * mt * t2 * c1.y + t3 * p1.y,
        }
    }

    /// Generate points along bezier curve with horizontal tangent control points
    pub fn points_with_horizontal_tangents(from: DVec2, to: DVec2, segments: usize) -> Vec<DVec2> {
        let dx = (to.x - from.x).abs() * 0.5;
        let c0 = DVec2 { x: from.x + dx, y: from.y };
        let c1 = DVec2 { x: to.x - dx, y: to.y };

        let mut points = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let t = i as f64 / segments as f64;
            points.push(Self::point_at(t, from, c0, c1, to));
        }
        points
    }
}

// Drag state for edge creation
#[derive(Clone)]
pub enum DragState {
    None,
    DraggingNode { index: usize, offset: DVec2 },
    DraggingNodes { offsets: Vec<(usize, DVec2)> }, // Multi-node drag
    Panning { start: DVec2 },
    CreatingEdge { from_node: usize, is_output: bool, cursor_pos: DVec2 },
    SelectionBox { start: DVec2, current: DVec2 }, // Drag selection box
}

// History entry for undo/redo
#[derive(Clone)]
pub struct HistoryEntry {
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<EdgeConnection>,
}

// Flow canvas ref for external access
#[derive(Clone, Debug, DefaultNone)]
pub enum FlowCanvasCommand {
    None,
    AddNode,
    Delete,
    FitView,
    Clear,
    SetLineStyle(f32),
    SetLineWidth(f32),
    LoadDataflow { nodes: Vec<FlowNode>, edges: Vec<EdgeConnection> },
}

// Flow canvas widget
#[derive(Live, LiveHook, Widget)]
pub struct FlowCanvas {
    #[deref] view: View,
    #[live] draw_edge: DrawColor,
    #[live] draw_node_bg: DrawColor,
    #[live] draw_rounded_rect: DrawRoundedRect,
    #[live] draw_rounded_top_rect: DrawRoundedTopRect,
    #[live] draw_rounded_bottom_rect: DrawRoundedBottomRect,
    #[live] draw_text: DrawText,

    // Configurable visual properties (can be set via DSL)
    #[live(2.0)] pub line_width: f32,
    #[live(0.0)] pub line_style: f32,
    #[live(1.0)] pub default_zoom: f64,
    #[live] pub selection_color: Vec4,
    #[live] pub edge_color: Vec4,
    #[live] pub edge_selected_color: Vec4,
    #[live(true)] pub animate_edges: bool,

    #[rust] nodes: Vec<FlowNode>,
    #[rust] edges: Vec<EdgeConnection>,
    #[rust] drag_state: DragState,
    #[rust] selected_nodes: HashSet<usize>,  // Multi-selection support
    #[rust] selected_edges: HashSet<usize>,  // Multi-selection support
    #[rust] pan_offset: DVec2,
    #[rust] zoom: f64,
    #[rust] initialized: bool,
    #[rust] next_node_id: usize,
    #[rust] animation_timer: Timer,
    #[rust] animation_phase: f64, // 0.0 to 1.0, cycles continuously
    #[rust] context_menu_node: Option<usize>, // Which node is the context menu for
    #[rust] context_menu_edge: Option<usize>, // Which edge is the context menu for
    #[rust] context_menu_pos: DVec2, // Position to show context menu
    #[rust] undo_stack: Vec<HistoryEntry>,   // Undo history
    #[rust] redo_stack: Vec<HistoryEntry>,   // Redo history
}

impl Default for DragState {
    fn default() -> Self { DragState::None }
}

// Actions for the flow canvas
#[derive(Clone, Debug, DefaultNone)]
pub enum FlowCanvasAction {
    None,
    NodeAdded,
    NodeDeleted,
    EdgeCreated,
    EdgeDeleted,
    SelectionChanged,
    StatusUpdate { nodes: usize, edges: usize },
}

impl Widget for FlowCanvas {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        // Initialize if needed
        if !self.initialized {
            self.initialize(cx);
            // Send initial status
            cx.widget_action(uid, &scope.path, FlowCanvasAction::StatusUpdate {
                nodes: self.nodes.len(),
                edges: self.edges.len(),
            });
        }

        // Handle animation timer
        if self.animation_timer.is_event(event).is_some() {
            // Update animation phase (complete cycle every 2 seconds)
            self.animation_phase += 0.02; // ~50fps * 0.02 = 1.0 per second
            if self.animation_phase >= 1.0 {
                self.animation_phase -= 1.0;
            }
            self.view.redraw(cx);
        }

        let area_rect = self.view.area().rect(cx);

        // Handle keyboard events
        match event {
            Event::KeyDown(ke) => {
                // Delete key - delete selected
                if ke.key_code == KeyCode::Delete || ke.key_code == KeyCode::Backspace {
                    self.delete_selected(cx, uid, scope);
                    cx.widget_action(uid, &scope.path, FlowCanvasAction::StatusUpdate {
                        nodes: self.nodes.len(),
                        edges: self.edges.len(),
                    });
                }
                // Ctrl+A or Cmd+A - select all
                else if ke.key_code == KeyCode::KeyA && (ke.modifiers.control || ke.modifiers.logo) {
                    self.select_all(cx);
                    cx.widget_action(uid, &scope.path, FlowCanvasAction::SelectionChanged);
                }
                // Ctrl+Z or Cmd+Z - undo
                else if ke.key_code == KeyCode::KeyZ && (ke.modifiers.control || ke.modifiers.logo) && !ke.modifiers.shift {
                    self.undo(cx);
                    cx.widget_action(uid, &scope.path, FlowCanvasAction::StatusUpdate {
                        nodes: self.nodes.len(),
                        edges: self.edges.len(),
                    });
                }
                // Ctrl+Y/Cmd+Y or Ctrl+Shift+Z/Cmd+Shift+Z - redo
                else if (ke.key_code == KeyCode::KeyY && (ke.modifiers.control || ke.modifiers.logo)) ||
                        (ke.key_code == KeyCode::KeyZ && (ke.modifiers.control || ke.modifiers.logo) && ke.modifiers.shift) {
                    self.redo(cx);
                    cx.widget_action(uid, &scope.path, FlowCanvasAction::StatusUpdate {
                        nodes: self.nodes.len(),
                        edges: self.edges.len(),
                    });
                }
                // Escape - deselect all
                else if ke.key_code == KeyCode::Escape {
                    self.selected_nodes.clear();
                    self.selected_edges.clear();
                    self.context_menu_node = None;
                    self.context_menu_edge = None;
                    self.drag_state = DragState::None;
                    cx.widget_action(uid, &scope.path, FlowCanvasAction::SelectionChanged);
                    self.view.redraw(cx);
                }
            }
            _ => {}
        }

        // Handle mouse events with capture
        match event.hits_with_options(cx, self.view.area(), HitOptions::new().with_capture_overload(true)) {
            Hit::FingerDown(fe) => {
                let local = self.screen_to_canvas(fe.abs, area_rect);

                // Check if clicking on node context menu
                if let Some(node_idx) = self.context_menu_node {
                    if node_idx < self.nodes.len() {
                        let menu_pos = self.context_menu_pos;
                        let menu_width = 120.0;
                        let item_height = 22.0;
                        let padding = 4.0;

                        // Check if multi-selection mode
                        let is_multi = self.selected_nodes.contains(&node_idx) && self.selected_nodes.len() > 1;
                        let header_offset = if is_multi { item_height } else { 0.0 };

                        // Check if click is within menu bounds
                        let in_menu_x = fe.abs.x >= menu_pos.x && fe.abs.x <= menu_pos.x + menu_width;
                        let num_items = if is_multi { 12.0 } else { 11.0 };
                        let menu_height = item_height * num_items + padding * 2.0;
                        let in_menu_y = fe.abs.y >= menu_pos.y && fe.abs.y <= menu_pos.y + menu_height;

                        if in_menu_x && in_menu_y {
                            // Calculate which item was clicked
                            let rel_y = fe.abs.y - menu_pos.y - padding - header_offset;
                            let label_height = item_height * 0.8;

                            // Shape section: label + 5 items
                            let shape_start = label_height;
                            let shape_end = shape_start + item_height * 5.0;

                            // Border section: divider + label + 4 items
                            let border_label_start = shape_end + item_height * 0.5;
                            let border_start = border_label_start + label_height;

                            if rel_y >= shape_start && rel_y < shape_end {
                                // Shape item clicked
                                let shape_idx = ((rel_y - shape_start) / item_height) as usize;
                                let new_shape = match shape_idx {
                                    0 => NodeShape::RoundedRect,
                                    1 => NodeShape::DoubleRoundedRect,
                                    2 => NodeShape::Rectangle,
                                    3 => NodeShape::Round,
                                    _ => NodeShape::Diamond,
                                };
                                // Apply to all selected nodes if multi-selection
                                if self.selected_nodes.contains(&node_idx) && self.selected_nodes.len() > 1 {
                                    for &idx in &self.selected_nodes.clone() {
                                        self.nodes[idx].shape = new_shape;
                                    }
                                } else {
                                    self.nodes[node_idx].shape = new_shape;
                                }
                                self.context_menu_node = None;
                                self.view.redraw(cx);
                                return;
                            } else if rel_y >= border_start {
                                // Border item clicked
                                let border_idx = ((rel_y - border_start) / item_height) as usize;
                                let new_border = (border_idx + 1).min(4) as f32;
                                // Apply to all selected nodes if multi-selection
                                if self.selected_nodes.contains(&node_idx) && self.selected_nodes.len() > 1 {
                                    for &idx in &self.selected_nodes.clone() {
                                        self.nodes[idx].border_width = new_border;
                                    }
                                } else {
                                    self.nodes[node_idx].border_width = new_border;
                                }
                                self.context_menu_node = None;
                                self.view.redraw(cx);
                                return;
                            }

                            // Clicked in menu but not on item - keep menu open
                            return;
                        } else {
                            // Clicked outside menu - close it
                            self.context_menu_node = None;
                            self.view.redraw(cx);
                            // Continue to process click below
                        }
                    }
                }

                // Check if clicking on edge context menu
                if let Some(edge_idx) = self.context_menu_edge {
                    if edge_idx < self.edges.len() {
                        let menu_pos = self.context_menu_pos;
                        let menu_width = 120.0;
                        let item_height = 22.0;
                        let padding = 8.0;
                        let menu_height = 300.0;

                        let in_menu_x = fe.abs.x >= menu_pos.x && fe.abs.x <= menu_pos.x + menu_width;
                        let in_menu_y = fe.abs.y >= menu_pos.y && fe.abs.y <= menu_pos.y + menu_height;

                        if in_menu_x && in_menu_y {
                            let rel_y = fe.abs.y - menu_pos.y - padding;
                            let label_height = item_height * 0.8;

                            // Style section: label + 3 items
                            let style_start = label_height;
                            let style_end = style_start + item_height * 3.0;

                            // Width section: divider + label + 4 items
                            let width_label_start = style_end + item_height * 0.5;
                            let width_start = width_label_start + label_height;
                            let width_end = width_start + item_height * 4.0;

                            // Animation section: divider + label + 2 items
                            let anim_label_start = width_end + item_height * 0.5;
                            let anim_start = anim_label_start + label_height;

                            if rel_y >= style_start && rel_y < style_end {
                                // Style item clicked
                                let style_idx = ((rel_y - style_start) / item_height) as usize;
                                self.edges[edge_idx].style = style_idx.min(2) as f32;
                                self.context_menu_edge = None;
                                self.view.redraw(cx);
                                return;
                            } else if rel_y >= width_start && rel_y < width_end {
                                // Width item clicked
                                let width_idx = ((rel_y - width_start) / item_height) as usize;
                                self.edges[edge_idx].width = (width_idx + 1).min(4) as f32;
                                self.context_menu_edge = None;
                                self.view.redraw(cx);
                                return;
                            } else if rel_y >= anim_start {
                                // Animation item clicked
                                let anim_idx = ((rel_y - anim_start) / item_height) as usize;
                                self.edges[edge_idx].animated = anim_idx == 0;
                                self.context_menu_edge = None;
                                self.view.redraw(cx);
                                return;
                            }

                            // Clicked in menu but not on item
                            return;
                        } else {
                            // Clicked outside menu - close it
                            self.context_menu_edge = None;
                            self.view.redraw(cx);
                        }
                    }
                }

                // Check for shift+click for panning
                if fe.modifiers.shift {
                    self.drag_state = DragState::Panning { start: fe.abs };
                    cx.set_cursor(MouseCursor::Grab);
                    return;
                }

                // Check port clicks first (for edge creation)
                for (i, node) in self.nodes.iter().enumerate() {
                    // Check output port
                    if node.node_type.has_output() {
                        let port_rect = node.output_port_rect();
                        if port_rect.contains(local) {
                            self.drag_state = DragState::CreatingEdge {
                                from_node: i,
                                is_output: true,
                                cursor_pos: local
                            };
                            self.view.redraw(cx);
                            return;
                        }
                    }
                    // Check input port
                    if node.node_type.has_input() {
                        let port_rect = node.input_port_rect();
                        if port_rect.contains(local) {
                            self.drag_state = DragState::CreatingEdge {
                                from_node: i,
                                is_output: false,
                                cursor_pos: local
                            };
                            self.view.redraw(cx);
                            return;
                        }
                    }
                }

                // Check node clicks (reverse for z-order)
                for (i, node) in self.nodes.iter().enumerate().rev() {
                    if node.contains(local) {
                        // Check for Ctrl+click on node header for context menu
                        let header_bottom = node.y + 32.0; // Header is 32px tall
                        let is_header_click = local.y < header_bottom;
                        let is_context_click = fe.modifiers.control; // Ctrl+click

                        if is_context_click && is_header_click {
                            // Show context menu for this node
                            self.context_menu_node = Some(i);
                            self.context_menu_pos = fe.abs;
                            self.view.redraw(cx);
                            return;
                        }

                        // Hide context menu on regular click
                        self.context_menu_node = None;

                        // Multi-selection with shift+click
                        if fe.modifiers.shift {
                            // Toggle selection
                            if self.selected_nodes.contains(&i) {
                                self.selected_nodes.remove(&i);
                            } else {
                                self.selected_nodes.insert(i);
                            }
                        } else {
                            // Single selection - clear others
                            self.selected_nodes.clear();
                            self.selected_edges.clear();
                            self.selected_nodes.insert(i);
                        }

                        // Setup drag - handle multi-node drag if multiple selected
                        if self.selected_nodes.len() > 1 && self.selected_nodes.contains(&i) {
                            let offsets: Vec<(usize, DVec2)> = self.selected_nodes.iter()
                                .map(|&idx| {
                                    let n = &self.nodes[idx];
                                    (idx, DVec2 { x: local.x - n.x, y: local.y - n.y })
                                })
                                .collect();
                            self.drag_state = DragState::DraggingNodes { offsets };
                        } else {
                            self.drag_state = DragState::DraggingNode {
                                index: i,
                                offset: DVec2 { x: local.x - node.x, y: local.y - node.y },
                            };
                        }
                        cx.set_cursor(MouseCursor::Hand);
                        cx.widget_action(uid, &scope.path, FlowCanvasAction::SelectionChanged);
                        self.view.redraw(cx);
                        return;
                    }
                }

                // Check edge clicks for selection or context menu
                for (i, edge) in self.edges.iter().enumerate() {
                    if self.point_near_edge(local, edge) {
                        // Ctrl+click shows edge context menu
                        if fe.modifiers.control {
                            self.context_menu_edge = Some(i);
                            self.context_menu_node = None;
                            self.context_menu_pos = fe.abs;
                            self.view.redraw(cx);
                            return;
                        }
                        // Multi-selection with shift+click
                        if fe.modifiers.shift {
                            if self.selected_edges.contains(&i) {
                                self.selected_edges.remove(&i);
                            } else {
                                self.selected_edges.insert(i);
                            }
                        } else {
                            self.selected_edges.clear();
                            self.selected_nodes.clear();
                            self.selected_edges.insert(i);
                        }
                        self.context_menu_edge = None;
                        cx.widget_action(uid, &scope.path, FlowCanvasAction::SelectionChanged);
                        self.view.redraw(cx);
                        return;
                    }
                }

                // Clicked on empty space - start selection box or deselect
                if !fe.modifiers.shift {
                    // Start drag selection box
                    self.drag_state = DragState::SelectionBox { start: local, current: local };
                    self.selected_nodes.clear();
                    self.selected_edges.clear();
                }
                self.context_menu_node = None;
                self.context_menu_edge = None;
                cx.widget_action(uid, &scope.path, FlowCanvasAction::SelectionChanged);
                self.view.redraw(cx);
            }

            Hit::FingerMove(fe) => {
                let local = self.screen_to_canvas(fe.abs, area_rect);

                match &self.drag_state {
                    DragState::DraggingNode { index, offset } => {
                        let idx = *index;
                        let off = *offset;
                        self.nodes[idx].x = (local.x - off.x).max(0.0);
                        self.nodes[idx].y = (local.y - off.y).max(0.0);
                        self.view.redraw(cx);
                    }
                    DragState::DraggingNodes { offsets } => {
                        // Move all selected nodes together
                        let offsets_clone = offsets.clone();
                        for (idx, off) in offsets_clone {
                            self.nodes[idx].x = (local.x - off.x).max(0.0);
                            self.nodes[idx].y = (local.y - off.y).max(0.0);
                        }
                        self.view.redraw(cx);
                    }
                    DragState::Panning { start } => {
                        let delta = DVec2 {
                            x: fe.abs.x - start.x,
                            y: fe.abs.y - start.y,
                        };
                        self.pan_offset.x += delta.x;
                        self.pan_offset.y += delta.y;
                        self.drag_state = DragState::Panning { start: fe.abs };
                        self.view.redraw(cx);
                    }
                    DragState::CreatingEdge { from_node, is_output, .. } => {
                        self.drag_state = DragState::CreatingEdge {
                            from_node: *from_node,
                            is_output: *is_output,
                            cursor_pos: local,
                        };
                        self.view.redraw(cx);
                    }
                    DragState::SelectionBox { start, .. } => {
                        // Update selection box and select nodes within
                        let start_pos = *start;
                        self.drag_state = DragState::SelectionBox { start: start_pos, current: local };

                        // Find nodes within selection box
                        let min_x = start_pos.x.min(local.x);
                        let max_x = start_pos.x.max(local.x);
                        let min_y = start_pos.y.min(local.y);
                        let max_y = start_pos.y.max(local.y);

                        self.selected_nodes.clear();
                        for (i, node) in self.nodes.iter().enumerate() {
                            let node_center_x = node.x + node.width / 2.0;
                            let node_center_y = node.y + node.height / 2.0;
                            if node_center_x >= min_x && node_center_x <= max_x &&
                               node_center_y >= min_y && node_center_y <= max_y {
                                self.selected_nodes.insert(i);
                            }
                        }
                        self.view.redraw(cx);
                    }
                    DragState::None => {}
                }
            }

            Hit::FingerUp(fe) => {
                let local = self.screen_to_canvas(fe.abs, area_rect);

                // Handle edge creation completion
                if let DragState::CreatingEdge { from_node, is_output, .. } = &self.drag_state {
                    let from_idx = *from_node;
                    let from_is_output = *is_output;

                    // Find target port
                    for (i, node) in self.nodes.iter().enumerate() {
                        if i == from_idx { continue; }

                        // If dragging from output, look for input ports
                        if from_is_output && node.node_type.has_input() {
                            let port_rect = node.input_port_rect();
                            if port_rect.contains(local) {
                                // Check if edge already exists
                                let exists = self.edges.iter().any(|e|
                                    e.from_node == from_idx && e.to_node == i
                                );
                                if !exists {
                                    self.edges.push(EdgeConnection::new(from_idx, i));
                                    cx.widget_action(uid, &scope.path, FlowCanvasAction::EdgeCreated);
                                }
                                break;
                            }
                        }
                        // If dragging from input, look for output ports
                        if !from_is_output && node.node_type.has_output() {
                            let port_rect = node.output_port_rect();
                            if port_rect.contains(local) {
                                let exists = self.edges.iter().any(|e|
                                    e.from_node == i && e.to_node == from_idx
                                );
                                if !exists {
                                    self.edges.push(EdgeConnection::new(i, from_idx));
                                    cx.widget_action(uid, &scope.path, FlowCanvasAction::EdgeCreated);
                                }
                                break;
                            }
                        }
                    }
                }

                self.drag_state = DragState::None;
                cx.set_cursor(MouseCursor::Arrow);
                self.view.redraw(cx);
            }

            Hit::FingerScroll(se) => {
                // Zoom with scroll wheel
                let zoom_delta = if se.scroll.y > 0.0 { 1.1 } else { 0.9 };
                let local = self.screen_to_canvas(se.abs, area_rect);
                self.zoom = (self.zoom * zoom_delta).clamp(0.25, 4.0);

                // Zoom toward cursor position
                self.pan_offset.x = se.abs.x - area_rect.pos.x - (local.x * self.zoom);
                self.pan_offset.y = se.abs.y - area_rect.pos.y - (local.y * self.zoom);

                self.view.redraw(cx);
            }

            Hit::FingerHoverIn(_) => {
                cx.set_cursor(MouseCursor::Arrow);
            }

            _ => {}
        }

        self.view.handle_event(cx, event, scope);

        // Check for commands
        match event {
            Event::Actions(actions) => {
                for action in actions {
                    match action.cast() {
                        FlowCanvasCommand::AddNode => {
                            self.add_node(cx, NodeType::Custom);
                            cx.widget_action(uid, &scope.path, FlowCanvasAction::StatusUpdate {
                                nodes: self.nodes.len(),
                                edges: self.edges.len(),
                            });
                        }
                        FlowCanvasCommand::Delete => {
                            self.delete_selected(cx, uid, scope);
                            cx.widget_action(uid, &scope.path, FlowCanvasAction::StatusUpdate {
                                nodes: self.nodes.len(),
                                edges: self.edges.len(),
                            });
                        }
                        FlowCanvasCommand::FitView => {
                            self.fit_view(cx);
                        }
                        FlowCanvasCommand::Clear => {
                            self.clear(cx);
                            cx.widget_action(uid, &scope.path, FlowCanvasAction::StatusUpdate {
                                nodes: self.nodes.len(),
                                edges: self.edges.len(),
                            });
                        }
                        FlowCanvasCommand::SetLineStyle(style) => {
                            self.line_style = style;
                            self.view.redraw(cx);
                        }
                        FlowCanvasCommand::SetLineWidth(width) => {
                            self.line_width = width;
                            self.view.redraw(cx);
                        }
                        FlowCanvasCommand::LoadDataflow { nodes, edges } => {
                            self.nodes = nodes;
                            self.edges = edges;
                            self.selected_nodes.clear();
                            self.selected_edges.clear();
                            self.undo_stack.clear();
                            self.redo_stack.clear();
                            cx.action(FlowCanvasAction::StatusUpdate {
                                nodes: self.nodes.len(),
                                edges: self.edges.len(),
                            });
                            self.view.redraw(cx);
                        }
                        FlowCanvasCommand::None => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Begin drawing
        cx.begin_turtle(walk, Layout::flow_overlay());

        // Draw background
        let _ = self.view.draw_walk(cx, scope, walk.with_abs_pos(DVec2::default()));

        // Collect edge data first to avoid borrow issues - use per-edge properties
        let anim_phase = self.animation_phase;
        let edges_to_draw: Vec<_> = self.edges.iter().enumerate()
            .filter(|(_, edge)| edge.from_node < self.nodes.len() && edge.to_node < self.nodes.len())
            .map(|(i, edge)| {
                let from_node = &self.nodes[edge.from_node];
                let to_node = &self.nodes[edge.to_node];

                // Get port-specific positions if port IDs are specified
                let from_pos = if !edge.from_port.is_empty() {
                    if let Some(port_idx) = from_node.output_port_index(&edge.from_port) {
                        from_node.output_port_pos(port_idx)
                    } else {
                        from_node.output_pos()
                    }
                } else {
                    from_node.output_pos()
                };

                let to_pos = if !edge.to_port.is_empty() {
                    if let Some(port_idx) = to_node.input_port_index(&edge.to_port) {
                        to_node.input_port_pos(port_idx)
                    } else {
                        to_node.input_pos()
                    }
                } else {
                    to_node.input_pos()
                };

                let from = self.canvas_to_screen_pt(from_pos);
                let to = self.canvas_to_screen_pt(to_pos);
                let selected = self.selected_edges.contains(&i);
                // Use per-edge properties
                let edge_style = edge.style;
                let edge_width = edge.width as f64;
                let edge_animated = edge.animated;
                let marker = edge.marker_end;
                let label = edge.label.clone();
                (from, to, selected, edge_style, edge_width, edge_animated, marker, label)
            })
            .collect();

        // Draw edges using DrawColor for line segments
        for (from, to, selected, edge_style, edge_width, edge_animated, marker, label) in edges_to_draw {
            // Use negative value for anim_phase if animation is off (global or per-edge)
            let phase = if self.animate_edges && edge_animated { anim_phase } else { -1.0 };
            self.draw_bezier_edge(cx, from, to, selected, edge_width, edge_style, phase);

            // Draw edge marker (arrow) at endpoint
            if marker != EdgeMarker::None {
                self.draw_edge_marker(cx, from, to, selected, edge_width, marker);
            }

            // Draw edge label at midpoint (light theme)
            if !label.is_empty() {
                let mid = DVec2 { x: (from.x + to.x) / 2.0, y: (from.y + to.y) / 2.0 };
                self.draw_text.text_style.font_size = 9.0;
                self.draw_text.color = if selected { vec4(0.2, 0.4, 0.7, 1.0) } else { vec4(0.4, 0.4, 0.45, 1.0) };
                // Draw label background
                let label_width = label.len() as f64 * 6.0;
                self.draw_node_bg.color = vec4(1.0, 1.0, 1.0, 0.95); // White background
                self.draw_node_bg.draw_abs(cx, Rect {
                    pos: DVec2 { x: mid.x - label_width / 2.0 - 4.0, y: mid.y - 8.0 },
                    size: DVec2 { x: label_width + 8.0, y: 16.0 },
                });
                self.draw_text.draw_abs(cx, DVec2 { x: mid.x - label_width / 2.0, y: mid.y - 6.0 }, &label);
            }
        }

        // Draw edge being created
        let creating_edge_data = if let DragState::CreatingEdge { from_node, is_output, cursor_pos } = &self.drag_state {
            if *from_node < self.nodes.len() {
                let node = &self.nodes[*from_node];
                let port_pos = if *is_output { node.output_pos() } else { node.input_pos() };
                let from = self.canvas_to_screen_pt(port_pos);
                let to = self.canvas_to_screen_pt(*cursor_pos);
                Some((from, to))
            } else {
                None
            }
        } else {
            None
        };
        if let Some((from, to)) = creating_edge_data {
            // Use defaults for edge being created
            self.draw_bezier_edge(cx, from, to, true, 2.0, 0.0, anim_phase);
        }

        // Draw nodes - clone to avoid borrow issues, use each node's own shape and border
        let nodes_to_draw: Vec<_> = self.nodes.iter().cloned().enumerate()
            .map(|(i, node)| (node, self.selected_nodes.contains(&i)))
            .collect();
        for (node, is_selected) in nodes_to_draw {
            let shape = node.shape;
            let border_width = node.border_width as f64;
            self.draw_node(cx, &node, is_selected, shape, border_width);
        }

        // Draw selection box if dragging
        if let DragState::SelectionBox { start, current } = &self.drag_state {
            let start_screen = self.canvas_to_screen_pt(*start);
            let current_screen = self.canvas_to_screen_pt(*current);
            let min_x = start_screen.x.min(current_screen.x);
            let max_x = start_screen.x.max(current_screen.x);
            let min_y = start_screen.y.min(current_screen.y);
            let max_y = start_screen.y.max(current_screen.y);

            // Draw selection box fill
            self.draw_node_bg.color = vec4(0.3, 0.5, 0.8, 0.15);
            self.draw_node_bg.draw_abs(cx, Rect {
                pos: DVec2 { x: min_x, y: min_y },
                size: DVec2 { x: max_x - min_x, y: max_y - min_y },
            });

            // Draw selection box border
            self.draw_node_bg.color = vec4(0.4, 0.6, 1.0, 0.6);
            let border = 1.0;
            // Top
            self.draw_node_bg.draw_abs(cx, Rect { pos: DVec2 { x: min_x, y: min_y }, size: DVec2 { x: max_x - min_x, y: border } });
            // Bottom
            self.draw_node_bg.draw_abs(cx, Rect { pos: DVec2 { x: min_x, y: max_y - border }, size: DVec2 { x: max_x - min_x, y: border } });
            // Left
            self.draw_node_bg.draw_abs(cx, Rect { pos: DVec2 { x: min_x, y: min_y }, size: DVec2 { x: border, y: max_y - min_y } });
            // Right
            self.draw_node_bg.draw_abs(cx, Rect { pos: DVec2 { x: max_x - border, y: min_y }, size: DVec2 { x: border, y: max_y - min_y } });
        }

        // Draw node context menu if open
        if let Some(node_idx) = self.context_menu_node {
            if node_idx < self.nodes.len() {
                self.draw_context_menu(cx, self.context_menu_pos);
            }
        }

        // Draw edge context menu if open
        if let Some(edge_idx) = self.context_menu_edge {
            if edge_idx < self.edges.len() {
                self.draw_edge_context_menu(cx, self.context_menu_pos, edge_idx);
            }
        }

        cx.end_turtle();
        DrawStep::done()
    }
}

impl FlowCanvas {
    /// Draw a bezier curve edge using line segment quads with optional animated flow
    /// anim_phase < 0 means animation is disabled
    fn draw_bezier_edge(&mut self, cx: &mut Cx2d, from: DVec2, to: DVec2, selected: bool, thickness: f64, style: f32, anim_phase: f64) {
        // Get bezier curve points - more segments for smoother curves
        let points = BezierCurve::points_with_horizontal_tangents(from, to, 100);

        // Calculate total curve length for animation
        let mut total_len = 0.0;
        for i in 1..points.len() {
            let dx = points[i].x - points[i - 1].x;
            let dy = points[i].y - points[i - 1].y;
            total_len += (dx * dx + dy * dy).sqrt();
        }

        let animated = anim_phase >= 0.0;
        let phase = if animated { anim_phase } else { 0.0 };

        let half_thick = thickness * 0.5;

        let mut accumulated_len = 0.0;

        match style as i32 {
            0 => {
                // Solid line
                if animated {
                    // With animation: dim base line + bright flow particles
                    let edge_col = if selected { self.edge_selected_color } else { self.edge_color };
                    let base_color = vec4(edge_col.x * 0.6, edge_col.y * 0.6, edge_col.z * 0.6, 0.5);
                    let flow_color = vec4(
                        (edge_col.x * 1.2).min(1.0),
                        (edge_col.y * 1.2).min(1.0),
                        (edge_col.z * 1.2).min(1.0),
                        1.0
                    );

                    // Draw base line
                    self.draw_edge.color = base_color;
                    for i in 1..points.len() {
                        let prev_pt = points[i - 1];
                        let pt = points[i];
                        let dx = pt.x - prev_pt.x;
                        let dy = pt.y - prev_pt.y;
                        let seg_len = (dx * dx + dy * dy).sqrt();

                        if seg_len > 0.001 {
                            let steps = (seg_len / (thickness * 0.3)).max(1.0).ceil() as i32;
                            for s in 0..=steps {
                                let t = s as f64 / steps as f64;
                                let ix = prev_pt.x + dx * t;
                                let iy = prev_pt.y + dy * t;
                                self.draw_edge.draw_abs(cx, Rect {
                                    pos: DVec2 { x: ix - half_thick, y: iy - half_thick },
                                    size: DVec2 { x: thickness, y: thickness },
                                });
                            }
                        }
                    }

                    // Draw flow particles
                    let particle_spacing = 30.0;
                    let particle_size = thickness * 2.0;
                    let half_particle = particle_size * 0.5;
                    let anim_offset = phase * particle_spacing;

                    self.draw_edge.color = flow_color;
                    let mut next_particle_at = anim_offset;
                    accumulated_len = 0.0;

                    for i in 1..points.len() {
                        let prev_pt = points[i - 1];
                        let pt = points[i];
                        let dx = pt.x - prev_pt.x;
                        let dy = pt.y - prev_pt.y;
                        let seg_len = (dx * dx + dy * dy).sqrt();

                        while next_particle_at <= accumulated_len + seg_len && next_particle_at <= total_len {
                            let t = if seg_len > 0.001 {
                                (next_particle_at - accumulated_len) / seg_len
                            } else {
                                0.0
                            };
                            let t = t.clamp(0.0, 1.0);
                            let px = prev_pt.x + dx * t;
                            let py = prev_pt.y + dy * t;

                            self.draw_edge.draw_abs(cx, Rect {
                                pos: DVec2 { x: px - half_particle, y: py - half_particle },
                                size: DVec2 { x: particle_size, y: particle_size },
                            });

                            next_particle_at += particle_spacing;
                        }

                        accumulated_len += seg_len;
                    }
                } else {
                    // No animation: just solid bright line
                    self.draw_edge.color = if selected {
                        self.edge_selected_color
                    } else {
                        self.edge_color
                    };

                    for i in 1..points.len() {
                        let prev_pt = points[i - 1];
                        let pt = points[i];
                        let dx = pt.x - prev_pt.x;
                        let dy = pt.y - prev_pt.y;
                        let seg_len = (dx * dx + dy * dy).sqrt();

                        if seg_len > 0.001 {
                            let steps = (seg_len / (thickness * 0.3)).max(1.0).ceil() as i32;
                            for s in 0..=steps {
                                let t = s as f64 / steps as f64;
                                let ix = prev_pt.x + dx * t;
                                let iy = prev_pt.y + dy * t;
                                self.draw_edge.draw_abs(cx, Rect {
                                    pos: DVec2 { x: ix - half_thick, y: iy - half_thick },
                                    size: DVec2 { x: thickness, y: thickness },
                                });
                            }
                        }
                    }
                }
            }
            1 => {
                // Dashed line
                let dash_len = 12.0;
                let dash_gap = 8.0;
                let cycle = dash_len + dash_gap;
                let dash_offset = if animated { phase * cycle } else { 0.0 };

                self.draw_edge.color = if selected {
                    self.edge_selected_color
                } else {
                    self.edge_color
                };

                for i in 1..points.len() {
                    let prev_pt = points[i - 1];
                    let pt = points[i];
                    let dx = pt.x - prev_pt.x;
                    let dy = pt.y - prev_pt.y;
                    let seg_len = (dx * dx + dy * dy).sqrt();

                    let pos_in_cycle = (accumulated_len + dash_offset) % cycle;

                    if pos_in_cycle < dash_len && seg_len > 0.001 {
                        let steps = (seg_len / (thickness * 0.3)).max(1.0).ceil() as i32;
                        for s in 0..=steps {
                            let t = s as f64 / steps as f64;
                            let ix = prev_pt.x + dx * t;
                            let iy = prev_pt.y + dy * t;
                            self.draw_edge.draw_abs(cx, Rect {
                                pos: DVec2 { x: ix - half_thick, y: iy - half_thick },
                                size: DVec2 { x: thickness, y: thickness },
                            });
                        }
                    }

                    accumulated_len += seg_len;
                }
            }
            2 => {
                // Dotted line
                let dot_spacing = 12.0;
                let dot_size = thickness * 1.5;
                let half_dot = dot_size * 0.5;
                let dot_offset = if animated { phase * dot_spacing } else { 0.0 };

                self.draw_edge.color = if selected {
                    self.edge_selected_color
                } else {
                    self.edge_color
                };

                let mut next_dot_at = dot_offset;

                for i in 1..points.len() {
                    let prev_pt = points[i - 1];
                    let pt = points[i];
                    let dx = pt.x - prev_pt.x;
                    let dy = pt.y - prev_pt.y;
                    let seg_len = (dx * dx + dy * dy).sqrt();

                    while next_dot_at <= accumulated_len + seg_len && next_dot_at <= total_len {
                        let t = if seg_len > 0.001 {
                            (next_dot_at - accumulated_len) / seg_len
                        } else {
                            0.0
                        };
                        let t = t.clamp(0.0, 1.0);
                        let dot_x = prev_pt.x + dx * t;
                        let dot_y = prev_pt.y + dy * t;

                        self.draw_edge.draw_abs(cx, Rect {
                            pos: DVec2 { x: dot_x - half_dot, y: dot_y - half_dot },
                            size: DVec2 { x: dot_size, y: dot_size },
                        });

                        next_dot_at += dot_spacing;
                    }

                    accumulated_len += seg_len;
                }
            }
            _ => {}
        }
    }

    fn initialize(&mut self, cx: &mut Cx) {
        if self.initialized { return; }

        self.zoom = self.default_zoom;
        self.pan_offset = DVec2::default();
        self.drag_state = DragState::None;
        self.next_node_id = 4;
        self.animation_phase = 0.0;
        self.context_menu_node = None;
        self.context_menu_edge = None;
        self.context_menu_pos = DVec2::default();
        self.selected_nodes = HashSet::new();
        self.selected_edges = HashSet::new();
        self.undo_stack = Vec::new();
        self.redo_stack = Vec::new();

        // Start animation timer (~50fps for smooth animation)
        self.animation_timer = cx.start_interval(0.02);

        // Create sample nodes by default
        self.create_sample_nodes();

        self.initialized = true;
    }

    fn create_sample_nodes(&mut self) {
        // Create initial nodes
        self.nodes = vec![
            FlowNode::new(50.0, 100.0, NodeType::Camera),
            FlowNode::new(300.0, 50.0, NodeType::Detector),
            FlowNode::new(300.0, 200.0, NodeType::Tracker),
            FlowNode::new(550.0, 120.0, NodeType::Visualizer),
        ];

        // Create initial edges
        self.edges = vec![
            EdgeConnection::new(0, 1),
            EdgeConnection::new(1, 2),
            EdgeConnection::new(2, 3),
        ];
    }

    fn screen_to_canvas(&self, screen_pos: DVec2, area_rect: Rect) -> DVec2 {
        DVec2 {
            x: (screen_pos.x - area_rect.pos.x - self.pan_offset.x) / self.zoom,
            y: (screen_pos.y - area_rect.pos.y - self.pan_offset.y) / self.zoom,
        }
    }

    fn canvas_to_screen_pt(&self, canvas_pos: DVec2) -> DVec2 {
        DVec2 {
            x: canvas_pos.x * self.zoom + self.pan_offset.x,
            y: canvas_pos.y * self.zoom + self.pan_offset.y,
        }
    }

    fn point_near_edge(&self, point: DVec2, edge: &EdgeConnection) -> bool {
        if edge.from_node >= self.nodes.len() || edge.to_node >= self.nodes.len() {
            return false;
        }
        let from = self.nodes[edge.from_node].output_pos();
        let to = self.nodes[edge.to_node].input_pos();

        // Simple distance check to bezier (approximate with line segments)
        let dx = (to.x - from.x) * 0.5;
        let c0 = DVec2 { x: from.x + dx, y: from.y };
        let c1 = DVec2 { x: to.x - dx, y: to.y };

        for i in 0..32 {
            let t = i as f64 / 31.0;
            let t2 = t * t;
            let t3 = t2 * t;
            let mt = 1.0 - t;
            let mt2 = mt * mt;
            let mt3 = mt2 * mt;
            let bp = DVec2 {
                x: mt3 * from.x + 3.0 * mt2 * t * c0.x + 3.0 * mt * t2 * c1.x + t3 * to.x,
                y: mt3 * from.y + 3.0 * mt2 * t * c0.y + 3.0 * mt * t2 * c1.y + t3 * to.y,
            };
            let dist = ((point.x - bp.x).powi(2) + (point.y - bp.y).powi(2)).sqrt();
            if dist < 8.0 {
                return true;
            }
        }
        false
    }

    fn draw_node(&mut self, cx: &mut Cx2d, node: &FlowNode, selected: bool, shape: NodeShape, border_width: f64) {
        let pos = self.canvas_to_screen_pt(DVec2 { x: node.x, y: node.y });
        let size = DVec2 { x: node.width * self.zoom, y: node.height * self.zoom };
        let center = DVec2 { x: pos.x + size.x * 0.5, y: pos.y + size.y * 0.5 };

        // Node background color - light theme (white nodes with colored headers)
        let (bg_color, header_color) = if node.category == NodeCategory::Default {
            if selected {
                // Light blue tint when selected
                (vec4(0.93, 0.96, 1.0, 1.0), vec4(0.90, 0.93, 0.98, 1.0))
            } else {
                // White background, light gray header
                (vec4(1.0, 1.0, 1.0, 1.0), vec4(0.97, 0.97, 0.97, 1.0))
            }
        } else {
            // For category nodes, use lighter pastel versions
            let base_color = node.category.color();
            let head_color = node.category.header_color();
            if selected {
                // Slightly darker tint when selected
                (vec4(base_color.x * 0.95 + 0.05, base_color.y * 0.95 + 0.05, base_color.z * 0.95 + 0.05, 1.0),
                 vec4(head_color.x * 0.95, head_color.y * 0.95, head_color.z * 0.95, 1.0))
            } else {
                // Light pastel body with category header
                (vec4(1.0, 1.0, 1.0, 1.0), head_color)
            }
        };
        let border_color = self.selection_color;

        match shape {
            NodeShape::RoundedRect => {
                // Draw rounded rectangle node with header (rounded top, straight bottom)
                let corner_r = (8.0 * self.zoom) as f32;
                let header_h = 32.0 * self.zoom;
                let bw = if selected { border_width.max(2.0) as f32 } else { border_width as f32 };
                let bc = if selected { border_color } else { vec4(0.88, 0.88, 0.88, 1.0) }; // #e0e0e0
                let inset = bw as f64;

                // 1. Draw body (below header)
                self.draw_node_bg.color = bg_color;
                self.draw_node_bg.draw_abs(cx, Rect {
                    pos: DVec2 { x: pos.x + inset, y: pos.y + header_h },
                    size: DVec2 { x: size.x - inset * 2.0, y: size.y - header_h - inset }
                });

                // 2. Draw header with rounded TOP corners only (flat bottom)
                self.draw_rounded_top_rect.color = header_color;
                self.draw_rounded_top_rect.radius = corner_r;
                self.draw_rounded_top_rect.draw_abs(cx, Rect {
                    pos: DVec2 { x: pos.x + inset, y: pos.y + inset },
                    size: DVec2 { x: size.x - inset * 2.0, y: header_h - inset }
                });

                // 3. Draw outer border
                self.draw_rounded_rect.color = vec4(0.0, 0.0, 0.0, 0.0);
                self.draw_rounded_rect.radius = corner_r;
                self.draw_rounded_rect.border_width = bw;
                self.draw_rounded_rect.border_color = bc;
                self.draw_rounded_rect.draw_abs(cx, Rect { pos, size });
            }
            NodeShape::DoubleRoundedRect => {
                // Draw fully rounded rectangle node (rounded top header + rounded bottom body)
                let corner_r = (8.0 * self.zoom) as f32;
                let header_h = 32.0 * self.zoom;
                let bw = if selected { border_width.max(2.0) as f32 } else { border_width as f32 };
                let bc = if selected { border_color } else { vec4(0.88, 0.88, 0.88, 1.0) }; // #e0e0e0
                let inset = bw as f64;

                // 1. Draw body with rounded BOTTOM corners only (straight top)
                self.draw_rounded_bottom_rect.color = bg_color;
                self.draw_rounded_bottom_rect.radius = corner_r;
                self.draw_rounded_bottom_rect.draw_abs(cx, Rect {
                    pos: DVec2 { x: pos.x + inset, y: pos.y + header_h },
                    size: DVec2 { x: size.x - inset * 2.0, y: size.y - header_h - inset }
                });

                // 2. Draw header with rounded TOP corners only (straight bottom)
                self.draw_rounded_top_rect.color = header_color;
                self.draw_rounded_top_rect.radius = corner_r;
                self.draw_rounded_top_rect.draw_abs(cx, Rect {
                    pos: DVec2 { x: pos.x + inset, y: pos.y + inset },
                    size: DVec2 { x: size.x - inset * 2.0, y: header_h - inset }
                });

                // 3. Draw outer border
                self.draw_rounded_rect.color = vec4(0.0, 0.0, 0.0, 0.0);
                self.draw_rounded_rect.radius = corner_r;
                self.draw_rounded_rect.border_width = bw;
                self.draw_rounded_rect.border_color = bc;
                self.draw_rounded_rect.draw_abs(cx, Rect { pos, size });
            }
            NodeShape::Rectangle => {
                // Draw plain rectangle
                self.draw_node_bg.color = bg_color;
                self.draw_node_bg.draw_abs(cx, Rect { pos, size });

                // Draw header
                self.draw_node_bg.color = header_color;
                self.draw_node_bg.draw_abs(cx, Rect {
                    pos,
                    size: DVec2 { x: size.x, y: 32.0 * self.zoom }
                });

                // Border
                if selected || border_width > 0.0 {
                    let bw = if selected { border_width.max(2.0) } else { border_width };
                    self.draw_node_bg.color = if selected { border_color } else { vec4(0.4, 0.4, 0.5, 0.6) };
                    // Top
                    self.draw_node_bg.draw_abs(cx, Rect { pos, size: DVec2 { x: size.x, y: bw } });
                    // Bottom
                    self.draw_node_bg.draw_abs(cx, Rect {
                        pos: DVec2 { x: pos.x, y: pos.y + size.y - bw },
                        size: DVec2 { x: size.x, y: bw }
                    });
                    // Left
                    self.draw_node_bg.draw_abs(cx, Rect { pos, size: DVec2 { x: bw, y: size.y } });
                    // Right
                    self.draw_node_bg.draw_abs(cx, Rect {
                        pos: DVec2 { x: pos.x + size.x - bw, y: pos.y },
                        size: DVec2 { x: bw, y: size.y }
                    });
                }
            }
            NodeShape::Round => {
                // Draw as circle/ellipse using overlapping squares
                let radius = size.x.min(size.y) * 0.5;
                self.draw_node_bg.color = bg_color;

                // Draw filled circle
                let segments = 36;
                for i in 0..segments {
                    let angle = (i as f64 / segments as f64) * std::f64::consts::PI * 2.0;
                    let next_angle = ((i + 1) as f64 / segments as f64) * std::f64::consts::PI * 2.0;

                    // Draw pie slice as triangular approximation
                    for r in 0..((radius / 3.0) as i32).max(1) {
                        let r_ratio = r as f64 / (radius / 3.0);
                        let inner_r = radius * r_ratio;
                        let outer_r = radius * (r_ratio + 1.0 / (radius / 3.0)).min(1.0);

                        for a in 0..3 {
                            let t = a as f64 / 3.0;
                            let curr_angle = angle + (next_angle - angle) * t;
                            let px = center.x + curr_angle.cos() * (inner_r + outer_r) * 0.5;
                            let py = center.y + curr_angle.sin() * (inner_r + outer_r) * 0.5;
                            self.draw_node_bg.draw_abs(cx, Rect {
                                pos: DVec2 { x: px - 2.0, y: py - 2.0 },
                                size: DVec2 { x: 4.0, y: 4.0 },
                            });
                        }
                    }
                }

                // Draw header portion (top half darker)
                self.draw_node_bg.color = header_color;
                for i in 0..(segments / 2) {
                    let angle = (i as f64 / segments as f64) * std::f64::consts::PI * 2.0 - std::f64::consts::PI * 0.5;
                    let next_angle = ((i + 1) as f64 / segments as f64) * std::f64::consts::PI * 2.0 - std::f64::consts::PI * 0.5;

                    for r in 0..((radius / 3.0) as i32).max(1) {
                        let r_ratio = r as f64 / (radius / 3.0);
                        let inner_r = radius * r_ratio;
                        let outer_r = radius * (r_ratio + 1.0 / (radius / 3.0)).min(1.0);

                        for a in 0..3 {
                            let t = a as f64 / 3.0;
                            let curr_angle = angle + (next_angle - angle) * t;
                            let px = center.x + curr_angle.cos() * (inner_r + outer_r) * 0.5;
                            let py = center.y + curr_angle.sin() * (inner_r + outer_r) * 0.5;
                            if py < center.y {
                                self.draw_node_bg.draw_abs(cx, Rect {
                                    pos: DVec2 { x: px - 2.0, y: py - 2.0 },
                                    size: DVec2 { x: 4.0, y: 4.0 },
                                });
                            }
                        }
                    }
                }

                // Border ring
                if selected || border_width > 0.0 {
                    let bw = if selected { border_width.max(2.0) } else { border_width };
                    self.draw_node_bg.color = if selected { border_color } else { vec4(0.4, 0.4, 0.5, 0.6) };
                    for i in 0..72 {
                        let angle = (i as f64 / 72.0) * std::f64::consts::PI * 2.0;
                        let px = center.x + angle.cos() * radius;
                        let py = center.y + angle.sin() * radius;
                        self.draw_node_bg.draw_abs(cx, Rect {
                            pos: DVec2 { x: px - bw * 0.5, y: py - bw * 0.5 },
                            size: DVec2 { x: bw, y: bw },
                        });
                    }
                }
            }
            NodeShape::Diamond => {
                // Draw diamond shape
                let half_w = size.x * 0.5;
                let half_h = size.y * 0.5;

                // Fill diamond with small squares
                self.draw_node_bg.color = bg_color;
                let step = 4.0;
                let mut y = pos.y;
                while y < pos.y + size.y {
                    let dy = y - center.y;
                    let ratio = 1.0 - (dy.abs() / half_h);
                    let width_at_y = half_w * ratio;

                    let mut x = center.x - width_at_y;
                    while x < center.x + width_at_y {
                        self.draw_node_bg.draw_abs(cx, Rect {
                            pos: DVec2 { x, y },
                            size: DVec2 { x: step, y: step },
                        });
                        x += step;
                    }
                    y += step;
                }

                // Header (top portion)
                self.draw_node_bg.color = header_color;
                y = pos.y;
                while y < center.y - half_h * 0.3 {
                    let dy = y - center.y;
                    let ratio = 1.0 - (dy.abs() / half_h);
                    let width_at_y = half_w * ratio;

                    let mut x = center.x - width_at_y;
                    while x < center.x + width_at_y {
                        self.draw_node_bg.draw_abs(cx, Rect {
                            pos: DVec2 { x, y },
                            size: DVec2 { x: step, y: step },
                        });
                        x += step;
                    }
                    y += step;
                }

                // Diamond border
                if selected || border_width > 0.0 {
                    let bw = if selected { border_width.max(2.0) } else { border_width };
                    self.draw_node_bg.color = if selected { border_color } else { vec4(0.4, 0.4, 0.5, 0.6) };

                    // Draw 4 edges of diamond
                    let top = DVec2 { x: center.x, y: pos.y };
                    let right = DVec2 { x: pos.x + size.x, y: center.y };
                    let bottom = DVec2 { x: center.x, y: pos.y + size.y };
                    let left = DVec2 { x: pos.x, y: center.y };

                    // Draw each edge
                    for (start, end) in [(top, right), (right, bottom), (bottom, left), (left, top)] {
                        let dx = end.x - start.x;
                        let dy = end.y - start.y;
                        let len = (dx * dx + dy * dy).sqrt();
                        let steps = (len / 3.0) as i32;
                        for s in 0..=steps {
                            let t = s as f64 / steps as f64;
                            let px = start.x + dx * t;
                            let py = start.y + dy * t;
                            self.draw_node_bg.draw_abs(cx, Rect {
                                pos: DVec2 { x: px - bw * 0.5, y: py - bw * 0.5 },
                                size: DVec2 { x: bw, y: bw },
                            });
                        }
                    }
                }
            }
        }

        // Title text (centered for round/diamond)
        self.draw_text.text_style.font_size = (11.0 * self.zoom) as f32;
        self.draw_text.color = vec4(0.88, 0.88, 0.88, 1.0);

        // Truncate title to fit within node (max ~20 chars for 180px node)
        let mut display_title = node.title.clone();
        let max_chars = 20;
        if display_title.chars().count() > max_chars {
            display_title = display_title.chars().take(max_chars - 2).collect::<String>() + "..";
        }

        match shape {
            NodeShape::Round | NodeShape::Diamond => {
                let laidout = self.draw_text.layout(cx, 0.0, 0.0, None, Align::default(), &display_title);
                let text_w = laidout.size_in_lpxs.width as f64;
                self.draw_text.draw_abs(cx, DVec2 { x: center.x - text_w / 2.0, y: center.y - 8.0 }, &display_title);
            }
            _ => {
                let laidout = self.draw_text.layout(cx, 0.0, 0.0, None, Align::default(), &display_title);
                let text_w = laidout.size_in_lpxs.width as f64;
                let text_h = laidout.size_in_lpxs.height as f64;
                let header_h = 32.0 * self.zoom;
                self.draw_text.draw_abs(cx, DVec2 { x: center.x - text_w / 2.0, y: pos.y + (header_h - text_h) / 2.0 }, &display_title);
            }
        }

        // Draw ports
        let port_radius = 5.0 * self.zoom;
        let port_height = 20.0 * self.zoom;
        let header_h = 32.0 * self.zoom;

        // For Round/Diamond shapes, use legacy single port
        if matches!(shape, NodeShape::Round | NodeShape::Diamond) {
            let (input_screen_pos, output_screen_pos) = match shape {
                NodeShape::Round => {
                    let radius = size.x.min(size.y) * 0.5;
                    (
                        DVec2 { x: center.x - radius, y: center.y },
                        DVec2 { x: center.x + radius, y: center.y }
                    )
                }
                _ => {
                    (
                        DVec2 { x: pos.x, y: center.y },
                        DVec2 { x: pos.x + size.x, y: center.y }
                    )
                }
            };

            if node.node_type.has_input() {
                self.draw_node_bg.color = vec4(0.23, 0.51, 0.96, 1.0);
                self.draw_node_bg.draw_abs(cx, Rect {
                    pos: DVec2 { x: input_screen_pos.x - port_radius, y: input_screen_pos.y - port_radius },
                    size: DVec2 { x: port_radius * 2.0, y: port_radius * 2.0 },
                });
            }

            if node.node_type.has_output() {
                self.draw_node_bg.color = vec4(0.13, 0.77, 0.37, 1.0);
                self.draw_node_bg.draw_abs(cx, Rect {
                    pos: DVec2 { x: output_screen_pos.x - port_radius, y: output_screen_pos.y - port_radius },
                    size: DVec2 { x: port_radius * 2.0, y: port_radius * 2.0 },
                });
            }
        } else {
            // Draw multiple input ports with labels
            self.draw_text.text_style.font_size = (9.0 * self.zoom) as f32;
            self.draw_text.color = vec4(0.7, 0.7, 0.8, 1.0);

            for (i, port) in node.input_ports.iter().enumerate() {
                let port_y = pos.y + header_h + (i as f64 * port_height) + port_height / 2.0;
                let port_x = pos.x;

                // Draw port circle (blue for input)
                self.draw_node_bg.color = vec4(0.23, 0.51, 0.96, 1.0);
                self.draw_node_bg.draw_abs(cx, Rect {
                    pos: DVec2 { x: port_x - port_radius, y: port_y - port_radius },
                    size: DVec2 { x: port_radius * 2.0, y: port_radius * 2.0 },
                });

                // Draw port label
                let label = if port.label.len() > 12 {
                    format!("{}...", &port.label[..10])
                } else {
                    port.label.clone()
                };
                self.draw_text.draw_abs(cx, DVec2 { x: port_x + port_radius + 4.0, y: port_y - 5.0 }, &label);
            }

            // Draw multiple output ports with labels
            for (i, port) in node.output_ports.iter().enumerate() {
                let port_y = pos.y + header_h + (i as f64 * port_height) + port_height / 2.0;
                let port_x = pos.x + size.x;

                // Draw port circle (green for output)
                self.draw_node_bg.color = vec4(0.13, 0.77, 0.37, 1.0);
                self.draw_node_bg.draw_abs(cx, Rect {
                    pos: DVec2 { x: port_x - port_radius, y: port_y - port_radius },
                    size: DVec2 { x: port_radius * 2.0, y: port_radius * 2.0 },
                });

                // Draw port label (right-aligned)
                let label = if port.label.len() > 12 {
                    format!("{}...", &port.label[..10])
                } else {
                    port.label.clone()
                };
                let laidout = self.draw_text.layout(cx, 0.0, 0.0, None, Align::default(), &label);
                let text_w = laidout.size_in_lpxs.width as f64;
                self.draw_text.draw_abs(cx, DVec2 { x: port_x - port_radius - text_w - 4.0, y: port_y - 5.0 }, &label);
            }
        }
    }

    /// Draw context menu at screen position
    fn draw_context_menu(&mut self, cx: &mut Cx2d, pos: DVec2) {
        let menu_width = 120.0;
        let item_height = 22.0;
        let padding = 4.0;

        // Check if this is a multi-selection context menu
        let node_idx = self.context_menu_node.unwrap_or(0);
        let is_multi = self.selected_nodes.contains(&node_idx) && self.selected_nodes.len() > 1;
        let multi_count = if is_multi { self.selected_nodes.len() } else { 1 };

        // Add extra space for multi-selection header
        let num_items = if is_multi { 12 } else { 11 }; // +1 for multi header
        let menu_height = item_height * num_items as f64 + padding * 2.0;

        // Menu background (light theme)
        self.draw_node_bg.color = vec4(1.0, 1.0, 1.0, 0.98);
        self.draw_node_bg.draw_abs(cx, Rect {
            pos,
            size: DVec2 { x: menu_width, y: menu_height },
        });

        // Border (light theme)
        self.draw_node_bg.color = vec4(0.88, 0.88, 0.88, 1.0); // #e0e0e0
        // Top
        self.draw_node_bg.draw_abs(cx, Rect { pos, size: DVec2 { x: menu_width, y: 1.0 } });
        // Bottom
        self.draw_node_bg.draw_abs(cx, Rect {
            pos: DVec2 { x: pos.x, y: pos.y + menu_height - 1.0 },
            size: DVec2 { x: menu_width, y: 1.0 }
        });
        // Left
        self.draw_node_bg.draw_abs(cx, Rect { pos, size: DVec2 { x: 1.0, y: menu_height } });
        // Right
        self.draw_node_bg.draw_abs(cx, Rect {
            pos: DVec2 { x: pos.x + menu_width - 1.0, y: pos.y },
            size: DVec2 { x: 1.0, y: menu_height }
        });

        let mut y = pos.y + padding;

        // Multi-selection header (light theme)
        if is_multi {
            self.draw_text.text_style.font_size = 9.0;
            self.draw_text.color = self.selection_color;
            self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 8.0, y }, &format!("Apply to {} nodes", multi_count));
            y += item_height;
        }

        // Section label - Shape (light theme)
        self.draw_text.text_style.font_size = 9.0;
        self.draw_text.color = vec4(0.6, 0.6, 0.6, 1.0); // #999999
        self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 8.0, y }, "Shape");
        y += item_height * 0.8;

        // Shape items (light theme)
        let shape_items = ["Rounded Rect", "Double Rounded", "Rectangle", "Round", "Diamond"];
        self.draw_text.text_style.font_size = 10.0;
        self.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0); // #333333
        for label in shape_items {
            self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 12.0, y }, label);
            y += item_height;
        }

        // Divider (light theme)
        self.draw_node_bg.color = vec4(0.88, 0.88, 0.88, 1.0); // #e0e0e0
        self.draw_node_bg.draw_abs(cx, Rect {
            pos: DVec2 { x: pos.x + 8.0, y: y + 2.0 },
            size: DVec2 { x: menu_width - 16.0, y: 1.0 }
        });
        y += item_height * 0.5;

        // Section label - Border (light theme)
        self.draw_text.text_style.font_size = 9.0;
        self.draw_text.color = vec4(0.6, 0.6, 0.6, 1.0); // #999999
        self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 8.0, y }, "Border");
        y += item_height * 0.8;

        // Border items (light theme)
        let border_items = ["1px", "2px", "3px", "4px"];
        self.draw_text.text_style.font_size = 10.0;
        self.draw_text.color = vec4(0.2, 0.2, 0.2, 1.0); // #333333
        for label in border_items {
            self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 12.0, y }, label);
            y += item_height;
        }
    }

    pub fn add_node(&mut self, cx: &mut Cx, node_type: NodeType) {
        self.save_undo_state();

        // Position new node at center of view
        let x = (-self.pan_offset.x / self.zoom) + 200.0;
        let y = (-self.pan_offset.y / self.zoom) + 150.0;

        let mut node = FlowNode::new(x, y, node_type);
        if node_type == NodeType::Custom {
            node.title = format!("Node {}", self.next_node_id);
        }
        self.next_node_id += 1;

        self.nodes.push(node);
        self.selected_nodes.clear();
        self.selected_nodes.insert(self.nodes.len() - 1);
        self.view.redraw(cx);
    }

    fn delete_selected(&mut self, cx: &mut Cx, uid: WidgetUid, scope: &Scope) {
        if self.selected_edges.is_empty() && self.selected_nodes.is_empty() {
            return;
        }

        self.save_undo_state();

        // Delete selected edges first (simpler - no index updates needed for other edges)
        if !self.selected_edges.is_empty() {
            let mut edges_to_remove: Vec<usize> = self.selected_edges.iter().cloned().collect();
            edges_to_remove.sort_by(|a, b| b.cmp(a)); // Sort descending to remove from end first
            for idx in edges_to_remove {
                if idx < self.edges.len() {
                    self.edges.remove(idx);
                }
            }
            self.selected_edges.clear();
            cx.widget_action(uid, &scope.path, FlowCanvasAction::EdgeDeleted);
        }

        // Delete selected nodes
        if !self.selected_nodes.is_empty() {
            let mut nodes_to_remove: Vec<usize> = self.selected_nodes.iter().cloned().collect();
            nodes_to_remove.sort_by(|a, b| b.cmp(a)); // Sort descending

            for node_idx in nodes_to_remove {
                if node_idx < self.nodes.len() {
                    // Remove edges connected to this node
                    self.edges.retain(|e| e.from_node != node_idx && e.to_node != node_idx);
                    // Update edge indices for remaining edges
                    for edge in &mut self.edges {
                        if edge.from_node > node_idx { edge.from_node -= 1; }
                        if edge.to_node > node_idx { edge.to_node -= 1; }
                    }
                    self.nodes.remove(node_idx);
                }
            }
            self.selected_nodes.clear();
            cx.widget_action(uid, &scope.path, FlowCanvasAction::NodeDeleted);
        }

        self.view.redraw(cx);
    }

    fn save_undo_state(&mut self) {
        self.undo_stack.push(HistoryEntry {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        });
        // Limit undo stack size
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        // Clear redo stack on new action
        self.redo_stack.clear();
    }

    fn undo(&mut self, cx: &mut Cx) {
        if let Some(state) = self.undo_stack.pop() {
            // Save current state to redo stack
            self.redo_stack.push(HistoryEntry {
                nodes: self.nodes.clone(),
                edges: self.edges.clone(),
            });
            // Restore previous state
            self.nodes = state.nodes;
            self.edges = state.edges;
            self.selected_nodes.clear();
            self.selected_edges.clear();
            self.view.redraw(cx);
        }
    }

    fn redo(&mut self, cx: &mut Cx) {
        if let Some(state) = self.redo_stack.pop() {
            // Save current state to undo stack
            self.undo_stack.push(HistoryEntry {
                nodes: self.nodes.clone(),
                edges: self.edges.clone(),
            });
            // Restore redo state
            self.nodes = state.nodes;
            self.edges = state.edges;
            self.selected_nodes.clear();
            self.selected_edges.clear();
            self.view.redraw(cx);
        }
    }

    fn select_all(&mut self, cx: &mut Cx) {
        self.selected_nodes.clear();
        self.selected_edges.clear();
        for i in 0..self.nodes.len() {
            self.selected_nodes.insert(i);
        }
        self.view.redraw(cx);
    }

    pub fn fit_view(&mut self, cx: &mut Cx) {
        if self.nodes.is_empty() { return; }

        // Find bounding box of all nodes
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;

        for node in &self.nodes {
            min_x = min_x.min(node.x);
            min_y = min_y.min(node.y);
        }

        // Add padding
        let padding = 50.0;
        min_x -= padding;
        min_y -= padding;

        // Reset to fit
        self.zoom = 1.0;
        self.pan_offset = DVec2 { x: -min_x, y: -min_y };
        self.view.redraw(cx);
    }

    pub fn clear(&mut self, cx: &mut Cx) {
        self.save_undo_state();
        self.nodes.clear();
        self.edges.clear();
        self.selected_nodes.clear();
        self.selected_edges.clear();
        self.view.redraw(cx);
    }

    /// Draw arrow marker at edge endpoint
    fn draw_edge_marker(&mut self, cx: &mut Cx2d, from: DVec2, to: DVec2, selected: bool, thickness: f64, marker: EdgeMarker) {
        // Calculate direction at endpoint using bezier tangent
        let dx = to.x - from.x;
        let control_offset = dx.abs() * 0.5;
        // Approximate tangent at end point (towards to from control point)
        let c1 = DVec2 { x: to.x - control_offset, y: to.y };
        let dir_x = to.x - c1.x;
        let dir_y = to.y - c1.y;
        let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
        if len < 0.001 { return; }
        let nx = dir_x / len;
        let ny = dir_y / len;

        let arrow_size = thickness * 4.0;

        match marker {
            EdgeMarker::Arrow | EdgeMarker::ArrowFilled => {
                // Arrow head points
                let tip = to;
                let back = arrow_size * 1.5;
                let width = arrow_size * 0.8;

                // Perpendicular direction
                let px = -ny;
                let py = nx;

                let left = DVec2 { x: tip.x - nx * back + px * width, y: tip.y - ny * back + py * width };
                let right = DVec2 { x: tip.x - nx * back - px * width, y: tip.y - ny * back - py * width };

                self.draw_edge.color = if selected {
                    self.edge_selected_color
                } else {
                    self.edge_color
                };

                if marker == EdgeMarker::ArrowFilled {
                    // Fill triangle
                    self.fill_triangle(cx, tip, left, right);
                } else {
                    // Draw outline
                    self.draw_line(cx, tip, left, thickness);
                    self.draw_line(cx, tip, right, thickness);
                }
            }
            EdgeMarker::Circle => {
                let radius = arrow_size;
                self.draw_edge.color = if selected {
                    self.edge_selected_color
                } else {
                    self.edge_color
                };
                // Draw circle at endpoint
                for i in 0..16 {
                    let angle = (i as f64 / 16.0) * std::f64::consts::PI * 2.0;
                    let px = to.x + angle.cos() * radius;
                    let py = to.y + angle.sin() * radius;
                    self.draw_edge.draw_abs(cx, Rect {
                        pos: DVec2 { x: px - thickness * 0.5, y: py - thickness * 0.5 },
                        size: DVec2 { x: thickness, y: thickness },
                    });
                }
            }
            EdgeMarker::None => {}
        }
    }

    fn fill_triangle(&mut self, cx: &mut Cx2d, p0: DVec2, p1: DVec2, p2: DVec2) {
        // Simple scanline fill for triangle
        let min_y = p0.y.min(p1.y).min(p2.y);
        let max_y = p0.y.max(p1.y).max(p2.y);
        let min_x = p0.x.min(p1.x).min(p2.x);
        let max_x = p0.x.max(p1.x).max(p2.x);

        let step = 2.0;
        let mut y = min_y;
        while y <= max_y {
            let mut x = min_x;
            while x <= max_x {
                if self.point_in_triangle(DVec2 { x, y }, p0, p1, p2) {
                    self.draw_edge.draw_abs(cx, Rect {
                        pos: DVec2 { x, y },
                        size: DVec2 { x: step, y: step },
                    });
                }
                x += step;
            }
            y += step;
        }
    }

    fn point_in_triangle(&self, p: DVec2, p0: DVec2, p1: DVec2, p2: DVec2) -> bool {
        let area = 0.5 * (-p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y);
        let s = 1.0 / (2.0 * area) * (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * p.x + (p0.x - p2.x) * p.y);
        let t = 1.0 / (2.0 * area) * (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * p.x + (p1.x - p0.x) * p.y);
        s >= 0.0 && t >= 0.0 && (s + t) <= 1.0
    }

    fn draw_line(&mut self, cx: &mut Cx2d, from: DVec2, to: DVec2, thickness: f64) {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let len = (dx * dx + dy * dy).sqrt();
        let steps = (len / (thickness * 0.5)).max(1.0).ceil() as i32;
        let half = thickness * 0.5;

        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let x = from.x + dx * t;
            let y = from.y + dy * t;
            self.draw_edge.draw_abs(cx, Rect {
                pos: DVec2 { x: x - half, y: y - half },
                size: DVec2 { x: thickness, y: thickness },
            });
        }
    }

    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }

    /// Load nodes and edges from external source
    pub fn load_graph(&mut self, cx: &mut Cx, nodes: Vec<FlowNode>, edges: Vec<EdgeConnection>) {
        self.save_undo_state();
        self.nodes = nodes;
        self.edges = edges;
        self.selected_nodes.clear();
        self.selected_edges.clear();
        self.view.redraw(cx);
    }

    /// Get reference to nodes
    pub fn nodes(&self) -> &Vec<FlowNode> {
        &self.nodes
    }

    /// Get reference to edges
    pub fn edges(&self) -> &Vec<EdgeConnection> {
        &self.edges
    }

    /// Draw edge context menu at screen position
    fn draw_edge_context_menu(&mut self, cx: &mut Cx2d, pos: DVec2, edge_idx: usize) {
        let menu_width = 120.0;
        let item_height = 22.0;
        let padding = 8.0;
        // Total: 3 labels + 9 items + 2 dividers + top/bottom padding
        // = 3*18 + 9*22 + 2*11 + 16 = 54 + 198 + 22 + 16 = 290
        let menu_height = 300.0;

        // Get current edge properties for highlighting
        let (current_style, current_width, current_animated) = if edge_idx < self.edges.len() {
            let edge = &self.edges[edge_idx];
            (edge.style as i32, edge.width as i32, edge.animated)
        } else {
            (0, 2, true)
        };

        // Menu background (light theme)
        self.draw_node_bg.color = vec4(1.0, 1.0, 1.0, 0.98);
        self.draw_node_bg.draw_abs(cx, Rect {
            pos,
            size: DVec2 { x: menu_width, y: menu_height },
        });

        // Border (light theme)
        self.draw_node_bg.color = vec4(0.88, 0.88, 0.88, 1.0); // #e0e0e0
        self.draw_node_bg.draw_abs(cx, Rect { pos, size: DVec2 { x: menu_width, y: 1.0 } });
        self.draw_node_bg.draw_abs(cx, Rect {
            pos: DVec2 { x: pos.x, y: pos.y + menu_height - 1.0 },
            size: DVec2 { x: menu_width, y: 1.0 }
        });
        self.draw_node_bg.draw_abs(cx, Rect { pos, size: DVec2 { x: 1.0, y: menu_height } });
        self.draw_node_bg.draw_abs(cx, Rect {
            pos: DVec2 { x: pos.x + menu_width - 1.0, y: pos.y },
            size: DVec2 { x: 1.0, y: menu_height }
        });

        let mut y = pos.y + padding;
        let label_height = item_height * 0.8;

        // Section label - Style (light theme)
        self.draw_text.text_style.font_size = 9.0;
        self.draw_text.color = vec4(0.6, 0.6, 0.6, 1.0); // #999999
        self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 8.0, y }, "Style");
        y += label_height;

        // Style items (light theme)
        let style_items = ["Solid", "Dashed", "Dotted"];
        for (i, label) in style_items.iter().enumerate() {
            let is_selected = current_style == i as i32;
            self.draw_text.text_style.font_size = 10.0;
            self.draw_text.color = if is_selected {
                self.selection_color
            } else {
                vec4(0.2, 0.2, 0.2, 1.0) // #333333
            };
            let prefix = if is_selected { "> " } else { "  " };
            self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 8.0, y }, &format!("{}{}", prefix, label));
            y += item_height;
        }

        // Divider (light theme)
        self.draw_node_bg.color = vec4(0.88, 0.88, 0.88, 1.0); // #e0e0e0
        self.draw_node_bg.draw_abs(cx, Rect {
            pos: DVec2 { x: pos.x + 8.0, y: y + 2.0 },
            size: DVec2 { x: menu_width - 16.0, y: 1.0 }
        });
        y += item_height * 0.5;

        // Section label - Width (light theme)
        self.draw_text.text_style.font_size = 9.0;
        self.draw_text.color = vec4(0.6, 0.6, 0.6, 1.0); // #999999
        self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 8.0, y }, "Width");
        y += label_height;

        // Width items (light theme)
        let width_items = ["1px", "2px", "3px", "4px"];
        for (i, label) in width_items.iter().enumerate() {
            let is_selected = current_width == (i + 1) as i32;
            self.draw_text.text_style.font_size = 10.0;
            self.draw_text.color = if is_selected {
                self.selection_color
            } else {
                vec4(0.2, 0.2, 0.2, 1.0) // #333333
            };
            let prefix = if is_selected { "> " } else { "  " };
            self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 8.0, y }, &format!("{}{}", prefix, label));
            y += item_height;
        }

        // Divider (light theme)
        self.draw_node_bg.color = vec4(0.88, 0.88, 0.88, 1.0); // #e0e0e0
        self.draw_node_bg.draw_abs(cx, Rect {
            pos: DVec2 { x: pos.x + 8.0, y: y + 2.0 },
            size: DVec2 { x: menu_width - 16.0, y: 1.0 }
        });
        y += item_height * 0.5;

        // Section label - Animation (light theme)
        self.draw_text.text_style.font_size = 9.0;
        self.draw_text.color = vec4(0.6, 0.6, 0.6, 1.0); // #999999
        self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 8.0, y }, "Animation");
        y += label_height;

        // Animation items (light theme)
        let anim_items = ["On", "Off"];
        for (i, label) in anim_items.iter().enumerate() {
            let is_selected = (i == 0) == current_animated;
            self.draw_text.text_style.font_size = 10.0;
            self.draw_text.color = if is_selected {
                self.selection_color
            } else {
                vec4(0.2, 0.2, 0.2, 1.0) // #333333
            };
            let prefix = if is_selected { "> " } else { "  " };
            self.draw_text.draw_abs(cx, DVec2 { x: pos.x + 8.0, y }, &format!("{}{}", prefix, label));
            y += item_height;
        }
    }
}
