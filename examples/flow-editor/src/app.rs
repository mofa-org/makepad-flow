use makepad_flow::*;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_flow::flow_canvas::*;

    // Main application
    App = {{App}} {
        ui: <Window> {
            window: { title: "Flow Editor", inner_size: vec2(1200, 800) }
            show_bg: true
            draw_bg: { color: #1a1a2e }

            body = <View> {
                width: Fill, height: Fill, flow: Down

                // Toolbar
                <View> {
                    width: Fill, height: 48
                    padding: { left: 12, right: 12 }, spacing: 8, align: { y: 0.5 }
                    show_bg: true, draw_bg: { color: #252538 }

                    <Label> { draw_text: { text_style: { font_size: 14.0 }, color: #e0e0e0 }, text: "Flow Editor" }
                    <View> { width: 20, height: 1 }

                    <Label> { draw_text: { text_style: { font_size: 10.0 }, color: #a0a0b0 }, text: "Style:" }
                    line_style_dropdown = <DropDown> {
                        width: 90, height: 28
                        draw_bg: { color: #3d3d5c }
                        labels: ["Solid", "Dashed", "Dotted"]
                        values: [solid, dashed, dotted]
                    }

                    <Label> { draw_text: { text_style: { font_size: 10.0 }, color: #a0a0b0 }, text: "Width:" }
                    line_width_dropdown = <DropDown> {
                        width: 70, height: 28
                        draw_bg: { color: #3d3d5c }
                        labels: ["1px", "2px", "3px", "4px"]
                        values: [w1, w2, w3, w4]
                    }

                    <View> { width: Fill, height: 1 }
                    add_node_btn = <Button> {
                        width: Fit, height: 32, padding: { left: 12, right: 12 }
                        draw_bg: { color: #3d3d5c }
                        text: "+ Add Node"
                    }
                    delete_btn = <Button> {
                        width: Fit, height: 32, padding: { left: 12, right: 12 }
                        draw_bg: { color: #5c3d3d }
                        text: "Delete"
                    }
                    fit_view_btn = <Button> {
                        width: Fit, height: 32, padding: { left: 12, right: 12 }
                        draw_bg: { color: #3d3d5c }
                        text: "Fit View"
                    }
                    clear_btn = <Button> {
                        width: Fit, height: 32, padding: { left: 12, right: 12 }
                        draw_bg: { color: #3d3d5c }
                        text: "Clear"
                    }
                }

                // Canvas area
                canvas = <FlowCanvas> {}

                // Status bar
                status_bar = <View> {
                    width: Fill, height: 28
                    padding: { left: 12, right: 12 }, align: { y: 0.5 }
                    show_bg: true, draw_bg: { color: #252538 }
                    status_label = <Label> {
                        draw_text: { text_style: { font_size: 10.0 }, color: #8080a0 },
                        text: "Shift+Click: multi-select | Drag: select box | Ctrl+A/Z/Y | Ctrl+Click: menu"
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

// App
#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_flow::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle button clicks - send commands as widget actions
        if self.ui.button(id!(add_node_btn)).clicked(actions) {
            cx.action(FlowCanvasCommand::AddNode);
        }

        if self.ui.button(id!(delete_btn)).clicked(actions) {
            cx.action(FlowCanvasCommand::Delete);
        }

        if self.ui.button(id!(fit_view_btn)).clicked(actions) {
            cx.action(FlowCanvasCommand::FitView);
        }

        if self.ui.button(id!(clear_btn)).clicked(actions) {
            cx.action(FlowCanvasCommand::Clear);
        }

        // Handle line style dropdown
        if let Some(index) = self.ui.drop_down(id!(line_style_dropdown)).changed(actions) {
            cx.action(FlowCanvasCommand::SetLineStyle(index as f32));
        }

        // Handle line width dropdown
        if let Some(index) = self.ui.drop_down(id!(line_width_dropdown)).changed(actions) {
            cx.action(FlowCanvasCommand::SetLineWidth((index + 1) as f32));
        }

        // Update status on canvas actions
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
