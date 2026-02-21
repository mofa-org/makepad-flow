#![allow(dead_code)]

use makepad_widgets::*;

pub fn register_live_design(cx: &mut Cx) {
    self::live_design(cx);
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Individual log entry
    LogEntryView = <View> {
        width: Fill, height: Fit
        flow: Right
        padding: { left: 8, right: 8, top: 4, bottom: 4 }
        spacing: 8
        align: { y: 0.0 }

        show_bg: true
        draw_bg: {
            color: #00000000
            instance hover: 0.0
            fn pixel(self) -> vec4 {
                return mix(self.color, #ffffff08, self.hover);
            }
        }

        timestamp = <Label> {
            width: 60, height: Fit
            draw_text: {
                color: #666680
                text_style: { font_size: 9.0 }
            }
        }

        level_badge = <View> {
            width: 40, height: 16
            align: { x: 0.5, y: 0.5 }
            show_bg: true
            draw_bg: {
                instance color: #3d5a3d
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(0., 0., self.rect_size.x, self.rect_size.y, 3.0);
                    sdf.fill(self.color);
                    return sdf.result;
                }
            }

            level_text = <Label> {
                width: Fit, height: Fit
                draw_text: {
                    color: #ffffff
                    text_style: { font_size: 8.0 }
                }
            }
        }

        node_id = <Label> {
            width: 100, height: Fit
            draw_text: {
                color: #8888aa
                text_style: { font_size: 9.0 }
            }
        }

        message = <Label> {
            width: Fill, height: Fit
            draw_text: {
                color: #cccccc
                text_style: { font_size: 9.0 }
                wrap: Word
            }
        }
    }

    pub LogPanel = {{LogPanel}} <View> {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: { color: #1a1a2e }

        // Header with filters
        header = <View> {
            width: Fill, height: Fit
            padding: 8
            flow: Down, spacing: 6

            // Filter row
            filter_row = <View> {
                width: Fill, height: Fit
                flow: Right, spacing: 4

                log_search = <TextInput> {
                    width: Fill, height: 24
                    empty_text: "Filter logs..."
                    draw_bg: {
                        color: #3d3d5c
                        border_radius: 4.0
                    }
                    draw_text: {
                        color: #ffffff
                        text_style: { font_size: 9.0 }
                    }
                }
            }

            // Level filter buttons
            level_filter_row = <View> {
                width: Fill, height: Fit
                flow: Right, spacing: 4

                <Label> {
                    width: Fit, height: Fit
                    draw_text: {
                        color: #888888
                        text_style: { font_size: 9.0 }
                    }
                    text: "Level:"
                }

                filter_all_levels = <Button> {
                    width: Fit, height: 20
                    padding: { left: 6, right: 6 }
                    draw_bg: { color: #4a4a6a, border_radius: 10.0 }
                    text: "All"
                    draw_text: { color: #ffffff, text_style: { font_size: 8.0 } }
                }

                filter_debug = <Button> {
                    width: Fit, height: 20
                    padding: { left: 6, right: 6 }
                    draw_bg: { color: #3d3d5c, border_radius: 10.0 }
                    text: "Debug"
                    draw_text: { color: #888888, text_style: { font_size: 8.0 } }
                }

                filter_info = <Button> {
                    width: Fit, height: 20
                    padding: { left: 6, right: 6 }
                    draw_bg: { color: #3d3d5c, border_radius: 10.0 }
                    text: "Info"
                    draw_text: { color: #88cc88, text_style: { font_size: 8.0 } }
                }

                filter_warn = <Button> {
                    width: Fit, height: 20
                    padding: { left: 6, right: 6 }
                    draw_bg: { color: #3d3d5c, border_radius: 10.0 }
                    text: "Warn"
                    draw_text: { color: #cccc88, text_style: { font_size: 8.0 } }
                }

                filter_error = <Button> {
                    width: Fit, height: 20
                    padding: { left: 6, right: 6 }
                    draw_bg: { color: #3d3d5c, border_radius: 10.0 }
                    text: "Error"
                    draw_text: { color: #cc8888, text_style: { font_size: 8.0 } }
                }
            }
        }

        // Scrollable log content
        log_scroll = <ScrollYView> {
            width: Fill, height: Fill

            log_content = <View> {
                width: Fill, height: Fit
                flow: Down
            }
        }

        // Footer with stats
        footer = <View> {
            width: Fill, height: Fit
            padding: 8
            flow: Right, spacing: 4

            entry_count = <Label> {
                width: Fill, height: Fit
                draw_text: {
                    color: #666680
                    text_style: { font_size: 9.0 }
                }
                text: "0 entries"
            }

            auto_scroll_toggle = <CheckBox> {
                width: Fit, height: Fit
                text: "Auto-scroll"
                draw_text: { text_style: { font_size: 9.0 } }
            }

            clear_btn = <Button> {
                width: Fit, height: 24
                padding: { left: 8, right: 8 }
                draw_bg: { color: #5a3d3d, border_radius: 4.0 }
                text: "Clear"
                draw_text: { color: #cc8888, text_style: { font_size: 9.0 } }
            }
        }
    }
}

// Log entry data structure
#[derive(Clone, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub node_id: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    pub fn color(&self) -> Vec4 {
        match self {
            LogLevel::Debug => vec4(0.5, 0.5, 0.5, 1.0),
            LogLevel::Info => vec4(0.33, 0.67, 0.33, 1.0),
            LogLevel::Warn => vec4(0.67, 0.67, 0.33, 1.0),
            LogLevel::Error => vec4(0.67, 0.33, 0.33, 1.0),
        }
    }
}

// Actions emitted by LogPanel
#[derive(Clone, Debug, DefaultNone)]
pub enum LogPanelAction {
    None,
    ClearLogs,
}

#[derive(Live, LiveHook, Widget)]
pub struct LogPanel {
    #[deref] view: View,

    #[rust] entries: Vec<LogEntry>,
    #[rust] search_filter: String,
    #[rust] level_filter: Option<LogLevel>,
    #[rust] auto_scroll: bool,
}

impl Widget for LogPanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for LogPanel {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // Handle Clear button
        if self.button(ids!(clear_btn)).clicked(actions) {
            cx.widget_action(self.widget_uid(), &scope.path, LogPanelAction::ClearLogs);
        }

        // Handle level filter buttons
        if self.button(ids!(filter_all_levels)).clicked(actions) {
            self.level_filter = None;
            self.update_entry_count(cx);
            self.redraw(cx);
        }
        if self.button(ids!(filter_debug)).clicked(actions) {
            self.level_filter = Some(LogLevel::Debug);
            self.update_entry_count(cx);
            self.redraw(cx);
        }
        if self.button(ids!(filter_info)).clicked(actions) {
            self.level_filter = Some(LogLevel::Info);
            self.update_entry_count(cx);
            self.redraw(cx);
        }
        if self.button(ids!(filter_warn)).clicked(actions) {
            self.level_filter = Some(LogLevel::Warn);
            self.update_entry_count(cx);
            self.redraw(cx);
        }
        if self.button(ids!(filter_error)).clicked(actions) {
            self.level_filter = Some(LogLevel::Error);
            self.update_entry_count(cx);
            self.redraw(cx);
        }

        // Handle search input
        if let Some(text) = self.text_input(ids!(log_search)).changed(actions) {
            self.search_filter = text;
            self.update_entry_count(cx);
            self.redraw(cx);
        }

        // Handle auto-scroll toggle
        if let Some(checked) = self.check_box(ids!(auto_scroll_toggle)).changed(actions) {
            self.auto_scroll = checked;
        }
    }
}

impl LogPanel {
    pub fn set_entries(&mut self, cx: &mut Cx, entries: Vec<LogEntry>) {
        self.entries = entries;
        self.update_entry_count(cx);
        self.redraw(cx);
    }

    pub fn add_entry(&mut self, cx: &mut Cx, entry: LogEntry) {
        self.entries.push(entry);
        self.update_entry_count(cx);
        self.redraw(cx);
    }

    pub fn clear_entries(&mut self, cx: &mut Cx) {
        self.entries.clear();
        self.update_entry_count(cx);
        self.redraw(cx);
    }

    fn update_entry_count(&mut self, cx: &mut Cx) {
        let filtered_count = self.filtered_entries().count();
        let total_count = self.entries.len();
        let text = if self.level_filter.is_some() || !self.search_filter.is_empty() {
            format!("{} / {} entries", filtered_count, total_count)
        } else {
            format!("{} entries", total_count)
        };
        self.label(ids!(entry_count)).set_text(cx, &text);
    }

    fn filtered_entries(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter().filter(|entry| {
            // Level filter
            if let Some(level) = self.level_filter {
                if entry.level != level {
                    return false;
                }
            }

            // Search filter
            if !self.search_filter.is_empty() {
                let search_lower = self.search_filter.to_lowercase();
                if !entry.message.to_lowercase().contains(&search_lower)
                    && !entry.node_id.to_lowercase().contains(&search_lower) {
                    return false;
                }
            }

            true
        })
    }
}

// Widget reference extension for easier access from App
impl LogPanelRef {
    pub fn set_entries(&self, cx: &mut Cx, entries: Vec<LogEntry>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_entries(cx, entries);
        }
    }

    pub fn add_entry(&self, cx: &mut Cx, entry: LogEntry) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_entry(cx, entry);
        }
    }

    pub fn clear_entries(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear_entries(cx);
        }
    }

    pub fn clear_clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            matches!(item.cast(), LogPanelAction::ClearLogs)
        } else {
            false
        }
    }
}
