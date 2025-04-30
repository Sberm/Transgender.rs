/*═══════════════════════════════════════════════════════════════════════╗
║                         (C)  Howard Chu                                ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

/// All the constant
pub mod consts {
    pub const HOME_VAR: &str = "HOME";
    pub const CONFIG_FILE: &str = ".tsrc";
    pub const EDITOR_KEY: &str = "editor";
    pub const OPENER_KEY: &str = "open";
    pub const OPENER: &str = "vi";
    pub const THEME_KEY: &str = "theme";
    pub const O_KEY: &str = "o";
    pub const ENTER_KEY: &str = "enter";
}

/// File browser operation code
#[derive(PartialEq)]
pub enum Op {
    Noop,
    Up,
    Down,
    Left,
    Right,
    Exit,
    ExitCursorO,
    ExitCursorEnter,
    Quit,
    Top,
    Bottom,
    Search,
    RevSearch,
    NextMatch,
    PrevMatch,
    PageUp,
    PageDown,
}

#[derive(Copy, Clone)]
pub enum Mode {
    Normal,
    Search,
    RevSearch,
}
