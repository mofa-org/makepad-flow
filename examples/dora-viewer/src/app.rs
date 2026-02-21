use makepad_flow::*;
use makepad_widgets::*;
use std::collections::HashMap;
use serde::Deserialize;

use crate::dataflow_tree::{DataflowTreeWidgetRefExt, DataflowTreeHeaderWidgetRefExt, DataflowTreeFooterWidgetRefExt, DataflowTreeAction, TreeNode, TreePort};
use crate::log_panel::{LogPanelWidgetRefExt, LogEntry, LogLevel};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_flow::flow_canvas::*;
    use crate::dataflow_tree::DataflowTree;
    use crate::dataflow_tree::DataflowTreeHeader;
    use crate::dataflow_tree::DataflowTreeFooter;
    use crate::log_panel::LogPanel;

    // Manrope font
    FONT_MANROPE = {
        font_family: {
            latin = font("crate://self/resources/Manrope-Regular.ttf", 0.0, 0.0),
        }
    }

    // Splitter handle style (light theme)
    Splitter = <View> {
        width: 4, height: Fill
        cursor: ColResize
        show_bg: true
        draw_bg: { color: #e0e0e0 }
    }

    App = {{App}} {
        ui: <Window> {
            window: { title: "DORA Viewer", inner_size: vec2(1600, 1000) }
            show_bg: true
            draw_bg: { color: #f0f0f0 }

            body = <View> {
                width: Fill, height: Fill, flow: Down

                // Top toolbar (light theme)
                // Note: left padding is adjusted at runtime for macOS window controls
                toolbar = <View> {
                    width: Fill, height: 44
                    padding: { left: 16, right: 16 }, spacing: 12, align: { y: 0.5 }
                    show_bg: true, draw_bg: { color: #ffffff }

                    <Label> {
                        draw_text: { text_style: <FONT_MANROPE> { font_size: 16.0 }, color: #333333 }
                        text: "DORA Viewer"
                    }
                    <View> { width: 20, height: 1 }

                    file_label = <Label> {
                        draw_text: { text_style: <FONT_MANROPE> { font_size: 11.0 }, color: #666666 }
                        text: "No file loaded"
                    }

                    <View> { width: Fill, height: 1 }

                    reload_btn = <Button> {
                        width: Fit, height: 32, padding: { left: 16, right: 16 }
                        draw_bg: { color: #4A90D9, border_radius: 6.0 }
                        draw_text: {
                            text_style: <FONT_MANROPE> { font_size: 13.0 }
                            fn get_color(self) -> vec4 { return #ffffff; }
                        }
                        text: "Reload"
                    }

                    fit_view_btn = <Button> {
                        width: Fit, height: 32, padding: { left: 16, right: 16 }
                        draw_bg: { color: #4A90D9, border_radius: 6.0 }
                        draw_text: {
                            text_style: <FONT_MANROPE> { font_size: 13.0 }
                            fn get_color(self) -> vec4 { return #ffffff; }
                        }
                        text: "Fit View"
                    }
                }

                // Main area with panels
                main_area = <View> {
                    width: Fill, height: Fill, flow: Right

                    // ========== LEFT PANEL: Dataflow Tree ==========
                    left_panel = <View> {
                        width: 280, height: Fill, flow: Down
                        show_bg: true, draw_bg: { color: #ffffff }

                        // Panel header
                        <View> {
                            width: Fill, height: 40
                            padding: { left: 16, right: 16 }, align: { y: 0.5 }
                            show_bg: true, draw_bg: { color: #f8f8f8 }

                            <Label> {
                                draw_text: { text_style: <FONT_MANROPE> { font_size: 13.0 }, color: #333333 }
                                text: "Dataflow Tree"
                            }
                        }

                        // Search and filter header
                        tree_header = <DataflowTreeHeader> {}

                        // DataflowTree widget (wraps FileTree)
                        dataflow_tree = <DataflowTree> {}

                        // Footer with batch actions
                        tree_footer = <DataflowTreeFooter> {}
                    }

                    // Left splitter
                    left_splitter = <Splitter> {}

                    // ========== CENTER PANEL: FlowCanvas ==========
                    canvas = <FlowCanvas> {
                        width: Fill, height: Fill
                    }
                }

                // Bottom status bar (light theme)
                status_bar = <View> {
                    width: Fill, height: 28
                    padding: { left: 12, right: 12 }, align: { y: 0.5 }
                    show_bg: true, draw_bg: { color: #ffffff }

                    // Legend
                    <View> { flow: Right, spacing: 12, align: { y: 0.5 }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #4a90d9, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: <FONT_MANROPE> { font_size: 9.0 }, color: #666666 }, text: "MaaS" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #22c55e, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: <FONT_MANROPE> { font_size: 9.0 }, color: #666666 }, text: "TTS" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #f59e0b, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: <FONT_MANROPE> { font_size: 9.0 }, color: #666666 }, text: "Bridge" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #ef4444, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: <FONT_MANROPE> { font_size: 9.0 }, color: #666666 }, text: "Controller" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #8b5cf6, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: <FONT_MANROPE> { font_size: 9.0 }, color: #666666 }, text: "MoFA" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #06b6d4, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: <FONT_MANROPE> { font_size: 9.0 }, color: #666666 }, text: "Segmenter" }
                        }
                    }

                    <View> { width: Fill, height: 1 }

                    count_label = <Label> {
                        draw_text: { text_style: <FONT_MANROPE> { font_size: 10.0 }, color: #666666 },
                        text: "Nodes: 0 | Edges: 0 | Enabled: 0"
                    }
                }
            }
        }
    }
}

// ============ YAML Dataflow Parsing ============

#[derive(Debug, Deserialize)]
struct DataflowYaml {
    nodes: Vec<DataflowNodeYaml>,
}

#[derive(Debug, Deserialize)]
struct DataflowNodeYaml {
    id: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    inputs: Option<serde_yaml::Value>,
    #[serde(default)]
    outputs: Option<Vec<String>>,
}

fn parse_dataflow_yaml(yaml_content: &str) -> Result<(Vec<FlowNode>, Vec<EdgeConnection>), String> {
    let dataflow: DataflowYaml = serde_yaml::from_str(yaml_content)
        .map_err(|e| format!("YAML parse error: {}", e))?;

    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut node_id_to_index: HashMap<String, usize> = HashMap::new();

    // First pass: create all nodes
    for (i, node_yaml) in dataflow.nodes.iter().enumerate() {
        let mut input_ports = Vec::new();
        let mut input_sources: Vec<(String, String, String)> = Vec::new();

        if let Some(inputs) = &node_yaml.inputs {
            match inputs {
                serde_yaml::Value::Mapping(map) => {
                    for (key, value) in map {
                        if let Some(port_name) = key.as_str() {
                            input_ports.push(Port::new(port_name));

                            let source = match value {
                                serde_yaml::Value::String(s) => Some(s.clone()),
                                serde_yaml::Value::Mapping(m) => {
                                    m.get(&serde_yaml::Value::String("source".to_string()))
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                }
                                _ => None,
                            };

                            if let Some(src) = source {
                                if let Some((src_node, src_port)) = src.split_once('/') {
                                    input_sources.push((
                                        port_name.to_string(),
                                        src_node.to_string(),
                                        src_port.to_string(),
                                    ));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let output_ports: Vec<Port> = node_yaml.outputs
            .as_ref()
            .map(|outputs| outputs.iter().map(|s| Port::new(s)).collect())
            .unwrap_or_default();

        let category = categorize_node(&node_yaml.id, node_yaml.path.as_deref());

        // Auto-layout: arrange in rows
        let row = i / 4;
        let col = i % 4;
        let x = 50.0 + col as f64 * 280.0;
        let y = 50.0 + row as f64 * 180.0;

        let node = FlowNode::new_dataflow(
            &node_yaml.id,
            x, y,
            &node_yaml.id,
            category,
            input_ports,
            output_ports,
        );

        node_id_to_index.insert(node_yaml.id.clone(), nodes.len());
        nodes.push((node, input_sources));
    }

    // Second pass: create edges
    for (to_node_idx, (_, input_sources)) in nodes.iter().enumerate() {
        for (to_port, from_node_id, from_port) in input_sources {
            if let Some(&from_node_idx) = node_id_to_index.get(from_node_id) {
                edges.push(EdgeConnection::new_with_ports(
                    from_node_idx,
                    from_port,
                    to_node_idx,
                    to_port,
                ));
            }
        }
    }

    let nodes: Vec<FlowNode> = nodes.into_iter().map(|(n, _)| n).collect();
    Ok((nodes, edges))
}

fn categorize_node(id: &str, path: Option<&str>) -> NodeCategory {
    let id_lower = id.to_lowercase();

    if id_lower.starts_with("mofa-") {
        return NodeCategory::MoFA;
    }
    if id_lower.starts_with("bridge-") {
        return NodeCategory::Bridge;
    }
    if id_lower.contains("controller") {
        return NodeCategory::Controller;
    }
    if id_lower.contains("primespeech") || id_lower.contains("tts") {
        return NodeCategory::TTS;
    }
    if id_lower.contains("segmenter") {
        return NodeCategory::Segmenter;
    }
    if id_lower.contains("student") || id_lower.contains("tutor") {
        return NodeCategory::MaaS;
    }

    if let Some(p) = path {
        if p == "dynamic" {
            return NodeCategory::MoFA;
        }
    }

    NodeCategory::Default
}

// ============ App State ============

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] loaded_nodes: Vec<FlowNode>,
    #[rust] loaded_edges: Vec<EdgeConnection>,
    #[rust] node_enabled: HashMap<String, bool>,
    // Splitter state
    #[rust] left_panel_width: f64,
    #[rust] left_dragging: bool,
}

impl App {
    const MIN_LEFT_WIDTH: f64 = 200.0;
    const DEFAULT_LEFT_WIDTH: f64 = 300.0;
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_flow::live_design(cx);
        crate::dataflow_tree::register_live_design(cx);
        crate::log_panel::register_live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // Initialize panel width
        self.left_panel_width = Self::DEFAULT_LEFT_WIDTH;

        // Adjust toolbar padding for macOS window controls (traffic lights)
        if let OsType::Macos = cx.os_type() {
            self.ui.view(id!(toolbar)).apply_over(cx, live! {
                padding: { left: 80.0, right: 16.0 }
            });
        }

        // Load dataflow YAML
        let yaml_paths = [
            "examples/dora-viewer/dataflow/voice-chat.yml",
            "dataflow/voice-chat.yml",
        ];

        for path in &yaml_paths {
            if let Ok(yaml_content) = std::fs::read_to_string(path) {
                match parse_dataflow_yaml(&yaml_content) {
                    Ok((nodes, edges)) => {
                        log!("Loaded dataflow from {}: {} nodes, {} edges", path, nodes.len(), edges.len());

                        // Initialize node enabled state
                        for node in &nodes {
                            self.node_enabled.insert(node.id.clone(), true);
                        }

                        self.loaded_nodes = nodes;
                        self.loaded_edges = edges;

                        // Update UI
                        self.ui.label(ids!(file_label)).set_text(cx, path);
                        self.update_node_count_label(cx);
                        self.update_status_bar(cx);

                        // Load into canvas
                        cx.action(FlowCanvasCommand::LoadDataflow {
                            nodes: self.loaded_nodes.clone(),
                            edges: self.loaded_edges.clone(),
                        });

                        // Populate the DataflowTree
                        self.populate_dataflow_tree(cx);
                        log!("Ctrl+click on tree items to toggle enabled state");

                        // Log panel disabled for now
                        // self.add_demo_logs(cx);

                        break;
                    }
                    Err(e) => {
                        log!("Failed to parse {}: {}", path, e);
                    }
                }
            }
        }

        self.ui.redraw(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Fit view button
        if self.ui.button(ids!(fit_view_btn)).clicked(actions) {
            cx.action(FlowCanvasCommand::FitView);
        }

        // Reload button
        if self.ui.button(ids!(reload_btn)).clicked(actions) {
            // Re-trigger startup to reload
            self.handle_startup(cx);
        }

        // LogPanel disabled for now
        // let log_panel = self.ui.log_panel(ids!(log_panel));
        // if log_panel.clear_clicked(actions) {
        //     log_panel.clear_entries(cx);
        // }

        // Handle DataflowTree header actions
        let tree_header = self.ui.dataflow_tree_header(ids!(tree_header));
        let dataflow_tree = self.ui.dataflow_tree(ids!(dataflow_tree));
        let tree_footer = self.ui.dataflow_tree_footer(ids!(tree_footer));

        // Handle search
        if let Some(text) = tree_header.search_changed(actions) {
            dataflow_tree.set_search_filter(cx, text);
            tree_footer.set_node_count(cx, dataflow_tree.get_node_count());
        }

        // Handle category filter
        if let Some(category) = tree_header.filter_category(actions) {
            dataflow_tree.set_category_filter(cx, category);
            tree_footer.set_node_count(cx, dataflow_tree.get_node_count());
        }

        // Handle expand/collapse
        if tree_header.expand_all_clicked(actions) {
            dataflow_tree.expand_all(cx);
        }
        if tree_header.collapse_all_clicked(actions) {
            dataflow_tree.collapse_all(cx);
        }

        // Handle toggle matching ports (from header button)
        if tree_header.toggle_match_clicked(actions) {
            log!("App: toggle_match_clicked detected from header");
            if dataflow_tree.toggle_matching_ports_from_app(cx) {
                log!("App: Toggled matching ports via header button");
                self.reload_flow_with_enabled_filter(cx);
            }
        }

        // Handle footer actions
        if tree_footer.enable_all_clicked(actions) {
            for (_, enabled) in self.node_enabled.iter_mut() {
                *enabled = true;
            }
            self.populate_dataflow_tree(cx);
            self.update_status_bar(cx);
            self.ui.redraw(cx);
        }

        if tree_footer.disable_all_clicked(actions) {
            for (_, enabled) in self.node_enabled.iter_mut() {
                *enabled = false;
            }
            self.populate_dataflow_tree(cx);
            self.update_status_bar(cx);
            self.ui.redraw(cx);
        }

        // Handle toggle matching ports button
        if tree_footer.toggle_matching_clicked(actions) {
            if dataflow_tree.toggle_matching_ports_from_app(cx) {
                log!("App: Toggled matching ports via button");
                self.reload_flow_with_enabled_filter(cx);
            }
        }

        // Handle tree node/port enable/disable actions
        if let Some((node_id, enabled)) = dataflow_tree.node_enabled_changed(actions) {
            log!("App: Node {} enabled changed to {}", node_id, enabled);
            self.node_enabled.insert(node_id, enabled);
            self.reload_flow_with_enabled_filter(cx);
        }
        if let Some((node_id, port_id, enabled)) = dataflow_tree.port_enabled_changed(actions) {
            log!("App: Port {}/{} enabled changed to {}", node_id, port_id, enabled);
            // For now, port-level changes also trigger a reload
            // In future, could have finer-grained edge filtering
            self.reload_flow_with_enabled_filter(cx);
        }

        // Canvas status updates
        for action in actions {
            if let FlowCanvasAction::StatusUpdate { nodes, edges } = action.cast() {
                let enabled_count = self.node_enabled.values().filter(|&&e| e).count();
                let text = format!("Nodes: {} | Edges: {} | Enabled: {}", nodes, edges, enabled_count);
                self.ui.label(ids!(count_label)).set_text(cx, &text);
            }
        }
    }
}

impl App {
    fn update_node_count_label(&mut self, cx: &mut Cx) {
        let tree_footer = self.ui.dataflow_tree_footer(ids!(tree_footer));
        tree_footer.set_node_count(cx, self.loaded_nodes.len());
    }

    fn update_status_bar(&mut self, cx: &mut Cx) {
        let enabled_count = self.node_enabled.values().filter(|&&e| e).count();
        let text = format!(
            "Nodes: {} | Edges: {} | Enabled: {}",
            self.loaded_nodes.len(),
            self.loaded_edges.len(),
            enabled_count
        );
        self.ui.label(ids!(count_label)).set_text(cx, &text);
    }

    fn reload_flow_with_enabled_filter(&mut self, cx: &mut Cx) {
        // Get current enabled states from the tree widget
        let dataflow_tree = self.ui.dataflow_tree(ids!(dataflow_tree));
        let node_states = dataflow_tree.get_node_enabled_states();
        let port_states = dataflow_tree.get_port_enabled_states();

        // Step 1: Find all edges that have both ports enabled
        // We need to do this first to determine which nodes have connections
        let mut valid_edges: Vec<(usize, usize, EdgeConnection)> = Vec::new();

        for edge in &self.loaded_edges {
            let from_node = &self.loaded_nodes[edge.from_node];
            let to_node = &self.loaded_nodes[edge.to_node];

            // Check if both nodes are enabled
            let from_node_enabled = node_states.get(&from_node.id).copied().unwrap_or(true);
            let to_node_enabled = node_states.get(&to_node.id).copied().unwrap_or(true);

            if !from_node_enabled || !to_node_enabled {
                continue;
            }

            // Check if the source port (output) is enabled
            let from_port_enabled = port_states
                .get(&(from_node.id.clone(), edge.from_port.clone(), false))
                .copied()
                .unwrap_or(true);

            // Check if the target port (input) is enabled
            let to_port_enabled = port_states
                .get(&(to_node.id.clone(), edge.to_port.clone(), true))
                .copied()
                .unwrap_or(true);

            // Only include edge if both ports are enabled
            if from_port_enabled && to_port_enabled {
                valid_edges.push((edge.from_node, edge.to_node, edge.clone()));
            }
        }

        // Step 2: Find nodes that have at least one valid connection
        let mut connected_node_indices: std::collections::HashSet<usize> = std::collections::HashSet::new();
        for (from_idx, to_idx, _) in &valid_edges {
            connected_node_indices.insert(*from_idx);
            connected_node_indices.insert(*to_idx);
        }

        // Step 3: Build list of nodes to show (only those with connections)
        let mut enabled_indices: Vec<usize> = Vec::new();
        let mut old_to_new_idx: HashMap<usize, usize> = HashMap::new();

        for (old_idx, node) in self.loaded_nodes.iter().enumerate() {
            // Node must be enabled AND have at least one connection
            let node_enabled = node_states.get(&node.id).copied().unwrap_or(true);
            let has_connections = connected_node_indices.contains(&old_idx);

            if node_enabled && has_connections {
                let new_idx = enabled_indices.len();
                old_to_new_idx.insert(old_idx, new_idx);
                enabled_indices.push(old_idx);
            }
        }

        // Step 4: Collect enabled nodes
        let enabled_nodes: Vec<FlowNode> = enabled_indices.iter()
            .map(|&idx| self.loaded_nodes[idx].clone())
            .collect();

        // Step 5: Remap edges to new indices
        let enabled_edges: Vec<EdgeConnection> = valid_edges.iter()
            .filter_map(|(from_idx, to_idx, edge)| {
                if let (Some(&new_from), Some(&new_to)) = (
                    old_to_new_idx.get(from_idx),
                    old_to_new_idx.get(to_idx)
                ) {
                    let mut new_edge = edge.clone();
                    new_edge.from_node = new_from;
                    new_edge.to_node = new_to;
                    Some(new_edge)
                } else {
                    None
                }
            })
            .collect();

        log!("Reloading flow: {} enabled nodes, {} enabled edges",
             enabled_nodes.len(), enabled_edges.len());

        // Reload the flow canvas with filtered data
        cx.action(FlowCanvasCommand::LoadDataflow {
            nodes: enabled_nodes,
            edges: enabled_edges,
        });

        self.update_status_bar(cx);
    }

    fn populate_dataflow_tree(&mut self, cx: &mut Cx) {
        // Convert FlowNodes to TreeNodes for the DataflowTree widget
        let tree_nodes: Vec<TreeNode> = self.loaded_nodes.iter().map(|flow_node| {
            // Get enabled state
            let enabled = self.node_enabled.get(&flow_node.id).copied().unwrap_or(true);

            // Build ports list from input and output ports
            let mut ports = Vec::new();

            for port in &flow_node.input_ports {
                ports.push(TreePort {
                    id: port.id.clone(),
                    label: port.label.clone(),
                    is_input: true,
                    enabled: true,
                });
            }

            for port in &flow_node.output_ports {
                ports.push(TreePort {
                    id: port.id.clone(),
                    label: port.label.clone(),
                    is_input: false,
                    enabled: true,
                });
            }

            TreeNode {
                id: flow_node.id.clone(),
                label: flow_node.title.clone(),
                category: flow_node.category,
                enabled,
                ports,
            }
        }).collect();

        // Set the nodes on the DataflowTree widget
        self.ui.dataflow_tree(ids!(dataflow_tree)).set_nodes(cx, tree_nodes);
    }

    fn add_demo_logs(&mut self, cx: &mut Cx) {
        // Add some demo log entries for testing
        let demo_logs = vec![
            (LogLevel::Info, "student1", "Initialized MaaS client"),
            (LogLevel::Info, "student2", "Initialized MaaS client"),
            (LogLevel::Info, "tutor", "Initialized MaaS client"),
            (LogLevel::Debug, "conference-controller", "Policy pattern loaded"),
            (LogLevel::Info, "bridge-to-student1", "Bridge connected"),
            (LogLevel::Info, "bridge-to-student2", "Bridge connected"),
            (LogLevel::Info, "bridge-to-tutor", "Bridge connected"),
            (LogLevel::Warn, "multi-text-segmenter", "High buffer threshold"),
            (LogLevel::Info, "primespeech-student1", "TTS engine ready"),
            (LogLevel::Info, "primespeech-student2", "TTS engine ready"),
            (LogLevel::Info, "primespeech-tutor", "TTS engine ready"),
            (LogLevel::Debug, "mofa-audio-player", "Audio buffer initialized"),
            (LogLevel::Info, "mofa-prompt-input", "UI widget ready"),
            (LogLevel::Info, "mofa-system-log", "Log aggregator started"),
        ];

        let entries: Vec<LogEntry> = demo_logs.into_iter().enumerate().map(|(i, (level, node_id, message))| {
            LogEntry {
                timestamp: format!("12:34:{:02}", i),
                level,
                node_id: node_id.to_string(),
                message: message.to_string(),
            }
        }).collect();

        self.ui.log_panel(ids!(log_panel)).set_entries(cx, entries);
    }

    fn handle_splitter_events(&mut self, cx: &mut Cx, event: &Event) {
        // Left splitter
        let left_splitter = self.ui.view(ids!(left_splitter));
        match event.hits(cx, left_splitter.area()) {
            Hit::FingerDown(_) => {
                self.left_dragging = true;
            }
            Hit::FingerUp(_) => {
                self.left_dragging = false;
            }
            Hit::FingerMove(fm) => {
                if self.left_dragging {
                    let body_rect = self.ui.view(ids!(main_area)).area().rect(cx);
                    let new_width = (fm.abs.x - body_rect.pos.x).max(Self::MIN_LEFT_WIDTH);
                    self.left_panel_width = new_width;
                    self.ui.view(ids!(left_panel)).apply_over(cx, live! {
                        width: (new_width)
                    });
                    self.ui.redraw(cx);
                }
            }
            _ => {}
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Handle global keyboard shortcuts
        if let Event::KeyDown(key_event) = event {
            let modifiers = cx.keyboard.modifiers();
            let ctrl_or_cmd = modifiers.control || modifiers.logo; // Support both Ctrl and Cmd (macOS)
            let shift = modifiers.shift;

            // Ctrl+Shift+D or Cmd+Shift+D: Toggle all ports matching search filter
            if ctrl_or_cmd && shift && key_event.key_code == KeyCode::KeyD {
                log!("App: Ctrl/Cmd+Shift+D pressed - toggling matching ports");
                let dataflow_tree = self.ui.dataflow_tree(ids!(dataflow_tree));
                if dataflow_tree.toggle_matching_ports_from_app(cx) {
                    log!("App: Toggled matching ports");
                    self.reload_flow_with_enabled_filter(cx);
                }
            }
        }

        self.match_event(cx, event);
        self.handle_splitter_events(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
