use makepad_flow::*;
use makepad_widgets::*;
use std::collections::HashMap;
use serde::Deserialize;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_flow::flow_canvas::*;

    App = {{App}} {
        ui: <Window> {
            window: { title: "Dora Viewer", inner_size: vec2(1400, 900) }
            show_bg: true
            draw_bg: { color: #1a1a2e }

            body = <View> {
                width: Fill, height: Fill, flow: Down

                // Toolbar
                <View> {
                    width: Fill, height: 48
                    padding: { left: 12, right: 12 }, spacing: 8, align: { y: 0.5 }
                    show_bg: true, draw_bg: { color: #252538 }

                    <Label> { draw_text: { text_style: { font_size: 14.0 }, color: #e0e0e0 }, text: "Dora Viewer" }
                    <View> { width: 20, height: 1 }

                    file_label = <Label> {
                        draw_text: { text_style: { font_size: 10.0 }, color: #8080a0 },
                        text: "No file loaded"
                    }

                    <View> { width: Fill, height: 1 }

                    fit_view_btn = <Button> {
                        width: Fit, height: 32, padding: { left: 12, right: 12 }
                        draw_bg: { color: #3d3d5c }
                        text: "Fit View"
                    }
                }

                // Canvas area
                canvas = <FlowCanvas> {}

                // Status bar
                <View> {
                    width: Fill, height: 28
                    padding: { left: 12, right: 12 }, align: { y: 0.5 }
                    show_bg: true, draw_bg: { color: #252538 }

                    // Legend
                    <View> { flow: Right, spacing: 12, align: { y: 0.5 }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #4a90d9, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: { font_size: 9.0 }, color: #8080a0 }, text: "MaaS" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #22c55e, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: { font_size: 9.0 }, color: #8080a0 }, text: "TTS" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #f59e0b, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: { font_size: 9.0 }, color: #8080a0 }, text: "Bridge" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #ef4444, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: { font_size: 9.0 }, color: #8080a0 }, text: "Controller" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #8b5cf6, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: { font_size: 9.0 }, color: #8080a0 }, text: "MoFA" }
                        }
                        <View> { flow: Right, spacing: 4, align: { y: 0.5 }
                            <RoundedView> { width: 10, height: 10, draw_bg: { color: #06b6d4, border_radius: 2.0 } }
                            <Label> { draw_text: { text_style: { font_size: 9.0 }, color: #8080a0 }, text: "Segmenter" }
                        }
                    }

                    <View> { width: Fill, height: 1 }
                    count_label = <Label> {
                        draw_text: { text_style: { font_size: 10.0 }, color: #8080a0 },
                        text: "Nodes: 0 | Edges: 0"
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
        // Parse inputs
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

        // Parse outputs
        let output_ports: Vec<Port> = node_yaml.outputs
            .as_ref()
            .map(|outputs| outputs.iter().map(|s| Port::new(s)).collect())
            .unwrap_or_default();

        // Determine category from node id
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

// App
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] loaded_nodes: Vec<FlowNode>,
    #[rust] loaded_edges: Vec<EdgeConnection>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_flow::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // Try to load dataflow YAML from dora-viewer/dataflow directory
        let yaml_paths = [
            "examples/dora-viewer/dataflow/voice-chat.yml",
            "dataflow/voice-chat.yml",
        ];

        for path in &yaml_paths {
            if let Ok(yaml_content) = std::fs::read_to_string(path) {
                match parse_dataflow_yaml(&yaml_content) {
                    Ok((nodes, edges)) => {
                        log!("Loaded dataflow from {}: {} nodes, {} edges", path, nodes.len(), edges.len());
                        self.loaded_nodes = nodes;
                        self.loaded_edges = edges;

                        // Update file label
                        self.ui.label(id!(file_label)).set_text(cx, path);

                        // Load into canvas via command
                        cx.action(FlowCanvasCommand::LoadDataflow {
                            nodes: self.loaded_nodes.clone(),
                            edges: self.loaded_edges.clone(),
                        });
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
        if self.ui.button(id!(fit_view_btn)).clicked(actions) {
            cx.action(FlowCanvasCommand::FitView);
        }

        for action in actions {
            if let FlowCanvasAction::StatusUpdate { nodes, edges } = action.cast() {
                let text = format!("Nodes: {} | Edges: {}", nodes, edges);
                self.ui.label(id!(count_label)).set_text(cx, &text);
            }
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
