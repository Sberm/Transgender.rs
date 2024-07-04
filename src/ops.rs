pub mod consts {
    pub static HOME_VAR: &str = "HOME";
    pub static EDITOR: &str = "/bin/vi";
    pub static CONFIG_FILE: &str = ".tsrc";
    pub static EDITOR_KEY: &str = "editor";
}

pub mod code {
    pub const NOOP:u8 = 0;
    pub const UP:u8 = 1;
    pub const DOWN:u8 = 2;
    pub const LEFT:u8 = 3;
    pub const RIGHT:u8 = 4;
    pub const EXIT:u8 = 5;
    pub const EXIT_CURSOR:u8 = 6;
    pub const QUIT:u8 = 7;
    pub const TOP:u8 = 8;
    pub const BOTTOM:u8 = 9;
    pub const SEARCH:u8 = 10;
    pub const NEXT_MATCH:u8 = 11;
}

#[derive(Copy, Clone)]
pub enum Mode{
    NORMAL,
    SEARCH,
}

pub struct Ops {
    pub editor: String,
}
