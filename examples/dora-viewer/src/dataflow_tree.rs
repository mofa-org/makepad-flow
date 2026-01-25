#![allow(dead_code)]

use makepad_widgets::*;
use makepad_widgets::file_tree::{FileTree, FileTreeAction};
use makepad_flow::NodeCategory;
use std::collections::HashMap;

pub fn register_live_design(cx: &mut Cx) {
    self::live_design(cx);
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Header with search and filters - separate from the tree (light theme)
    pub DataflowTreeHeader = {{DataflowTreeHeader}} {
        width: Fill, height: Fit
        flow: Down
        show_bg: true
        draw_bg: { color: #f8f8f8 }

        // Search bar
        search_row = <View> {
            width: Fill, height: Fit
            padding: 8
            flow: Down, spacing: 4

            search_input = <TextInput> {
                width: Fill, height: 28
                empty_text: "Search nodes..."
                draw_bg: {
                    color: #f5f5f5
                    border_radius: 4.0
                }
                draw_text: {
                    color: #333333
                    text_style: { font_size: 10.0 }
                }
            }

            filter_row = <View> {
                width: Fill, height: Fit
                flow: Right, spacing: 4

                expand_all = <Button> {
                    width: Fit, height: 24
                    padding: { left: 8, right: 8 }
                    draw_bg: { color: #e8e8e8, border_radius: 4.0 }
                    text: "Expand"
                    draw_text: {
                        text_style: { font_size: 9.0 }
                        fn get_color(self) -> vec4 { return #555555; }
                    }
                }

                collapse_all = <Button> {
                    width: Fit, height: 24
                    padding: { left: 8, right: 8 }
                    draw_bg: { color: #e8e8e8, border_radius: 4.0 }
                    text: "Collapse"
                    draw_text: {
                        text_style: { font_size: 9.0 }
                        fn get_color(self) -> vec4 { return #555555; }
                    }
                }

                <View> { width: Fill, height: 1 }

                toggle_match = <Button> {
                    width: Fit, height: 24
                    padding: { left: 8, right: 8 }
                    draw_bg: { color: #e8f4fd, border_radius: 4.0 }
                    text: "Toggle Match"
                    draw_text: {
                        text_style: { font_size: 9.0 }
                        fn get_color(self) -> vec4 {
                            return #4A90D9;
                        }
                    }
                }
            }
        }

        // Category filter chips
        category_row = <View> {
            width: Fill, height: Fit
            padding: { left: 8, right: 8, bottom: 4 }
            flow: Right, spacing: 4

            <Label> {
                width: Fit, height: Fit
                draw_text: { color: #888888, text_style: { font_size: 9.0 } }
                text: "Filter:"
            }

            filter_all = <Button> {
                width: Fit, height: 22
                padding: { left: 8, right: 8 }
                draw_bg: { color: #4A90D9, border_radius: 4.0 }
                text: "All"
                draw_text: {
                    text_style: { font_size: 9.0 }
                    fn get_color(self) -> vec4 { return #ffffff; }
                }
            }

            filter_maas = <Button> {
                width: Fit, height: 22
                padding: { left: 8, right: 8 }
                draw_bg: { color: #e8f4fd, border_radius: 4.0 }
                text: "MaaS"
                draw_text: {
                    text_style: { font_size: 9.0 }
                    fn get_color(self) -> vec4 { return #4a90d9; }
                }
            }

            filter_tts = <Button> {
                width: Fit, height: 22
                padding: { left: 8, right: 8 }
                draw_bg: { color: #e6f7ed, border_radius: 4.0 }
                text: "TTS"
                draw_text: {
                    text_style: { font_size: 9.0 }
                    fn get_color(self) -> vec4 { return #22c55e; }
                }
            }

            filter_bridge = <Button> {
                width: Fit, height: 22
                padding: { left: 8, right: 8 }
                draw_bg: { color: #fef3e2, border_radius: 4.0 }
                text: "Bridge"
                draw_text: {
                    text_style: { font_size: 9.0 }
                    fn get_color(self) -> vec4 { return #f59e0b; }
                }
            }
        }
    }

    // Footer with batch actions - separate from the tree (light theme)
    pub DataflowTreeFooter = {{DataflowTreeFooter}} {
        width: Fill, height: Fit
        flow: Right
        padding: 8
        spacing: 4
        show_bg: true
        draw_bg: { color: #f8f8f8 }

        enable_selected = <Button> {
            width: Fit, height: 28
            padding: { left: 12, right: 12 }
            draw_bg: { color: #22c55e, border_radius: 4.0 }
            text: "Enable All"
            draw_text: {
                text_style: { font_size: 9.0 }
                fn get_color(self) -> vec4 { return #ffffff; }
            }
        }

        disable_selected = <Button> {
            width: Fit, height: 28
            padding: { left: 12, right: 12 }
            draw_bg: { color: #ef4444, border_radius: 4.0 }
            text: "Disable All"
            draw_text: {
                text_style: { font_size: 9.0 }
                fn get_color(self) -> vec4 { return #ffffff; }
            }
        }

        toggle_matching = <Button> {
            width: Fit, height: 28
            padding: { left: 12, right: 12 }
            draw_bg: { color: #4A90D9, border_radius: 4.0 }
            text: "Toggle Match"
            draw_text: {
                text_style: { font_size: 9.0 }
                fn get_color(self) -> vec4 { return #ffffff; }
            }
        }

        <View> { width: Fill, height: 1 }

        node_count = <Label> {
            width: Fit, height: Fit
            draw_text: { color: #666666, text_style: { font_size: 9.0 } }
            text: "0 nodes"
        }
    }

    // The actual tree widget - wraps FileTree directly (light theme)
    pub DataflowTree = {{DataflowTree}} {
        file_tree: <FileTree> {
            width: Fill
            height: Fill

            node_height: 24.0

            scroll_bars: <ScrollBars> {
                show_scroll_x: false
                show_scroll_y: true
            }

            file_node: <FileTreeNode> {
                is_folder: false
                indent_width: 14.0

                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                        sdf.fill(mix(
                            mix(#ffffff, #e8f4fd, self.hover),
                            #d0e8ff,
                            self.active
                        ));
                        return sdf.result;
                    }
                }

                draw_text: {
                    text_style: { font_size: 10.0 }
                    fn get_color(self) -> vec4 {
                        return mix(#555555, #333333, self.active);
                    }
                }

                draw_icon: {
                    fn get_color(self) -> vec4 {
                        return mix(#888888, #666666, self.hover);
                    }
                }
            }

            folder_node: <FileTreeNode> {
                is_folder: true
                indent_width: 14.0

                draw_bg: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.rect(0., 0., self.rect_size.x, self.rect_size.y);
                        sdf.fill(mix(
                            mix(#ffffff, #e8f4fd, self.hover),
                            #d0e8ff,
                            self.active
                        ));
                        return sdf.result;
                    }
                }

                draw_text: {
                    text_style: { font_size: 10.0 }
                    fn get_color(self) -> vec4 {
                        return mix(#333333, #222222, self.active);
                    }
                }

                draw_icon: {
                    fn get_color(self) -> vec4 {
                        return mix(#4a90d9, #3080c9, self.hover);
                    }
                }
            }

            filler: {
                fn pixel(self) -> vec4 {
                    return #ffffff;
                }
            }
        }
    }
}

// Tree node data
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub category: NodeCategory,
    pub enabled: bool,
    pub ports: Vec<TreePort>,
}

#[derive(Clone, Debug)]
pub struct TreePort {
    pub id: String,
    pub label: String,
    pub is_input: bool,
    pub enabled: bool,
}

// Internal tree structure for FileTree
#[derive(Debug)]
struct FileNode {
    name: String,
    child_edges: Option<Vec<FileEdge>>,
    item_type: TreeItemType,
}

#[derive(Debug, Clone)]
enum TreeItemType {
    Root,
    Node { node_id: String },
    InputsFolder { node_id: String },
    OutputsFolder { node_id: String },
    Port { node_id: String, port_id: String, is_input: bool },
}

#[derive(Debug)]
struct FileEdge {
    name: String,
    file_node_id: LiveId,
}

// Actions emitted by DataflowTree
#[derive(Clone, Debug, DefaultNone)]
pub enum DataflowTreeAction {
    None,
    NodeEnabledChanged { node_id: String, enabled: bool },
    PortEnabledChanged { node_id: String, port_id: String, enabled: bool },
    NodeSelected { node_id: String },
    SearchChanged { text: String },
    FilterCategory { category: Option<NodeCategory> },
    ExpandAll,
    CollapseAll,
    EnableAllNodes,
    DisableAllNodes,
    ToggleMatchingPorts,
}

// ============================================================================
// DataflowTreeHeader - Search and filter controls
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct DataflowTreeHeader {
    #[deref] view: View,
}

impl Widget for DataflowTreeHeader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for DataflowTreeHeader {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if let Some(text) = self.view.text_input(ids!(search_input)).changed(actions) {
            cx.widget_action(self.widget_uid(), &scope.path,
                DataflowTreeAction::SearchChanged { text });
        }

        if self.view.button(ids!(filter_all)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path,
                DataflowTreeAction::FilterCategory { category: None });
        }
        if self.view.button(ids!(filter_maas)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path,
                DataflowTreeAction::FilterCategory { category: Some(NodeCategory::MaaS) });
        }
        if self.view.button(ids!(filter_tts)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path,
                DataflowTreeAction::FilterCategory { category: Some(NodeCategory::TTS) });
        }
        if self.view.button(ids!(filter_bridge)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path,
                DataflowTreeAction::FilterCategory { category: Some(NodeCategory::Bridge) });
        }

        if self.view.button(ids!(expand_all)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, DataflowTreeAction::ExpandAll);
        }
        if self.view.button(ids!(collapse_all)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, DataflowTreeAction::CollapseAll);
        }
        if self.view.button(ids!(toggle_match)).clicked(actions) {
            log!("DataflowTreeHeader: Toggle Match button clicked");
            cx.widget_action(self.widget_uid(), &scope.path, DataflowTreeAction::ToggleMatchingPorts);
        }
    }
}

// ============================================================================
// DataflowTreeFooter - Batch actions
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct DataflowTreeFooter {
    #[deref] view: View,
}

impl Widget for DataflowTreeFooter {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for DataflowTreeFooter {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self.view.button(ids!(enable_selected)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, DataflowTreeAction::EnableAllNodes);
        }
        if self.view.button(ids!(disable_selected)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, DataflowTreeAction::DisableAllNodes);
        }
        if self.view.button(ids!(toggle_matching)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, DataflowTreeAction::ToggleMatchingPorts);
        }
    }
}

impl DataflowTreeFooterRef {
    pub fn set_node_count(&self, cx: &mut Cx, count: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.label(ids!(node_count)).set_text(cx, &format!("{} nodes", count));
        }
    }
}

// ============================================================================
// DataflowTree - The actual tree widget (wraps FileTree)
// ============================================================================

#[derive(Live, LiveHook, Widget)]
pub struct DataflowTree {
    #[wrap]
    #[live]
    pub file_tree: FileTree,

    #[rust] nodes: Vec<TreeNode>,
    #[rust] search_filter: String,
    #[rust] category_filter: Option<NodeCategory>,
    #[rust] selected_node: Option<String>,
    #[rust] file_nodes: LiveIdMap<LiveId, FileNode>,
    #[rust] node_id_to_live_id: HashMap<String, LiveId>,
    #[rust] live_id_counter: u64,
    #[rust] initialized: bool,
    // Track folder open states so we can restore them on Ctrl+click
    #[rust] folder_open_states: HashMap<LiveId, bool>,
}

impl Widget for DataflowTree {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        // Check if Ctrl is held before handling event
        let ctrl_held = cx.keyboard.modifiers().control;
        let shift_held = cx.keyboard.modifiers().shift;

        // Handle keyboard shortcuts for batch operations
        if let Event::KeyDown(key_event) = event {
            // Ctrl+Shift+D: Toggle all ports matching the search filter
            if ctrl_held && shift_held && key_event.key_code == KeyCode::KeyD {
                if !self.search_filter.is_empty() {
                    self.toggle_matching_ports(cx, scope);
                }
            }
        }

        // Let FileTree handle the event and capture any actions
        let actions = cx.capture_actions(|cx| {
            self.file_tree.handle_event(cx, event, scope);
        });

        // Process FileTree actions
        if let Some(item) = actions.find_widget_action(self.file_tree.widget_uid()) {
            let action: FileTreeAction = item.cast();
            match action {
                FileTreeAction::FileClicked(file_id) => {
                    if ctrl_held {
                        log!("Ctrl+FileClicked: {:?}", file_id);
                        self.toggle_item_enabled(cx, file_id, scope);
                    }
                }
                FileTreeAction::FolderClicked(file_id) => {
                    if ctrl_held {
                        log!("Ctrl+FolderClicked: {:?} - toggling enable state", file_id);
                        self.toggle_item_enabled(cx, file_id, scope);
                        // Restore the fold state from our tracked state
                        // FileTree just toggled it, so we set it back to what we had
                        if let Some(&was_open) = self.folder_open_states.get(&file_id) {
                            self.file_tree.set_folder_is_open(cx, file_id.into(), was_open, Animate::No);
                        }
                    } else {
                        // Normal click - update our tracked state (toggled from previous)
                        let was_open = self.folder_open_states.get(&file_id).copied().unwrap_or(false);
                        self.folder_open_states.insert(file_id, !was_open);
                    }
                }
                FileTreeAction::ShouldFileStartDrag(_) => {}
                FileTreeAction::None => {}
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Build tree data on first draw or when nodes change
        if !self.initialized && !self.nodes.is_empty() {
            self.build_file_tree_data();
            self.initialized = true;
        }

        // Drive the FileTree's draw loop (same pattern as flex-layout-demo)
        while self.file_tree.draw_walk(cx, scope, walk).is_step() {
            // Sync all folder states with FileTree
            for (&folder_id, &is_open) in &self.folder_open_states {
                self.file_tree.set_folder_is_open(cx, folder_id.into(), is_open, Animate::No);
            }

            // Draw the tree starting from root
            Self::draw_file_node(
                cx,
                live_id!(dataflow_root),
                &mut self.file_tree,
                &self.file_nodes,
            );
        }

        DrawStep::done()
    }
}

// Note: We don't use WidgetMatchEvent for DataflowTree because we capture
// FileTree actions directly in handle_event using cx.capture_actions()

impl DataflowTree {
    /// Process actions captured from the FileTree
    fn process_file_tree_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let ctrl_held = cx.keyboard.modifiers().control;

        // Find actions from the FileTree widget specifically
        if let Some(item) = actions.find_widget_action(self.file_tree.widget_uid()) {
            let action: FileTreeAction = item.cast();
            match action {
                FileTreeAction::FileClicked(file_id) => {
                    log!("FileClicked: {:?}, ctrl={}", file_id, ctrl_held);
                    if ctrl_held {
                        self.toggle_item_enabled(cx, file_id, scope);
                    }
                }
                FileTreeAction::FolderClicked(file_id) => {
                    log!("FolderClicked: {:?}, ctrl={}", file_id, ctrl_held);
                    if ctrl_held {
                        self.toggle_item_enabled(cx, file_id, scope);
                    }
                }
                FileTreeAction::ShouldFileStartDrag(file_id) => {
                    log!("ShouldFileStartDrag: {:?}, ctrl={}", file_id, ctrl_held);
                    // Could use this for drag-to-toggle, but for now just log
                }
                FileTreeAction::None => {}
            }
        }
    }

    /// Toggle the enabled state of a tree item with proper cascading
    fn toggle_item_enabled(&mut self, cx: &mut Cx, file_id: LiveId, scope: &mut Scope) {
        log!("toggle_item_enabled for {:?}", file_id);

        if let Some(file_node) = self.file_nodes.get(&file_id) {
            match &file_node.item_type {
                TreeItemType::Node { node_id } => {
                    // Toggle node - cascades to all children
                    let node_id = node_id.clone();
                    if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                        let new_state = !node.enabled;
                        node.enabled = new_state;
                        // Cascade: when node is disabled, disable all ports
                        // When node is enabled, enable all ports
                        for port in &mut node.ports {
                            port.enabled = new_state;
                        }
                        log!("Toggled node {} and all ports to enabled={}", node_id, new_state);
                        cx.widget_action(self.widget_uid(), &scope.path,
                            DataflowTreeAction::NodeEnabledChanged {
                                node_id,
                                enabled: new_state
                            });
                        // Rebuild tree data synchronously to keep FileTree cache in sync
                        self.build_file_tree_data();
                        self.file_tree.redraw(cx);
                    }
                }
                TreeItemType::Port { node_id, port_id, is_input } => {
                    // Toggle individual port - updates parent node state
                    let node_id = node_id.clone();
                    let port_id = port_id.clone();
                    let is_input = *is_input;
                    if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                        // Match both port_id AND is_input to handle duplicate port names
                        if let Some(port) = node.ports.iter_mut().find(|p| p.id == port_id && p.is_input == is_input) {
                            port.enabled = !port.enabled;
                            let enabled = port.enabled;
                            log!("Toggled port {}/{} to enabled={}", node_id, port_id, enabled);
                            cx.widget_action(self.widget_uid(), &scope.path,
                                DataflowTreeAction::PortEnabledChanged {
                                    node_id: node_id.clone(),
                                    port_id,
                                    enabled
                                });
                        }
                        // Update node enabled state based on children
                        self.update_node_state_from_children(&node_id);
                        // Rebuild tree data synchronously to keep FileTree cache in sync
                        self.build_file_tree_data();
                        self.file_tree.redraw(cx);
                    }
                }
                TreeItemType::InputsFolder { node_id } => {
                    // Toggle all inputs - updates parent node state
                    let node_id = node_id.clone();
                    if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                        let any_enabled = node.ports.iter().filter(|p| p.is_input).any(|p| p.enabled);
                        let new_state = !any_enabled;
                        for port in node.ports.iter_mut().filter(|p| p.is_input) {
                            port.enabled = new_state;
                        }
                        log!("Toggled all inputs for {} to enabled={}", node_id, new_state);
                        // Emit action to notify app that ports changed
                        cx.widget_action(self.widget_uid(), &scope.path,
                            DataflowTreeAction::PortEnabledChanged {
                                node_id: node_id.clone(),
                                port_id: "inputs".to_string(),  // Special marker for batch update
                                enabled: new_state
                            });
                    }
                    // Update node enabled state based on children
                    self.update_node_state_from_children(&node_id);
                    // Rebuild tree data synchronously to keep FileTree cache in sync
                    self.build_file_tree_data();
                    self.file_tree.redraw(cx);
                }
                TreeItemType::OutputsFolder { node_id } => {
                    // Toggle all outputs - updates parent node state
                    let node_id = node_id.clone();
                    if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
                        let any_enabled = node.ports.iter().filter(|p| !p.is_input).any(|p| p.enabled);
                        let new_state = !any_enabled;
                        for port in node.ports.iter_mut().filter(|p| !p.is_input) {
                            port.enabled = new_state;
                        }
                        log!("Toggled all outputs for {} to enabled={}", node_id, new_state);
                        // Emit action to notify app that ports changed
                        cx.widget_action(self.widget_uid(), &scope.path,
                            DataflowTreeAction::PortEnabledChanged {
                                node_id: node_id.clone(),
                                port_id: "outputs".to_string(),  // Special marker for batch update
                                enabled: new_state
                            });
                    }
                    // Update node enabled state based on children
                    self.update_node_state_from_children(&node_id);
                    // Rebuild tree data synchronously to keep FileTree cache in sync
                    self.build_file_tree_data();
                    self.file_tree.redraw(cx);
                }
                TreeItemType::Root => {
                    // Toggle root - toggle all nodes
                    let any_enabled = self.nodes.iter().any(|n| n.enabled);
                    let new_state = !any_enabled;
                    for node in &mut self.nodes {
                        node.enabled = new_state;
                        for port in &mut node.ports {
                            port.enabled = new_state;
                        }
                    }
                    log!("Toggled all nodes to enabled={}", new_state);
                    // Emit action to notify app that everything changed
                    cx.widget_action(self.widget_uid(), &scope.path,
                        DataflowTreeAction::NodeEnabledChanged {
                            node_id: "__root__".to_string(),
                            enabled: new_state
                        });
                    // Rebuild tree data synchronously to keep FileTree cache in sync
                    self.build_file_tree_data();
                    self.file_tree.redraw(cx);
                }
            }
        } else {
            log!("No file_node found for {:?}", file_id);
        }
    }

    /// Update a node's enabled state based on its children's states
    fn update_node_state_from_children(&mut self, node_id: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            if node.ports.is_empty() {
                // No ports, keep node state as is
                return;
            }
            let all_enabled = node.ports.iter().all(|p| p.enabled);
            let any_enabled = node.ports.iter().any(|p| p.enabled);
            // Node is enabled if all children are enabled
            // Node shows partial (◐) if some but not all are enabled
            // For the enabled flag, we'll use: enabled if ANY child is enabled
            node.enabled = any_enabled;
            log!("Updated node {} state: enabled={}, all_enabled={}, any_enabled={}",
                 node_id, node.enabled, all_enabled, any_enabled);
        }
    }

    /// Toggle all ports matching the current search filter
    fn toggle_matching_ports(&mut self, cx: &mut Cx, scope: &mut Scope) {
        if self.search_filter.is_empty() {
            return;
        }

        let search_lower = self.search_filter.to_lowercase();

        // First, check if any matching port is currently enabled
        // If any are enabled, we'll disable all. If all are disabled, we'll enable all.
        let any_matching_enabled = self.nodes.iter().any(|node| {
            node.ports.iter().any(|port| {
                let matches = port.id.to_lowercase().contains(&search_lower)
                    || port.label.to_lowercase().contains(&search_lower);
                matches && port.enabled
            })
        });

        let new_state = !any_matching_enabled;
        let mut affected_node_ids: Vec<String> = Vec::new();
        let mut total_toggled = 0;

        // Toggle all matching ports
        for node in &mut self.nodes {
            let mut node_affected = false;
            for port in &mut node.ports {
                let matches = port.id.to_lowercase().contains(&search_lower)
                    || port.label.to_lowercase().contains(&search_lower);
                if matches {
                    port.enabled = new_state;
                    node_affected = true;
                    total_toggled += 1;
                }
            }
            if node_affected {
                affected_node_ids.push(node.id.clone());
            }
        }

        log!("Toggled {} ports matching '{}' to enabled={}", total_toggled, self.search_filter, new_state);

        // Update node states based on their children
        for node_id in &affected_node_ids {
            self.update_node_state_from_children(node_id);
        }

        // Emit action to notify app
        cx.widget_action(self.widget_uid(), &scope.path,
            DataflowTreeAction::PortEnabledChanged {
                node_id: "__batch__".to_string(),
                port_id: self.search_filter.clone(),
                enabled: new_state
            });

        // Rebuild tree and redraw
        self.build_file_tree_data();
        self.file_tree.redraw(cx);
    }

    /// Update display names in-place without rebuilding the tree structure
    /// This is safer than a full rebuild as it doesn't call forget()
    fn update_display_names(&mut self) {
        // Update all file_node names based on current enabled states
        for (_live_id, file_node) in self.file_nodes.iter_mut() {
            match &file_node.item_type {
                TreeItemType::Root => {
                    // Root node name stays "Dataflow"
                }
                TreeItemType::Node { node_id } => {
                    if let Some(node) = self.nodes.iter().find(|n| n.id == *node_id) {
                        let all_enabled = node.ports.iter().all(|p| p.enabled);
                        let any_enabled = node.ports.iter().any(|p| p.enabled);
                        let status = if node.enabled && all_enabled {
                            "●"
                        } else if any_enabled {
                            "◐"
                        } else {
                            "○"
                        };
                        file_node.name = format!("{} {}", status, node.id);
                    }
                }
                TreeItemType::InputsFolder { node_id } => {
                    if let Some(node) = self.nodes.iter().find(|n| n.id == *node_id) {
                        let inputs: Vec<_> = node.ports.iter().filter(|p| p.is_input).collect();
                        let all_enabled = inputs.iter().all(|p| p.enabled);
                        let any_enabled = inputs.iter().any(|p| p.enabled);
                        let status = if all_enabled {
                            "●"
                        } else if any_enabled {
                            "◐"
                        } else {
                            "○"
                        };
                        file_node.name = format!("{} inputs", status);
                    }
                }
                TreeItemType::OutputsFolder { node_id } => {
                    if let Some(node) = self.nodes.iter().find(|n| n.id == *node_id) {
                        let outputs: Vec<_> = node.ports.iter().filter(|p| !p.is_input).collect();
                        let all_enabled = outputs.iter().all(|p| p.enabled);
                        let any_enabled = outputs.iter().any(|p| p.enabled);
                        let status = if all_enabled {
                            "●"
                        } else if any_enabled {
                            "◐"
                        } else {
                            "○"
                        };
                        file_node.name = format!("{} outputs", status);
                    }
                }
                TreeItemType::Port { node_id, port_id, is_input } => {
                    if let Some(node) = self.nodes.iter().find(|n| n.id == *node_id) {
                        if let Some(port) = node.ports.iter().find(|p| p.id == *port_id && p.is_input == *is_input) {
                            let status = if port.enabled { "●" } else { "○" };
                            file_node.name = format!("{} {}", status, port.id);
                        }
                    }
                }
            }
        }
    }

    fn draw_file_node(
        cx: &mut Cx2d,
        file_node_id: LiveId,
        file_tree: &mut FileTree,
        file_nodes: &LiveIdMap<LiveId, FileNode>,
    ) {
        if let Some(file_node) = file_nodes.get(&file_node_id) {
            match &file_node.child_edges {
                Some(child_edges) => {
                    if file_tree.begin_folder(cx, file_node_id, &file_node.name).is_ok() {
                        for child_edge in child_edges {
                            Self::draw_file_node(cx, child_edge.file_node_id, file_tree, file_nodes);
                        }
                        file_tree.end_folder();
                    }
                }
                None => {
                    file_tree.file(cx, file_node_id, &file_node.name);
                }
            }
        }
    }

    fn build_file_tree_data(&mut self) {
        // Note: Don't call forget() here as it clears FileTree's internal cache
        // which causes crashes if called during event handling.
        // The FileTree will pick up new names during draw_file_node calls.

        self.file_nodes.clear();
        self.node_id_to_live_id.clear();
        // Don't clear folder_open_states - preserve across rebuilds
        // Only initialize states for new folders
        self.live_id_counter = 1000;

        // Build child edges for root
        let mut root_edges = Vec::new();

        for node in &self.nodes {
            // Apply filters
            if !self.matches_filter(node) {
                continue;
            }

            // Generate unique LiveId for this node
            let node_live_id = LiveId(self.live_id_counter);
            self.live_id_counter += 1;
            self.node_id_to_live_id.insert(node.id.clone(), node_live_id);

            // Build port edges
            let mut port_edges = Vec::new();

            // Input ports
            let inputs: Vec<_> = node.ports.iter().filter(|p| p.is_input).collect();
            if !inputs.is_empty() {
                let inputs_live_id = LiveId(self.live_id_counter);
                self.live_id_counter += 1;

                let mut input_edges = Vec::new();
                for port in &inputs {
                    let port_live_id = LiveId(self.live_id_counter);
                    self.live_id_counter += 1;

                    let port_status = if port.enabled { "●" } else { "○" };
                    self.file_nodes.insert(port_live_id, FileNode {
                        name: format!("{} -> {}", port_status, port.label),
                        child_edges: None,
                        item_type: TreeItemType::Port {
                            node_id: node.id.clone(),
                            port_id: port.id.clone(),
                            is_input: true,
                        },
                    });

                    input_edges.push(FileEdge {
                        name: port.label.clone(),
                        file_node_id: port_live_id,
                    });
                }

                let inputs_enabled = inputs.iter().filter(|p| p.enabled).count();
                let inputs_status = if inputs_enabled == inputs.len() { "●" } else if inputs_enabled > 0 { "◐" } else { "○" };
                self.file_nodes.insert(inputs_live_id, FileNode {
                    name: format!("{} Inputs ({}/{})", inputs_status, inputs_enabled, inputs.len()),
                    child_edges: Some(input_edges),
                    item_type: TreeItemType::InputsFolder { node_id: node.id.clone() },
                });
                // Track folder state (initially closed, preserve if already tracked)
                self.folder_open_states.entry(inputs_live_id).or_insert(false);

                port_edges.push(FileEdge {
                    name: "Inputs".to_string(),
                    file_node_id: inputs_live_id,
                });
            }

            // Output ports
            let outputs: Vec<_> = node.ports.iter().filter(|p| !p.is_input).collect();
            if !outputs.is_empty() {
                let outputs_live_id = LiveId(self.live_id_counter);
                self.live_id_counter += 1;

                let mut output_edges = Vec::new();
                for port in &outputs {
                    let port_live_id = LiveId(self.live_id_counter);
                    self.live_id_counter += 1;

                    let port_status = if port.enabled { "●" } else { "○" };
                    self.file_nodes.insert(port_live_id, FileNode {
                        name: format!("{} <- {}", port_status, port.label),
                        child_edges: None,
                        item_type: TreeItemType::Port {
                            node_id: node.id.clone(),
                            port_id: port.id.clone(),
                            is_input: false,
                        },
                    });

                    output_edges.push(FileEdge {
                        name: port.label.clone(),
                        file_node_id: port_live_id,
                    });
                }

                let outputs_enabled = outputs.iter().filter(|p| p.enabled).count();
                let outputs_status = if outputs_enabled == outputs.len() { "●" } else if outputs_enabled > 0 { "◐" } else { "○" };
                self.file_nodes.insert(outputs_live_id, FileNode {
                    name: format!("{} Outputs ({}/{})", outputs_status, outputs_enabled, outputs.len()),
                    child_edges: Some(output_edges),
                    item_type: TreeItemType::OutputsFolder { node_id: node.id.clone() },
                });
                // Track folder state (initially closed, preserve if already tracked)
                self.folder_open_states.entry(outputs_live_id).or_insert(false);

                port_edges.push(FileEdge {
                    name: "Outputs".to_string(),
                    file_node_id: outputs_live_id,
                });
            }

            // Node as folder with ports as children
            // Show ● if all ports enabled, ○ if none, ◐ if partial
            let status = if node.ports.is_empty() {
                if node.enabled { "●" } else { "○" }
            } else {
                let all_enabled = node.ports.iter().all(|p| p.enabled);
                let any_enabled = node.ports.iter().any(|p| p.enabled);
                if all_enabled { "●" } else if any_enabled { "◐" } else { "○" }
            };
            let category_prefix = match node.category {
                NodeCategory::MaaS => "[MaaS]",
                NodeCategory::TTS => "[TTS]",
                NodeCategory::Bridge => "[Bridge]",
                NodeCategory::Controller => "[Ctrl]",
                NodeCategory::MoFA => "[MoFA]",
                NodeCategory::Segmenter => "[Seg]",
                NodeCategory::Default => "",
            };

            let node_name = if category_prefix.is_empty() {
                format!("{} {}", status, node.label)
            } else {
                format!("{} {} {}", status, category_prefix, node.label)
            };

            if port_edges.is_empty() {
                // Node with no ports - show as file
                self.file_nodes.insert(node_live_id, FileNode {
                    name: node_name,
                    child_edges: None,
                    item_type: TreeItemType::Node { node_id: node.id.clone() },
                });
            } else {
                // Node with ports - show as folder
                self.file_nodes.insert(node_live_id, FileNode {
                    name: node_name,
                    child_edges: Some(port_edges),
                    item_type: TreeItemType::Node { node_id: node.id.clone() },
                });
                // Track folder state (initially closed, preserve if already tracked)
                self.folder_open_states.entry(node_live_id).or_insert(false);
            }

            root_edges.push(FileEdge {
                name: node.label.clone(),
                file_node_id: node_live_id,
            });
        }

        // Create root node with status based on all nodes
        let root_id = live_id!(dataflow_root);
        let all_nodes_enabled = self.nodes.iter().all(|n| n.enabled && n.ports.iter().all(|p| p.enabled));
        let any_nodes_enabled = self.nodes.iter().any(|n| n.enabled || n.ports.iter().any(|p| p.enabled));
        let root_status = if all_nodes_enabled { "●" } else if any_nodes_enabled { "◐" } else { "○" };
        self.file_nodes.insert(root_id, FileNode {
            name: format!("{} Dataflow ({} nodes)", root_status, root_edges.len()),
            child_edges: Some(root_edges),
            item_type: TreeItemType::Root,
        });
        // Root starts open (preserve if already tracked)
        self.folder_open_states.entry(root_id).or_insert(true);
    }

    fn matches_filter(&self, node: &TreeNode) -> bool {
        // Search filter
        if !self.search_filter.is_empty() {
            let search_lower = self.search_filter.to_lowercase();

            // Check node name/id
            let node_matches = node.label.to_lowercase().contains(&search_lower)
                || node.id.to_lowercase().contains(&search_lower);

            // Check all ports (inputs and outputs)
            let port_matches = node.ports.iter().any(|port| {
                port.id.to_lowercase().contains(&search_lower)
                    || port.label.to_lowercase().contains(&search_lower)
            });

            if !node_matches && !port_matches {
                return false;
            }
        }

        // Category filter
        if let Some(cat) = self.category_filter {
            if node.category != cat {
                return false;
            }
        }

        true
    }

    pub fn set_nodes(&mut self, cx: &mut Cx, nodes: Vec<TreeNode>) {
        self.nodes = nodes;
        self.initialized = false;
        self.file_tree.redraw(cx);
    }

    pub fn set_search_filter(&mut self, cx: &mut Cx, filter: String) {
        self.search_filter = filter;
        self.initialized = false;
        self.file_tree.redraw(cx);
    }

    pub fn set_category_filter(&mut self, cx: &mut Cx, category: Option<NodeCategory>) {
        self.category_filter = category;
        self.initialized = false;
        self.file_tree.redraw(cx);
    }

    pub fn expand_all(&mut self, cx: &mut Cx) {
        for (live_id, node) in self.file_nodes.iter() {
            if node.child_edges.is_some() {
                self.file_tree.set_folder_is_open(cx, (*live_id).into(), true, Animate::No);
            }
        }
        self.file_tree.redraw(cx);
    }

    pub fn collapse_all(&mut self, cx: &mut Cx) {
        for (live_id, node) in self.file_nodes.iter() {
            if node.child_edges.is_some() && *live_id != live_id!(dataflow_root) {
                self.file_tree.set_folder_is_open(cx, (*live_id).into(), false, Animate::No);
            }
        }
        self.file_tree.redraw(cx);
    }

    pub fn update_node_enabled(&mut self, cx: &mut Cx, node_id: &str, enabled: bool) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == node_id) {
            node.enabled = enabled;
            self.initialized = false;
            self.file_tree.redraw(cx);
        }
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.iter().filter(|n| self.matches_filter(n)).count()
    }

    /// Get enabled states for all ports
    /// Returns a map of (node_id, port_id, is_input) -> enabled
    pub fn get_port_enabled_states(&self) -> HashMap<(String, String, bool), bool> {
        let mut states = HashMap::new();
        for node in &self.nodes {
            for port in &node.ports {
                states.insert(
                    (node.id.clone(), port.id.clone(), port.is_input),
                    port.enabled
                );
            }
        }
        states
    }

    /// Get enabled state for all nodes
    pub fn get_node_enabled_states(&self) -> HashMap<String, bool> {
        self.nodes.iter()
            .map(|n| (n.id.clone(), n.enabled))
            .collect()
    }
}

// Widget reference extension for easier access from App
impl DataflowTreeRef {
    pub fn set_nodes(&self, cx: &mut Cx, nodes: Vec<TreeNode>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_nodes(cx, nodes);
        }
    }

    pub fn set_search_filter(&self, cx: &mut Cx, filter: String) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_search_filter(cx, filter);
        }
    }

    pub fn set_category_filter(&self, cx: &mut Cx, category: Option<NodeCategory>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_category_filter(cx, category);
        }
    }

    pub fn expand_all(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.expand_all(cx);
        }
    }

    pub fn collapse_all(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.collapse_all(cx);
        }
    }

    pub fn update_node_enabled(&self, cx: &mut Cx, node_id: &str, enabled: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_node_enabled(cx, node_id, enabled);
        }
    }

    pub fn get_node_count(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.get_node_count()
        } else {
            0
        }
    }

    /// Check if a node's enabled state changed
    pub fn node_enabled_changed(&self, actions: &Actions) -> Option<(String, bool)> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let DataflowTreeAction::NodeEnabledChanged { node_id, enabled } = item.cast() {
                return Some((node_id, enabled));
            }
        }
        None
    }

    /// Check if a port's enabled state changed
    pub fn port_enabled_changed(&self, actions: &Actions) -> Option<(String, String, bool)> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let DataflowTreeAction::PortEnabledChanged { node_id, port_id, enabled } = item.cast() {
                return Some((node_id, port_id, enabled));
            }
        }
        None
    }

    /// Get enabled states for all ports
    pub fn get_port_enabled_states(&self) -> HashMap<(String, String, bool), bool> {
        if let Some(inner) = self.borrow() {
            inner.get_port_enabled_states()
        } else {
            HashMap::new()
        }
    }

    /// Get enabled states for all nodes
    pub fn get_node_enabled_states(&self) -> HashMap<String, bool> {
        if let Some(inner) = self.borrow() {
            inner.get_node_enabled_states()
        } else {
            HashMap::new()
        }
    }

    /// Toggle all ports matching the search filter (called from App)
    /// Returns true if any ports were toggled
    pub fn toggle_matching_ports_from_app(&self, cx: &mut Cx) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            if inner.search_filter.is_empty() {
                return false;
            }

            let search_lower = inner.search_filter.to_lowercase();

            // Check if any matching port is currently enabled
            let any_matching_enabled = inner.nodes.iter().any(|node| {
                node.ports.iter().any(|port| {
                    let matches = port.id.to_lowercase().contains(&search_lower)
                        || port.label.to_lowercase().contains(&search_lower);
                    matches && port.enabled
                })
            });

            let new_state = !any_matching_enabled;
            let mut affected_node_ids: Vec<String> = Vec::new();
            let mut total_toggled = 0;

            // Toggle all matching ports
            for node in &mut inner.nodes {
                let mut node_affected = false;
                for port in &mut node.ports {
                    let matches = port.id.to_lowercase().contains(&search_lower)
                        || port.label.to_lowercase().contains(&search_lower);
                    if matches {
                        port.enabled = new_state;
                        node_affected = true;
                        total_toggled += 1;
                    }
                }
                if node_affected {
                    affected_node_ids.push(node.id.clone());
                }
            }

            if total_toggled == 0 {
                return false;
            }

            log!("Toggled {} ports matching '{}' to enabled={}", total_toggled, inner.search_filter, new_state);

            // Update node states based on their children
            for node_id in &affected_node_ids {
                // Inline the update logic since we can't call methods on borrowed inner
                if let Some(node) = inner.nodes.iter_mut().find(|n| n.id == *node_id) {
                    if !node.ports.is_empty() {
                        let any_enabled = node.ports.iter().any(|p| p.enabled);
                        node.enabled = any_enabled;
                    }
                }
            }

            // Rebuild tree and redraw
            inner.build_file_tree_data();
            inner.file_tree.redraw(cx);

            return true;
        }
        false
    }
}

// Action helper for the combined panel
impl DataflowTreeHeaderRef {
    pub fn search_changed(&self, actions: &Actions) -> Option<String> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let DataflowTreeAction::SearchChanged { text } = item.cast() {
                return Some(text);
            }
        }
        None
    }

    pub fn filter_category(&self, actions: &Actions) -> Option<Option<NodeCategory>> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let DataflowTreeAction::FilterCategory { category } = item.cast() {
                return Some(category);
            }
        }
        None
    }

    pub fn expand_all_clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), DataflowTreeAction::ExpandAll)
        } else {
            false
        }
    }

    pub fn collapse_all_clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), DataflowTreeAction::CollapseAll)
        } else {
            false
        }
    }

    pub fn toggle_match_clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), DataflowTreeAction::ToggleMatchingPorts)
        } else {
            false
        }
    }
}

impl DataflowTreeFooterRef {
    pub fn enable_all_clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), DataflowTreeAction::EnableAllNodes)
        } else {
            false
        }
    }

    pub fn disable_all_clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), DataflowTreeAction::DisableAllNodes)
        } else {
            false
        }
    }

    pub fn toggle_matching_clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), DataflowTreeAction::ToggleMatchingPorts)
        } else {
            false
        }
    }
}
