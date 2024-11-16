/*═══════════════════════════════════════════════════════════════════════╗
║                          ©  Howard Chu                                 ║
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
    pub const EDITOR: &str = "/bin/vi";

    pub const THEME_KEY: &str = "theme";
}

/// File browser operation code
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
