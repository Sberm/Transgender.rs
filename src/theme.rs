#[derive(Default)]
pub struct Theme {
    // Highlighted filename when selected
    pub highlight: String,
    // Highlighted directory name when selected
    pub highlight_dir: String,
    // Highlight block background
    pub highlight_background: String,
    // Normal entry names
    pub normal: String,
    // Background of everything
    pub normal_background: String,
}

impl Theme {
    pub fn from(name: &str) -> Self {
        let theme_table = ThemeTable::new();

        for t in theme_table.theme_entries.iter() {
            if t.name == name {
                return Theme {
                    highlight: t.color[0].clone(),
                    highlight_dir: t.color[1].clone(),
                    highlight_background: t.color[2].clone(),
                    normal: t.color[3].clone(),
                    normal_background: t.color[4].clone(),
                };
            }
        }

        // If no theme is matched, use trans as default
        for t in theme_table.theme_entries.iter() {
            if t.name == "trans" {
                return Theme {
                    highlight: t.color[0].clone(),
                    highlight_dir: t.color[1].clone(),
                    highlight_background: t.color[2].clone(),
                    normal: t.color[3].clone(),
                    normal_background: t.color[4].clone(),
                };
            }
        }

        // Fallback just to pass the compiler
        Theme::default()
    }
}

struct ThemeTable {
    theme_entries: Vec<ThemeEntry>,
}

struct ThemeEntry {
    name: String,
    color: [String; 5],
}

macro_rules! __theme {
    ( $name: expr, $($x: expr),+ ) => {
        ThemeEntry {
            name: String::from($name),
            color: [
            $(
                String::from($x),
            )*],
        }
    };
}

impl ThemeTable {
    fn new() -> Self {
        ThemeTable {
            theme_entries: vec![
                __theme![
                    "trans",
                    "\x1b[0;30m",
                    "\x1b[38;5;57m",
                    "\x1b[48;5;175m",
                    "\x1b[0;37m",
                    "\x1b[48;5;31m"
                ],
                __theme![
                    "dark",
                    "\x1b[38;5;0m",
                    "\x1b[38;5;27m",
                    "\x1b[48;5;255m",
                    "\x1b[38;5;255m",
                    "\x1b[48;5;0m"
                ],
            ],
        }
    }
}
