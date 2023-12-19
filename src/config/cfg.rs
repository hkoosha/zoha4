use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use gdk4::{Monitor, RGBA};
use gdk4::prelude::MonitorExt;
use gdk4::traits::DisplayExt;
#[allow(unused_imports)] // IntelliJ goes bananas without this.
use glib::bitflags::Flags;
use glib::Cast;
use gtk4::{accelerator_parse, PositionType};
use pango::FontDescription;
use serde::Deserialize;
use thiserror::Error;
use vte4::CursorBlinkMode;

use crate::config::args::ZohaArgs;
use crate::config::color::Pallet;

// =============================================================================

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TabMode {
    Always,
    Never,
    Auto,
}

impl Display for TabMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TabMode::Always => write!(f, "Always"),
            TabMode::Never => write!(f, "Never"),
            TabMode::Auto => write!(f, "Auto"),
        }
    }
}


#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TabPosition {
    Left,
    Right,
    Top,
    Bottom,
}

impl Display for TabPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TabPosition::Left => write!(f, "Left"),
            TabPosition::Right => write!(f, "Right"),
            TabPosition::Top => write!(f, "Top"),
            TabPosition::Bottom => write!(f, "Bottom"),
        }
    }
}

impl TabPosition {
    pub fn to_gtk(&self) -> PositionType {
        match self {
            TabPosition::Left => PositionType::Left,
            TabPosition::Right => PositionType::Right,
            TabPosition::Top => PositionType::Top,
            TabPosition::Bottom => PositionType::Bottom,
        }
    }
}


#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum CursorShape {
    Block,
    IBeam,
    Underline,
}

impl Display for CursorShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorShape::Block => write!(f, "Block"),
            CursorShape::IBeam => write!(f, "IBeam"),
            CursorShape::Underline => write!(f, "Underline"),
        }
    }
}

impl CursorShape {
    pub fn to_vte(&self) -> vte4::CursorShape {
        match self {
            CursorShape::Block => vte4::CursorShape::Block,
            CursorShape::IBeam => vte4::CursorShape::Ibeam,
            CursorShape::Underline => vte4::CursorShape::Underline,
        }
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum EraseBinding {
    Auto,
    AsciiBackspace,
    AsciiDelete,
    DeleteSequence,
    Tty,
}

impl Display for EraseBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EraseBinding::Auto => write!(f, "Auto"),
            EraseBinding::AsciiBackspace => write!(f, "AsciiBackspace"),
            EraseBinding::AsciiDelete => write!(f, "AsciiDelete"),
            EraseBinding::DeleteSequence => write!(f, "DeleteSequence"),
            EraseBinding::Tty => write!(f, "Tty"),
        }
    }
}

impl EraseBinding {
    pub fn to_vte(&self) -> vte4::EraseBinding {
        match self {
            EraseBinding::Auto => vte4::EraseBinding::Auto,
            EraseBinding::AsciiBackspace => vte4::EraseBinding::AsciiBackspace,
            EraseBinding::AsciiDelete => vte4::EraseBinding::AsciiDelete,
            EraseBinding::DeleteSequence => vte4::EraseBinding::DeleteSequence,
            EraseBinding::Tty => vte4::EraseBinding::Tty,
        }
    }
}


#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScrollbarPosition {
    Left,
    Right,
    Hidden,
}

impl Display for ScrollbarPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScrollbarPosition::Left => write!(f, "Left"),
            ScrollbarPosition::Right => write!(f, "Right"),
            ScrollbarPosition::Hidden => write!(f, "Hidden"),
        }
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TerminalExitBehavior {
    // TODO: implement:
    // DropToDefaultShell,

    // TODO: implement:
    // RestartCommand,

    ExitTerminal,
}

impl Display for TerminalExitBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminalExitBehavior::ExitTerminal => write!(f, "ExitTerminal"),
        }
    }
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum LastTabExitBehavior {
    RestartTerminal,
    RestartTerminalAndHide,
    Exit,
}

impl Display for LastTabExitBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LastTabExitBehavior::RestartTerminal => write!(f, "RestartTerminal"),
            LastTabExitBehavior::RestartTerminalAndHide => write!(f, "RestartTerminalAndHide"),
            LastTabExitBehavior::Exit => write!(f, "Exit"),
        }
    }
}

// =============================================================================
// TODO combine Raw & Cfg once I learn how to deserialize external types.

#[derive(Deserialize, Debug, Default)]
struct RawCfgProcess {
    pub command: Option<String>,
    pub working_dir: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct RawCfgFont {
    font: Option<String>,
    size: Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
struct RawCfgColor {
    bg: Option<String>,
    fg: Option<String>,
    cursor: Option<String>,
    pallet: Option<Pallet>,
    color_00: Option<String>,
    color_01: Option<String>,
    color_02: Option<String>,
    color_03: Option<String>,
    color_04: Option<String>,
    color_05: Option<String>,
    color_06: Option<String>,
    color_07: Option<String>,
    color_08: Option<String>,
    color_09: Option<String>,
    color_10: Option<String>,
    color_11: Option<String>,
    color_12: Option<String>,
    color_13: Option<String>,
    color_14: Option<String>,
    color_15: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct RawCfgDisplay {
    monitor: Option<String>,
    title: Option<String>,
    x_pos: Option<u32>,
    y_pos: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
    width_percentage: Option<u8>,
    height_percentage: Option<u8>,
    start_hidden: Option<bool>,
    skip_task_bar: Option<bool>,
    always_on_top: Option<bool>,
    sticky: Option<bool>,
    fullscreen: Option<bool>,
    tab_scroll_wrap: Option<bool>,
    tab_mode: Option<TabMode>,
    tab_position: Option<TabPosition>,
    tab_expand: Option<bool>,
    tab_title_num_characters: Option<i8>,
    scrollbar_position: Option<ScrollbarPosition>,
}

#[derive(Deserialize, Debug, Default)]
struct RawCfgKey {
    copy: Option<String>,
    paste: Option<String>,
    quit: Option<String>,
    transparency_toggle: Option<String>,

    tab_add: Option<String>,
    tab_close: Option<String>,
    tab_move_backward: Option<String>,
    tab_move_forward: Option<String>,
    tab_goto_next: Option<String>,
    tab_goto_previous: Option<String>,
    tab_goto_last: Option<String>,
    tab_goto_01: Option<String>,
    tab_goto_02: Option<String>,
    tab_goto_03: Option<String>,
    tab_goto_04: Option<String>,
    tab_goto_05: Option<String>,
    tab_goto_06: Option<String>,
    tab_goto_07: Option<String>,
    tab_goto_08: Option<String>,

    font_size_inc: Option<String>,
    font_size_dec: Option<String>,
    font_size_reset: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct RawCfgTerminal {
    allow_hyper_link: Option<bool>,
    audible_bell: Option<bool>,
    cursor_blink: Option<bool>,
    cursor_shape: Option<CursorShape>,
    scroll_on_output: Option<bool>,
    scroll_on_keystroke: Option<bool>,
    mouse_auto_hide: Option<bool>,
    scrollback_lines: Option<i64>,
    backspace_binding: Option<EraseBinding>,
    delete_binding: Option<EraseBinding>,
    word_char_exceptions: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
struct RawCfgBehavior {
    terminal_exit_behavior: Option<TerminalExitBehavior>,
    last_tab_exit_behavior: Option<LastTabExitBehavior>,
    // prompt_on_exit: Option<bool>,
}

#[cfg(feature = "hack")]
#[derive(Deserialize, Debug, Default)]
struct RawCfgHack {
    toggle: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Default)]
struct RawCfg {
    #[serde(default)]
    font: RawCfgFont,
    #[serde(default)]
    color: RawCfgColor,
    #[serde(default)]
    display: RawCfgDisplay,
    #[serde(default)]
    keys: RawCfgKey,
    #[serde(default)]
    process: RawCfgProcess,
    #[serde(default)]
    terminal: RawCfgTerminal,
    #[serde(default)]
    behavior: RawCfgBehavior,
    #[serde(default)]
    #[cfg(feature = "hack")]
    hack: RawCfgHack,
}

// =============================================================================

#[derive(Deserialize, Debug, Default)]
pub struct CfgProcess {
    pub command: String,
    pub working_dir: Option<String>,
}

#[derive(Debug)]
pub struct CfgFont {
    pub font: FontDescription,
}

#[derive(Debug)]
pub struct CfgColor {
    pub bg: RGBA,
    pub fg: RGBA,
    pub cursor: RGBA,
    pub pallet: Pallet,
    pub color_00: Option<RGBA>,
    pub color_01: Option<RGBA>,
    pub color_02: Option<RGBA>,
    pub color_03: Option<RGBA>,
    pub color_04: Option<RGBA>,
    pub color_05: Option<RGBA>,
    pub color_06: Option<RGBA>,
    pub color_07: Option<RGBA>,
    pub color_08: Option<RGBA>,
    pub color_09: Option<RGBA>,
    pub color_10: Option<RGBA>,
    pub color_11: Option<RGBA>,
    pub color_12: Option<RGBA>,
    pub color_13: Option<RGBA>,
    pub color_14: Option<RGBA>,
    pub color_15: Option<RGBA>,
}

impl CfgColor {
    pub fn user_pallet(&self) -> Vec<RGBA> {
        let mut pallet_colors: Vec<RGBA> = self.pallet.colors();

        let mut pallet: Vec<RGBA> = vec![
            match self.color_15 {
                None => pallet_colors.pop().expect("missing color_15 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_14 {
                None => pallet_colors.pop().expect("missing color_14 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_13 {
                None => pallet_colors.pop().expect("missing color_13 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_12 {
                None => pallet_colors.pop().expect("missing color_12 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_11 {
                None => pallet_colors.pop().expect("missing color_11 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_10 {
                None => pallet_colors.pop().expect("missing color_10 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_09 {
                None => pallet_colors.pop().expect("missing color_09 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_08 {
                None => pallet_colors.pop().expect("missing color_08 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_07 {
                None => pallet_colors.pop().expect("missing color_07 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_06 {
                None => pallet_colors.pop().expect("missing color_06 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_05 {
                None => pallet_colors.pop().expect("missing color_05 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_04 {
                None => pallet_colors.pop().expect("missing color_04 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_03 {
                None => pallet_colors.pop().expect("missing color_03 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_02 {
                None => pallet_colors.pop().expect("missing color_02 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_01 {
                None => pallet_colors.pop().expect("missing color_01 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
            match self.color_00 {
                None => pallet_colors.pop().expect("missing color_00 in pallet"),
                Some(color) => {
                    pallet_colors.pop();
                    color
                }
            },
        ];

        pallet.reverse();

        return pallet;
    }
}

#[derive(Debug)]
pub struct CfgDisplay {
    pub monitor: Option<String>,
    pub title: String,
    pub x_pos: u32,
    pub y_pos: u32,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub width_percentage: Option<u8>,
    pub height_percentage: Option<u8>,
    pub start_hidden: bool,
    pub skip_task_bar: bool,
    pub always_on_top: bool,
    pub sticky: bool,
    pub fullscreen: bool,
    pub tab_scroll_wrap: bool,
    pub tab_mode: TabMode,
    pub tab_position: TabPosition,
    pub tab_expand: bool,
    pub tab_title_num_characters: Option<i8>,
    pub scrollbar_position: ScrollbarPosition,
}

impl CfgDisplay {
    pub fn get_monitor(&self) -> Monitor {
        let display = gdk4::Display::default().expect("could not get display");

        return match &self.monitor {
            None => display
                .monitors()
                .into_iter()
                .nth(0)
                .expect("no primary monitor present")
                .expect("no primary monitor present")
                .downcast::<Monitor>()
                .expect("could not cast to Monitor"),

            Some(index_or_model) => {
                // If user copies the output of `zoha --list-monitors` directly, it'll be
                // like `2 - DP-5` and we just want the first part.
                let cfg_model: String = if index_or_model.contains(" - ") {
                    index_or_model
                        .split(" - ")
                        .next()
                        .unwrap_or(index_or_model)
                        .to_string()
                } else {
                    index_or_model.to_string()
                };

                match cfg_model.parse::<u8>() {
                    Ok(index) => {
                        let mut i = 0;
                        for m in display.monitors().into_iter() {
                            let m = m.expect("failed to get monitor");
                            let m = m.downcast::<Monitor>().expect("failed to cast to Monitor");
                            if i == index {
                                return m;
                            }
                            i += 1;
                        }
                    }
                    Err(_) => {
                        for m in display.monitors().into_iter() {
                            let m = m.expect("failed to get monitor");
                            let m = m.downcast::<Monitor>().expect("failed to cast to Monitor");
                            if m.model().map(|it| it.to_string()).unwrap_or_else(|| "".to_string()) == cfg_model {
                                return m;
                            }
                        }
                    }
                }

                eprintln!("using primary monitor, \
                           configured monitor not found: {}", index_or_model);
                display
                    .monitors()
                    .into_iter()
                    .nth(0)
                    .expect("no primary monitor present")
                    .expect("no primary monitor present")
                    .downcast::<Monitor>()
                    .expect("could not cast to Monitor")
            }
        };
    }

    pub fn get_width(&self) -> u32 {
        let monitor: Monitor = self.get_monitor();
        let monitor_width: u32 = monitor.geometry().width().clamp(1, i32::MAX) as u32;

        return
            if let Some(percentage) = self.width_percentage {
                let percentage: u32 = percentage as u32;
                if percentage > 100 {
                    eprintln!("invalid width percentage: {}", percentage);
                    monitor_width
                } else {
                    percentage / 100 * monitor_width
                }
            } else if let Some(absolute) = self.width {
                min(absolute, monitor_width)
            } else {
                monitor_width
            }
        ;
    }

    pub fn get_height(&self) -> u32 {
        let monitor: Monitor = self.get_monitor();
        let monitor_height: u32 = monitor.geometry().height().clamp(1, i32::MAX) as u32;

        return
            if let Some(percentage) = self.height_percentage {
                let percentage: u32 = percentage as u32;
                if percentage > 100 {
                    eprintln!("invalid height percentage: {}", percentage);
                    monitor_height
                } else {
                    percentage / 100 * monitor_height
                }
            } else if let Some(absolute) = self.height {
                min(absolute, monitor_height)
            } else {
                monitor_height
            }
        ;
    }
}

#[derive(Debug)]
pub struct CfgKey {
    pub copy: Option<String>,
    pub paste: Option<String>,

    pub quit: Option<String>,

    pub transparency_toggle: Option<String>,

    pub tab_add: Option<String>,
    pub tab_close: Option<String>,
    pub tab_move_backward: Option<String>,
    pub tab_move_forward: Option<String>,
    pub tab_goto_next: Option<String>,
    pub tab_goto_previous: Option<String>,
    pub tab_goto_last: Option<String>,
    pub tab_goto_01: Option<String>,
    pub tab_goto_02: Option<String>,
    pub tab_goto_03: Option<String>,
    pub tab_goto_04: Option<String>,
    pub tab_goto_05: Option<String>,
    pub tab_goto_06: Option<String>,
    pub tab_goto_07: Option<String>,
    pub tab_goto_08: Option<String>,

    pub font_size_inc: Option<String>,
    pub font_size_dec: Option<String>,
    pub font_size_reset: Option<String>,
}

#[derive(Debug)]
pub struct CfgTerminal {
    pub allow_hyper_link: bool,
    pub audible_bell: bool,
    pub cursor_blink: bool,
    pub cursor_shape: CursorShape,
    pub scroll_on_output: bool,
    pub scroll_on_keystroke: bool,
    pub mouse_auto_hide: bool,
    pub scrollback_lines: i64,
    pub backspace_binding: EraseBinding,
    pub delete_binding: EraseBinding,
    pub word_char_exceptions: String,
}

impl CfgTerminal {
    pub fn cursor_blink_to_vte(&self) -> CursorBlinkMode {
        return match self.cursor_blink {
            true => CursorBlinkMode::On,
            false => CursorBlinkMode::Off
        };
    }
}

#[derive(Debug)]
pub struct CfgBehavior {
    pub terminal_exit_behavior: TerminalExitBehavior,
    pub last_tab_exit_behavior: LastTabExitBehavior,
    // pub prompt_on_exit: bool,
}

#[cfg(feature = "hack")]
#[derive(Debug)]
pub struct CfgHack {
    pub toggle: Vec<String>,
}

#[derive(Debug)]
pub struct ZohaCfg {
    pub font: CfgFont,
    pub display: CfgDisplay,
    pub color: CfgColor,
    pub process: CfgProcess,
    pub keys: CfgKey,
    pub terminal: CfgTerminal,
    pub behavior: CfgBehavior,
    #[cfg(feature = "hack")]
    pub hack: CfgHack,
}

// =============================================================================

mod defaults {
    pub(super) const CONFIG_FILE_NAME: &str = ".zoha.toml";
    pub(super) const FONT: &str = "Monospace Regular";
    pub(super) const FONT_SIZE: u8 = 16u8;
    pub(super) const X: u32 = 0;
    pub(super) const Y: u32 = 0;
    pub(super) const START_HIDDEN: bool = false;
    pub(super) const SKIP_TASK_BAR: bool = true;
    pub(super) const ALWAYS_ON_TOP: bool = true;
    pub(super) const STICKY: bool = true;
    pub(super) const FULLSCREEN: bool = false;
    pub(super) const BG_COLOR: &str = "rgba(0,0,0,0.95)";
    pub(super) const FG_COLOR: &str = "rgba(255,255,255,1.0)";
    pub(super) const CURSOR_COLOR: &str = "rgba(0,0,0,1.0)";
    pub(super) const TITLE: &str = "Zoha";
    pub(super) const CURSOR_BLINK: bool = false;
    pub(super) const ALLOW_HYPERLINK: bool = true;
    pub(super) const AUDIBLE_BELL: bool = false;
    pub(super) const SCROLL_ON_KEYSTROKE: bool = true;
    pub(super) const SCROLL_ON_OUTPUT: bool = false;
    pub(super) const MOUSE_AUTO_HIDE: bool = false;
    pub(super) const SCROLLBACK_LINES: i64 = -1;
    pub(super) const WORD_CHARS: &str = "-A-Za-z0-9,./?%&#:_";
    pub(super) const TAB_EXPAND: bool = false;
    pub(super) const TAB_NUM_CHARS: i8 = 25;
    // pub(super) const PROMPT_ON_EXIT: bool = false;
    pub(super) const TAB_SCROLL_WRAP: bool = true;

    pub(super) const TOGGLE_KEYCODE: &str = "F1";

    pub(super) const ACTION_TAB_ADD: &str = "<Ctrl><Shift>t";
    pub(super) const ACTION_TAB_CLOSE: &str = "<Ctrl><Shift>w";
    pub(super) const ACTION_TAB_MOVE_BACKWARD: &str = "<Ctrl><Shift>Page_Up";
    pub(super) const ACTION_TAB_MOVE_FORWARD: &str = "<Ctrl><Shift>Page_Down";
    pub(super) const ACTION_TAB_GOTO_NEXT: &str = "<Ctrl>Page_Down";
    pub(super) const ACTION_TAB_GOTO_PREV: &str = "<Ctrl>Page_Up";
    pub(super) const ACTION_TAB_GOTO_LAST: &str = "<Ctrl><Alt>9";
    pub(super) const ACTION_TAB_GOTO_1: &str = "<Ctrl><Alt>1";
    pub(super) const ACTION_TAB_GOTO_2: &str = "<Ctrl><Alt>2";
    pub(super) const ACTION_TAB_GOTO_3: &str = "<Ctrl><Alt>3";
    pub(super) const ACTION_TAB_GOTO_4: &str = "<Ctrl><Alt>4";
    pub(super) const ACTION_TAB_GOTO_5: &str = "<Ctrl><Alt>5";
    pub(super) const ACTION_TAB_GOTO_6: &str = "<Ctrl><Alt>6";
    pub(super) const ACTION_TAB_GOTO_7: &str = "<Ctrl><Alt>7";
    pub(super) const ACTION_TAB_GOTO_8: &str = "<Ctrl><Alt>8";
    pub(super) const ACTION_COPY: &str = "<Ctrl><Shift>c";
    pub(super) const ACTION_PASTE: &str = "<Ctrl><Shift>v";
    pub(super) const ACTION_QUIT: &str = "<Ctrl><Shift><Alt>q";
    pub(super) const ACTION_TRANSPARENCY_TOGGLE: &str = "F12";
    pub(super) const ACTION_FONT_SIZE_INC: &str = "<Ctrl><Alt>equal";
    pub(super) const ACTION_FONT_SIZE_DEC: &str = "<Ctrl><Alt>minus";
    pub(super) const ACTION_FONT_SIZE_RESET: &str = "<Ctrl><Alt>0";

    pub(super) fn default_key_codes() -> Vec<String> {
        return TOGGLE_KEYCODE.split('+').map(|it| it.to_string()).collect();
    }
}

fn try_parse_color(
    name: &str,
    color_spec: Option<String>,
) -> Option<RGBA> {
    color_spec.as_ref()?;

    let color_spec: String = color_spec.unwrap();

    return if let Some(color_code) = color_spec.strip_prefix("#x") {
        match u32::from_str_radix(color_code, 16) {
            Ok(code) => {
                if code > 0xffff {
                    eprintln!("invalid {}, value too big: {}", name, color_spec);
                    None
                } else {
                    Some(RGBA::new(
                        ((code / (256 * 256)) as f32) / (0xFFFF as f32),
                        (((code / 256) % 256) as f32) / (0xFFFF as f32),
                        ((code % 256) as f32) / (0xFFFF as f32),
                        1.0,
                    ))
                }
            }
            Err(err) => {
                eprintln!("invalid {}, {}: {}", name, err, color_spec);
                None
            }
        }
    } else {
        match RGBA::parse(&color_spec) {
            Ok(color) => {
                Some(color)
            }
            Err(_) => {
                eprintln!("invalid {}: {}", name, color_spec);
                None
            }
        }
    };
}

fn try_parse_color_or_default(
    name: &str,
    color_spec: Option<String>,
    default: &str,
) -> RGBA {
    return try_parse_color(name, color_spec).unwrap_or_else(|| {
        RGBA::parse(default).unwrap_or_else(|_| panic!("invalid default color for: {}", name))
    });
}

impl ZohaCfg {
    pub fn from_toml(cfg: &str) -> Self {
        use defaults::*;

        let mut seen = HashSet::new();

        return match toml::from_str::<RawCfg>(cfg) {
            Ok(raw) => {
                Self {
                    font: CfgFont {
                        font: FontDescription::from_string(&format!(
                            "{} {}",
                            &raw.font.font.unwrap_or_else(|| FONT.to_string()),
                            &raw.font.size.map(|it| max(it, 1)).unwrap_or(FONT_SIZE),
                        )),
                    },
                    display: CfgDisplay {
                        monitor: raw.display.monitor,
                        x_pos: raw.display.x_pos.unwrap_or(X),
                        y_pos: raw.display.y_pos.unwrap_or(Y),
                        width: raw.display.width,
                        height: raw.display.height,
                        width_percentage: raw.display.width_percentage,
                        height_percentage: raw.display.height_percentage,
                        start_hidden: raw.display.start_hidden.unwrap_or(START_HIDDEN),
                        skip_task_bar: raw.display.skip_task_bar.unwrap_or(SKIP_TASK_BAR),
                        always_on_top: raw.display.always_on_top.unwrap_or(ALWAYS_ON_TOP),
                        sticky: raw.display.sticky.unwrap_or(STICKY),
                        fullscreen: raw.display.fullscreen.unwrap_or(FULLSCREEN),
                        title: raw.display.title.unwrap_or_else(|| TITLE.to_string()),
                        tab_scroll_wrap: raw.display.tab_scroll_wrap.unwrap_or(TAB_SCROLL_WRAP),
                        tab_mode: raw.display.tab_mode.unwrap_or(TabMode::Auto),
                        tab_position: raw.display.tab_position.unwrap_or(TabPosition::Top),
                        tab_expand: raw.display.tab_expand.unwrap_or(TAB_EXPAND),
                        tab_title_num_characters: raw.display.tab_title_num_characters,
                        scrollbar_position: raw.display.scrollbar_position
                            .unwrap_or(ScrollbarPosition::Hidden),
                    },
                    color: CfgColor {
                        bg: try_parse_color_or_default(
                            "bg_color",
                            raw.color.bg,
                            BG_COLOR,
                        ),
                        fg: try_parse_color_or_default(
                            "fg_color",
                            raw.color.fg,
                            FG_COLOR,
                        ),
                        cursor: try_parse_color_or_default(
                            "cursor_color",
                            raw.color.cursor,
                            CURSOR_COLOR,
                        ),
                        pallet: raw.color.pallet.unwrap_or(Pallet::Tango),
                        color_00: try_parse_color(
                            "color_00",
                            raw.color.color_00,
                        ),
                        color_01: try_parse_color(
                            "color_01",
                            raw.color.color_01,
                        ),
                        color_02: try_parse_color(
                            "color_02",
                            raw.color.color_02,
                        ),
                        color_03: try_parse_color(
                            "color_03",
                            raw.color.color_03,
                        ),
                        color_04: try_parse_color(
                            "color_04",
                            raw.color.color_04,
                        ),
                        color_05: try_parse_color(
                            "color_05",
                            raw.color.color_05,
                        ),
                        color_06: try_parse_color(
                            "color_06",
                            raw.color.color_06,
                        ),
                        color_07: try_parse_color(
                            "color_07",
                            raw.color.color_07,
                        ),
                        color_08: try_parse_color(
                            "color_08",
                            raw.color.color_08,
                        ),
                        color_09: try_parse_color(
                            "color_09",
                            raw.color.color_09,
                        ),
                        color_10: try_parse_color(
                            "color_10",
                            raw.color.color_10,
                        ),
                        color_11: try_parse_color(
                            "color_11",
                            raw.color.color_11,
                        ),
                        color_12: try_parse_color(
                            "color_12",
                            raw.color.color_12,
                        ),
                        color_13: try_parse_color(
                            "color_13",
                            raw.color.color_13,
                        ),
                        color_14: try_parse_color(
                            "color_14",
                            raw.color.color_14,
                        ),
                        color_15: try_parse_color(
                            "color_15",
                            raw.color.color_15,
                        ),
                    },
                    process: CfgProcess {
                        command: raw.process.command
                            .map(|it| shell(Some(it)))
                            .unwrap_or_else(|| shell(None)),
                        working_dir: raw.process.working_dir,
                    },
                    keys: CfgKey {
                        copy: sanitize_key(
                            raw.keys.copy,
                            ACTION_COPY,
                            &mut seen,
                        ),
                        paste: sanitize_key(
                            raw.keys.paste,
                            ACTION_PASTE,
                            &mut seen,
                        ),
                        quit: sanitize_key(
                            raw.keys.quit,
                            ACTION_QUIT,
                            &mut seen,
                        ),
                        transparency_toggle: sanitize_key(
                            raw.keys.transparency_toggle,
                            ACTION_TRANSPARENCY_TOGGLE,
                            &mut seen,
                        ),
                        tab_add: sanitize_key(
                            raw.keys.tab_add,
                            ACTION_TAB_ADD,
                            &mut seen,
                        ),
                        tab_close: sanitize_key(
                            raw.keys.tab_close,
                            ACTION_TAB_CLOSE,
                            &mut seen,
                        ),
                        tab_move_backward: sanitize_key(
                            raw.keys.tab_move_backward,
                            ACTION_TAB_MOVE_BACKWARD,
                            &mut seen,
                        ),
                        tab_move_forward: sanitize_key(
                            raw.keys.tab_move_forward,
                            ACTION_TAB_MOVE_FORWARD,
                            &mut seen,
                        ),
                        tab_goto_next: sanitize_key(
                            raw.keys.tab_goto_next,
                            ACTION_TAB_GOTO_NEXT,
                            &mut seen,
                        ),
                        tab_goto_previous: sanitize_key(
                            raw.keys.tab_goto_previous,
                            ACTION_TAB_GOTO_PREV,
                            &mut seen,
                        ),
                        tab_goto_last: sanitize_key(
                            raw.keys.tab_goto_last,
                            ACTION_TAB_GOTO_LAST,
                            &mut seen,
                        ),
                        tab_goto_01: sanitize_key(
                            raw.keys.tab_goto_01,
                            ACTION_TAB_GOTO_1,
                            &mut seen,
                        ),
                        tab_goto_02: sanitize_key(
                            raw.keys.tab_goto_02,
                            ACTION_TAB_GOTO_2,
                            &mut seen,
                        ),
                        tab_goto_03: sanitize_key(
                            raw.keys.tab_goto_03,
                            ACTION_TAB_GOTO_3,
                            &mut seen,
                        ),
                        tab_goto_04: sanitize_key(
                            raw.keys.tab_goto_04,
                            ACTION_TAB_GOTO_4,
                            &mut seen,
                        ),
                        tab_goto_05: sanitize_key(
                            raw.keys.tab_goto_05,
                            ACTION_TAB_GOTO_5,
                            &mut seen,
                        ),
                        tab_goto_06: sanitize_key(
                            raw.keys.tab_goto_06,
                            ACTION_TAB_GOTO_6,
                            &mut seen,
                        ),
                        tab_goto_07: sanitize_key(
                            raw.keys.tab_goto_07,
                            ACTION_TAB_GOTO_7,
                            &mut seen,
                        ),
                        tab_goto_08: sanitize_key(
                            raw.keys.tab_goto_08,
                            ACTION_TAB_GOTO_8,
                            &mut seen,
                        ),
                        font_size_inc: sanitize_key(
                            raw.keys.font_size_inc,
                            ACTION_FONT_SIZE_INC,
                            &mut seen,
                        ),
                        font_size_dec: sanitize_key(
                            raw.keys.font_size_dec,
                            ACTION_FONT_SIZE_DEC,
                            &mut seen,
                        ),
                        font_size_reset: sanitize_key(
                            raw.keys.font_size_reset,
                            ACTION_FONT_SIZE_RESET,
                            &mut seen,
                        ),
                    },
                    terminal: CfgTerminal {
                        allow_hyper_link: raw.terminal.allow_hyper_link
                            .unwrap_or(ALLOW_HYPERLINK),
                        audible_bell: raw.terminal.audible_bell
                            .unwrap_or(AUDIBLE_BELL),
                        cursor_blink: raw.terminal.cursor_blink
                            .unwrap_or(CURSOR_BLINK),
                        cursor_shape: raw.terminal.cursor_shape
                            .unwrap_or(CursorShape::Block),
                        scroll_on_output: raw.terminal.scroll_on_output
                            .unwrap_or(SCROLL_ON_OUTPUT),
                        scroll_on_keystroke: raw.terminal.scroll_on_keystroke
                            .unwrap_or(SCROLL_ON_KEYSTROKE),
                        mouse_auto_hide: raw.terminal.mouse_auto_hide
                            .unwrap_or(MOUSE_AUTO_HIDE),
                        scrollback_lines: raw.terminal.scrollback_lines
                            .unwrap_or(SCROLLBACK_LINES),
                        backspace_binding: raw.terminal.backspace_binding
                            .unwrap_or(EraseBinding::Auto),
                        delete_binding: raw.terminal.delete_binding
                            .unwrap_or(EraseBinding::Auto),
                        word_char_exceptions: raw.terminal.word_char_exceptions
                            .unwrap_or_else(|| WORD_CHARS.to_string()),
                    },
                    behavior: CfgBehavior {
                        terminal_exit_behavior: raw.behavior.terminal_exit_behavior
                            .unwrap_or(TerminalExitBehavior::ExitTerminal),
                        last_tab_exit_behavior: raw.behavior.last_tab_exit_behavior
                            .unwrap_or(LastTabExitBehavior::RestartTerminal),
                        // prompt_on_exit: raw.behavior.prompt_on_exit
                        //     .unwrap_or(PROMPT_ON_EXIT),
                    },
                    #[cfg(feature = "hack")]
                    hack: CfgHack {
                        toggle: raw.hack.toggle.unwrap_or_else(default_key_codes),
                    },
                }
            }
            Err(e) => {
                eprintln!("failed to parse config file, using default values: {}", e);
                Self::default()
            }
        };
    }
}

impl Default for ZohaCfg {
    fn default() -> Self {
        use defaults::*;

        Self {
            font: CfgFont {
                font: FontDescription::from_string(&format!("{} {}", FONT, FONT_SIZE)),
            },
            display: CfgDisplay {
                monitor: None,
                x_pos: X,
                y_pos: Y,
                width: None,
                height: None,
                width_percentage: None,
                height_percentage: None,
                start_hidden: START_HIDDEN,
                skip_task_bar: SKIP_TASK_BAR,
                always_on_top: ALWAYS_ON_TOP,
                sticky: STICKY,
                fullscreen: FULLSCREEN,
                title: TITLE.to_string(),
                tab_scroll_wrap: TAB_SCROLL_WRAP,
                tab_mode: TabMode::Auto,
                tab_position: TabPosition::Top,
                tab_expand: TAB_EXPAND,
                tab_title_num_characters: Some(TAB_NUM_CHARS),
                scrollbar_position: ScrollbarPosition::Hidden,
            },
            color: CfgColor {
                bg: RGBA::from_str(BG_COLOR).unwrap(),
                fg: RGBA::from_str(FG_COLOR).unwrap(),
                cursor: RGBA::from_str(CURSOR_COLOR).unwrap(),
                pallet: Pallet::Tango,
                color_00: None,
                color_01: None,
                color_02: None,
                color_03: None,
                color_04: None,
                color_05: None,
                color_06: None,
                color_07: None,
                color_08: None,
                color_09: None,
                color_10: None,
                color_11: None,
                color_12: None,
                color_13: None,
                color_14: None,
                color_15: None,
            },
            process: CfgProcess {
                command: shell(None),
                working_dir: None,
            },
            keys: CfgKey {
                copy: Some(ACTION_COPY.to_string()),
                paste: Some(ACTION_PASTE.to_string()),
                quit: Some(ACTION_QUIT.to_string()),
                transparency_toggle: Some(ACTION_TRANSPARENCY_TOGGLE.to_string()),
                tab_add: Some(ACTION_TAB_ADD.to_string()),
                tab_close: Some(ACTION_TAB_CLOSE.to_string()),
                tab_move_backward: Some(ACTION_TAB_MOVE_BACKWARD.to_string()),
                tab_move_forward: Some(ACTION_TAB_MOVE_FORWARD.to_string()),
                tab_goto_next: Some(ACTION_TAB_GOTO_NEXT.to_string()),
                tab_goto_previous: Some(ACTION_TAB_GOTO_PREV.to_string()),
                tab_goto_last: Some(ACTION_TAB_GOTO_LAST.to_string()),
                tab_goto_01: Some(ACTION_TAB_GOTO_1.to_string()),
                tab_goto_02: Some(ACTION_TAB_GOTO_2.to_string()),
                tab_goto_03: Some(ACTION_TAB_GOTO_3.to_string()),
                tab_goto_04: Some(ACTION_TAB_GOTO_4.to_string()),
                tab_goto_05: Some(ACTION_TAB_GOTO_5.to_string()),
                tab_goto_06: Some(ACTION_TAB_GOTO_6.to_string()),
                tab_goto_07: Some(ACTION_TAB_GOTO_7.to_string()),
                tab_goto_08: Some(ACTION_TAB_GOTO_8.to_string()),
                font_size_inc: Some(ACTION_FONT_SIZE_INC.to_string()),
                font_size_dec: Some(ACTION_FONT_SIZE_DEC.to_string()),
                font_size_reset: Some(ACTION_FONT_SIZE_RESET.to_string()),
            },
            terminal: CfgTerminal {
                allow_hyper_link: ALLOW_HYPERLINK,
                audible_bell: AUDIBLE_BELL,
                cursor_blink: CURSOR_BLINK,
                cursor_shape: CursorShape::Block,
                scroll_on_output: SCROLL_ON_OUTPUT,
                scroll_on_keystroke: SCROLL_ON_KEYSTROKE,
                mouse_auto_hide: MOUSE_AUTO_HIDE,
                scrollback_lines: SCROLLBACK_LINES,
                backspace_binding: EraseBinding::Auto,
                delete_binding: EraseBinding::Auto,
                word_char_exceptions: WORD_CHARS.to_string(),
            },
            behavior: CfgBehavior {
                terminal_exit_behavior: TerminalExitBehavior::ExitTerminal,
                last_tab_exit_behavior: LastTabExitBehavior::RestartTerminal,
                // prompt_on_exit: PROMPT_ON_EXIT,
            },
            #[cfg(feature = "hack")]
            hack: CfgHack {
                toggle: default_key_codes(),
            },
        }
    }
}

#[derive(Error, Debug)]
pub enum CfgReadError {
    #[error("overridden config location is specified but it does not exist: {location}")]
    OverriddenCfgDoesNotExist {
        location: String,
    },

    #[error("no config location specified and user has no home directory")]
    NoHomeDir,

    #[error("overridden config not specified and no config file in user home directory, \
             looked for: {location}")]
    NoConfigInHomeDir {
        location: String,
    },

    #[error("error reading the config file, tried to read: {location}, error: {error}")]
    FileReadError {
        location: String,
        error: std::io::Error,
    },
}

impl CfgReadError {
    pub fn is_no_config(&self) -> bool {
        return matches!(&self, CfgReadError::NoConfigInHomeDir {..});
    }
}

fn do_read_cfg(cfg_location: &Path) -> Result<String, CfgReadError> {
    return match fs::read_to_string(cfg_location) {
        Ok(content) => Ok(content),
        Err(err) => {
            Err(CfgReadError::FileReadError {
                location: cfg_location.to_str().unwrap().to_string(),
                error: err,
            })
        }
    };
}

pub fn read_cfg_content(args: &ZohaArgs) -> Result<String, CfgReadError> {
    if args.cfg_file.is_some() {
        let cfg_path: &Path = Path::new(args.cfg_file.as_ref().unwrap());
        if !cfg_path.exists() {
            return Err(CfgReadError::OverriddenCfgDoesNotExist {
                location: args.cfg_file.as_ref().unwrap().to_string()
            });
        }

        do_read_cfg(cfg_path)
    } else {
        let cfg_path: PathBuf = match dirs::home_dir() {
            None => {
                return Err(CfgReadError::NoHomeDir);
            }
            Some(home_dir) => {
                let home_cfg_path = home_dir.join(defaults::CONFIG_FILE_NAME);
                if !home_cfg_path.exists() {
                    return Err(CfgReadError::NoConfigInHomeDir {
                        location: home_cfg_path.to_str().unwrap().to_string()
                    });
                }
                home_cfg_path
            }
        };

        do_read_cfg(cfg_path.as_path())
    }
}

fn sanitize_key(key: Option<String>,
                default: &str,
                seen: &mut HashSet<String>) -> Option<String> {
    fn do_sanitize_key(key: &str) -> Option<String> {
        let parse = accelerator_parse(key);

        return if parse.is_some() {
            Some(key.to_string())
        } else {
            eprintln!("failed to parse accelerator: {}", &key);
            None
        };
    }

    return if let Some(key) = key {
        if key.trim().is_empty() {
            None
        } else if let Some(key) = do_sanitize_key(&key) {
            if seen.insert(key.clone()) {
                Some(key)
            } else {
                eprintln!("duplicate key used for short cut, not adding: {}", key);
                None
            }
        } else {
            do_sanitize_key(default)
        }
    } else {
        do_sanitize_key(default)
    };
}

fn shell(user_cmd: Option<String>) -> String {
    if let Some(user_cmd) = user_cmd {
        let exec_path = Path::new(&user_cmd);
        if let Ok(meta) = fs::metadata(exec_path) {
            if !meta.is_dir() && ((meta.permissions().mode() & 0o111) > 0) {
                return user_cmd;
            }
        }

        eprintln!("can not execute user provided command, \
                   ignoring it and using default shell: {}", user_cmd);
    }

    if let Ok(exec) = std::env::var("SHELL") {
        let exec_path = Path::new(&exec);
        if let Ok(meta) = fs::metadata(exec_path) {
            if !meta.is_dir() && ((meta.permissions().mode() & 0o111) > 0) {
                return exec;
            }
        }
    }

    for exec_path in [
        "/usr/bin/bash",
        "/bin/bash",
        "/usr/bin/zsh",
        "/bin/zsh",
        "/usr/bin/fish",
        "/bin/fish",
    ] {
        if let Ok(meta) = fs::metadata(exec_path) {
            if !meta.is_dir() && ((meta.permissions().mode() & 0o111) > 0) {
                return exec_path.to_string();
            }
        }
    }

    panic!("Could not locate any shell");
}

// =============================================================================

#[cfg(test)]
mod tests {
    use crate::config::cfg::{RawCfg, ZohaCfg};

    #[test]
    fn test_cfg_deser() {
        let raw_cfg = toml::from_str::<RawCfg>("").unwrap();
        let cfg: ZohaCfg = raw_cfg.try_into().unwrap();
        println!("{:?}", cfg);
    }
}
