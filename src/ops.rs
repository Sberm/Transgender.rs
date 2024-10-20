pub enum Theme {
    TRANS,
    DARK,
}

/*
 * All the constant
 */
pub mod consts {
    pub const HOME_VAR: &str = "HOME";
    pub const CONFIG_FILE: &str = ".tsrc";

    pub const EDITOR_KEY: &str = "editor";
    pub const EDITOR: &str = "/bin/vi";

    pub const THEME_KEY: &str = "theme";
    pub const THEME_DARK: &str = "dark";

    /* If BG of 0;{ID} is set, the color of the text itself cannot be changed */
    pub const HIGHLIGHT_TRANS: &str = "\x1b[0;30m";
    pub const HIGHLIGHT_DIR_TRANS: &str = "\x1b[38;5;57m";
    pub const HIGHLIGHT_BG_TRANS: &str = "\x1b[48;5;175m";
    pub const NORMAL_TRANS: &str = "\x1b[0;37m";
    pub const NORMAL_BG_TRANS: &str = "\x1b[48;5;31m";

    pub const HIGHLIGHT_DARK: &str = "\x1b[38;5;0m";
    pub const HIGHLIGHT_DIR_DARK: &str = "\x1b[38;5;27m";
    pub const HIGHLIGHT_BG_DARK: &str = "\x1b[48;5;255m";
    pub const NORMAL_DARK: &str = "\x1b[38;5;255m";
    pub const NORMAL_BG_DARK: &str = "\x1b[48;5;0m";
}

/*
 * File browser operation code
 */
pub mod code {
    pub const NOOP: u8 = 0;
    pub const UP: u8 = 1;
    pub const DOWN: u8 = 2;
    pub const LEFT: u8 = 3;
    pub const RIGHT: u8 = 4;
    pub const EXIT: u8 = 5;
    pub const EXIT_CURSOR: u8 = 6;
    pub const QUIT: u8 = 7;
    pub const TOP: u8 = 8;
    pub const BOTTOM: u8 = 9;
    pub const SEARCH: u8 = 10;
    pub const NEXT_MATCH: u8 = 11;
    pub const PREV_MATCH: u8 = 12;
}

#[derive(Copy, Clone)]
pub enum Mode {
    NORMAL,
    SEARCH,
}

pub struct Ops {
    pub editor: String,
}
