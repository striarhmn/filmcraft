use serde::{Deserialize, Serialize};

/// Where wheel zoom keeps its pivot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ZoomAnchor {
    #[default]
    Cursor,
    Playhead,
}

/// Mouse navigation preferences shared by the Timeline, monitors, and Effect Controls.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NavigationPrefs {
    pub mouse_scrolling: String,
    pub alt_scrolling: String,
    pub shift_scrolling: String,
    pub ctrl_scrolling: String,
    pub zoom_anchor: ZoomAnchor,
    pub click_to_move_playhead: String,
    pub drag_playhead_line: String,
    pub ruler_drag_in_out: bool,
    pub program_wheel: String,
    pub program_alt_wheel: String,
    pub program_shift_wheel: String,
    pub program_ctrl_wheel: String,
    pub program_preview_wheel: String,
    pub program_preview_alt_wheel: String,
    pub program_preview_shift_wheel: String,
    pub program_preview_ctrl_wheel: String,
    pub source_wheel: String,
    pub source_alt_wheel: String,
    pub source_shift_wheel: String,
    pub source_ctrl_wheel: String,
    pub source_preview_wheel: String,
    pub source_preview_alt_wheel: String,
    pub source_preview_shift_wheel: String,
    pub source_preview_ctrl_wheel: String,
    pub effect_controls_wheel: String,
    pub effect_controls_alt_wheel: String,
    pub effect_controls_shift_wheel: String,
    pub effect_controls_ctrl_wheel: String,
    pub middle_button_pan: bool,
}

impl Default for NavigationPrefs {
    fn default() -> Self {
        Self {
            mouse_scrolling: "vertical".into(),
            alt_scrolling: "zoom".into(),
            shift_scrolling: "horizontal".into(),
            ctrl_scrolling: "horizontal".into(),
            zoom_anchor: ZoomAnchor::Cursor,
            click_to_move_playhead: "rulerOnly".into(),
            drag_playhead_line: "emptySpace".into(),
            ruler_drag_in_out: false,
            program_wheel: "stepFrames".into(),
            program_alt_wheel: "zoom".into(),
            program_shift_wheel: "horizontal".into(),
            program_ctrl_wheel: "horizontal".into(),
            program_preview_wheel: "zoom".into(),
            program_preview_alt_wheel: "zoom".into(),
            program_preview_shift_wheel: "zoom".into(),
            program_preview_ctrl_wheel: "zoom".into(),
            source_wheel: "stepFrames".into(),
            source_alt_wheel: "zoom".into(),
            source_shift_wheel: "horizontal".into(),
            source_ctrl_wheel: "horizontal".into(),
            source_preview_wheel: "zoom".into(),
            source_preview_alt_wheel: "zoom".into(),
            source_preview_shift_wheel: "zoom".into(),
            source_preview_ctrl_wheel: "zoom".into(),
            effect_controls_wheel: "stepFrames".into(),
            effect_controls_alt_wheel: "zoom".into(),
            effect_controls_shift_wheel: "horizontal".into(),
            effect_controls_ctrl_wheel: "horizontal".into(),
            middle_button_pan: true,
        }
    }
}

pub const MOUSE_SCROLL: &[(&str, &str)] = &[
    ("vertical", "Vertical"),
    ("horizontal", "Horizontal"),
    ("zoom", "Zoom"),
    ("step", "Step Playhead (Frames)"),
    ("off", "Disabled"),
];
pub const PANEL_WHEEL: &[(&str, &str)] = &[
    ("horizontal", "Horizontal"),
    ("zoom", "Zoom"),
    ("stepFrames", "Step Frames"),
    ("stepKeyframes", "Step Keyframes"),
    ("off", "Disabled"),
];
pub const NAVIGATION_STYLES: &[(&str, &str)] = &[("default", "Default"), ("alternative", "Alternative")];

impl NavigationPrefs {
    pub fn wheel_action(&self, ctrl: bool, alt: bool, shift: bool) -> &str {
        let mouse = valid(&self.mouse_scrolling, "vertical", MOUSE_SCROLL);
        if ctrl && (shift || alt) {
            "off"
        } else if ctrl {
            let action = valid(&self.ctrl_scrolling, "horizontal", MOUSE_SCROLL);
            if action == "horizontal" && mouse == "horizontal" { "vertical" } else { action }
        } else if alt {
            valid(&self.alt_scrolling, "zoom", MOUSE_SCROLL)
        } else if shift {
            valid(&self.shift_scrolling, "horizontal", MOUSE_SCROLL)
        } else {
            valid(&self.mouse_scrolling, "vertical", MOUSE_SCROLL)
        }
    }

    pub fn panel_wheel(&self, panel: &str, preview: bool, ctrl: bool, alt: bool, shift: bool) -> &str {
        if ctrl && (shift || alt) {
            return "off";
        }
        let (bare, a, s, c) = match (panel, preview) {
            ("program", false) => (&self.program_wheel, &self.program_alt_wheel, &self.program_shift_wheel, &self.program_ctrl_wheel),
            ("program", true) => (
                &self.program_preview_wheel,
                &self.program_preview_alt_wheel,
                &self.program_preview_shift_wheel,
                &self.program_preview_ctrl_wheel,
            ),
            ("source", false) => (&self.source_wheel, &self.source_alt_wheel, &self.source_shift_wheel, &self.source_ctrl_wheel),
            ("source", true) => (
                &self.source_preview_wheel,
                &self.source_preview_alt_wheel,
                &self.source_preview_shift_wheel,
                &self.source_preview_ctrl_wheel,
            ),
            _ => (
                &self.effect_controls_wheel,
                &self.effect_controls_alt_wheel,
                &self.effect_controls_shift_wheel,
                &self.effect_controls_ctrl_wheel,
            ),
        };
        let value = if ctrl { c } else if alt { a } else if shift { s } else { bare };
        let fallback = if preview && panel != "effectControls" { "zoom" } else if ctrl || shift { "horizontal" } else if alt { "zoom" } else { "stepFrames" };
        valid(value, fallback, PANEL_WHEEL)
    }

    pub fn style(&self) -> &'static str {
        if self == &Self::default() {
            "default"
        } else if navigation_style_preset("alternative").as_ref() == Some(self) {
            "alternative"
        } else {
            "custom"
        }
    }
}

fn valid<'a>(value: &'a str, fallback: &'a str, choices: &[(&str, &str)]) -> &'a str {
    if choices.iter().any(|(id, _)| *id == value) { value } else { fallback }
}

pub fn navigation_style_preset(id: &str) -> Option<NavigationPrefs> {
    match id {
        "default" => Some(NavigationPrefs::default()),
        "alternative" => Some(NavigationPrefs {
            mouse_scrolling: "zoom".into(),
            alt_scrolling: "zoom".into(),
            shift_scrolling: "vertical".into(),
            ctrl_scrolling: "horizontal".into(),
            click_to_move_playhead: "anywhere".into(),
            drag_playhead_line: "anywhere".into(),
            ruler_drag_in_out: true,
            program_shift_wheel: "stepFrames".into(),
            program_ctrl_wheel: "horizontal".into(),
            program_preview_shift_wheel: "stepFrames".into(),
            program_preview_ctrl_wheel: "horizontal".into(),
            source_shift_wheel: "stepFrames".into(),
            source_ctrl_wheel: "horizontal".into(),
            source_preview_shift_wheel: "stepFrames".into(),
            source_preview_ctrl_wheel: "horizontal".into(),
            effect_controls_wheel: "zoom".into(),
            effect_controls_alt_wheel: "horizontal".into(),
            effect_controls_shift_wheel: "stepFrames".into(),
            effect_controls_ctrl_wheel: "stepKeyframes".into(),
            ..NavigationPrefs::default()
        }),
        _ => None,
    }
}
