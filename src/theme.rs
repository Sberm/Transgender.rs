/*═══════════════════════════════════════════════════════════════════════╗
║                         (C)  Howard Chu                                ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

#[derive(Default, Clone)]
pub struct Theme {
    // filename when selected
    pub highlight: String,
    // directory name when selected
    pub highlight_dir: String,
    // background of highlighted line
    pub highlight_background: String,
    // normal entry names
    pub normal: String,
    // background of everything
    pub normal_background: String,
    // these two are optional, if not specified, bottom_bar uses normal, and bottom_bar_background
    // uses normal_background
    pub bottom_bar: String,
    pub bottom_bar_background: String,
}

impl Theme {
    pub fn from(name: &str) -> Self {
        let theme_table = ThemeTable::new();
        let mut theme_i = 0;
        // default
        for (i, t) in theme_table.theme_entries.iter().enumerate() {
            if t.name == "lucius" {
                theme_i = i;
                break;
            }
        }
        for (i, t) in theme_table.theme_entries.iter().enumerate() {
            if t.name == name {
                theme_i = i;
                break;
            }
        }
        theme_table.theme_entries[theme_i].theme.clone()
    }
}

struct ThemeTable {
    theme_entries: Vec<ThemeEntry>,
}

struct ThemeEntry {
    name: String,
    theme: Theme
}

impl ThemeTable {
    fn new() -> Self {
        ThemeTable {
            theme_entries: vec![
                ThemeEntry {
                    name: "trans".to_string(),
                    theme: Theme {
                        highlight:             "\x1b[0;37m".to_string(),
                        highlight_dir:         "\x1b[38;5;175m".to_string(),
                        highlight_background:  "\x1b[48;5;24m".to_string(),
                        normal:                "\x1b[0;37m".to_string(),
                        normal_background:     "\x1b[48;5;31m".to_string(),
                        bottom_bar:            "\x1b[38;5;0m".to_string(),
                        bottom_bar_background: "\x1b[48;5;175m".to_string()
                    }
                },
                ThemeEntry {
                    name: "dark".to_string(),
                    theme: Theme {
                        highlight:             "\x1b[38;5;0m".to_string(),
                        highlight_dir:         "\x1b[38;5;27m".to_string(),
                        highlight_background:  "\x1b[48;5;255m".to_string(),
                        normal:                "\x1b[38;5;255m".to_string(),
                        normal_background:     "\x1b[48;5;0m".to_string(),
                        bottom_bar:            "\x1b[38;5;255m".to_string(),
                        bottom_bar_background: "\x1b[38;5;255m".to_string()
                    }
                },
                ThemeEntry {
                    name: "lucius".to_string(),
                    theme: Theme {
                        highlight:             "\x1b[38;5;187m".to_string(),
                        highlight_dir:         "\x1b[38;5;117m".to_string(),
                        highlight_background:  "\x1b[48;5;238m".to_string(),
                        normal:                "\x1b[38;5;188m".to_string(),
                        normal_background:     "\x1b[48;5;236m".to_string(),
                        bottom_bar:            "\x1b[38;5;188m".to_string(),
                        bottom_bar_background: "\x1b[48;5;238m".to_string()
                    }
                },
                ThemeEntry {
                    name: "acme".to_string(),
                    theme: Theme {
                        highlight:             "\x1b[38;5;233m".to_string(),
                        highlight_dir:         "\x1b[38;5;39m".to_string(),
                        highlight_background:  "\x1b[48;5;186m".to_string(),
                        normal:                "\x1b[38;5;233m".to_string(),
                        normal_background:     "\x1b[48;5;230m".to_string(),
                        bottom_bar:            "\x1b[38;5;233m".to_string(),
                        bottom_bar_background: "\x1b[48;5;195m".to_string()
                    }
                },
                ThemeEntry {
                    name: "sakura".to_string(),
                    theme: Theme {
                        highlight:             "\x1b[38;5;253m".to_string(),
                        highlight_dir:         "\x1b[38;5;52m".to_string(),
                        highlight_background:  "\x1b[48;5;175m".to_string(),
                        normal:                "\x1b[38;5;253m".to_string(),
                        normal_background:     "\x1b[48;5;168m".to_string(),
                        bottom_bar:            "\x1b[38;5;52m".to_string(),
                        bottom_bar_background: "\x1b[48;5;175m".to_string()
                    }
                },
                ThemeEntry {
                    name: "vscode".to_string(),
                    theme: Theme {
                        highlight:             "\x1b[38;5;176m".to_string(),
                        highlight_dir:         "\x1b[38;5;43m".to_string(),
                        highlight_background:  "\x1b[48;5;236m".to_string(),
                        normal:                "\x1b[38;5;75m".to_string(),
                        normal_background:     "\x1b[48;5;235m".to_string(),
                        bottom_bar:            "\x1b[38;5;117m".to_string(),
                        bottom_bar_background: "\x1b[48;5;236m".to_string()
                    }
                },
                ThemeEntry {
                    name: "jesus".to_string(),
                    theme: Theme {
                        highlight:             "\x1b[38;5;94m".to_string(),
                        highlight_dir:         "\x1b[38;5;236m".to_string(),
                        highlight_background:  "\x1b[48;5;180m".to_string(),
                        normal:                "\x1b[38;5;187m".to_string(),
                        normal_background:     "\x1b[48;5;137m".to_string(),
                        bottom_bar:            "\x1b[38;5;236m".to_string(),
                        bottom_bar_background: "\x1b[48;5;180m".to_string()
                    }
                },
                ThemeEntry {
                    name: "catppuccin".to_string(),
                    theme: Theme {
                        highlight:             "\x1b[38;2;238;212;159m".to_string(),
                        highlight_dir:         "\x1b[38;2;245;169;127m".to_string(),
                        highlight_background:  "\x1b[48;2;48;51;71m".to_string(),
                        normal:                "\x1b[38;2;138;173;244m".to_string(),
                        normal_background:     "\x1b[48;2;36;39;58m".to_string(),
                        bottom_bar:            "\x1b[38;2;128;135;162m".to_string(),
                        bottom_bar_background: "\x1b[48;2;54;58;79m".to_string()
                    }
                },
                ThemeEntry {
                    name: "lucius-l".to_string(),
                    theme: Theme {
                        highlight:             "\x1b[38;5;130m".to_string(),
                        highlight_dir:         "\x1b[38;5;25m".to_string(),
                        highlight_background:  "\x1b[48;5;253m".to_string(),
                        normal:                "\x1b[38;5;238m".to_string(),
                        normal_background:     "\x1b[48;5;255m".to_string(),
                        bottom_bar:            "\x1b[38;5;255m".to_string(),
                        bottom_bar_background: "\x1b[48;5;244m".to_string()
                    }
                }
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_theme() {
        let name = "trans";
        let need = Theme::from(name);
        let theme_table = ThemeTable::new();
        let mut theme_entry: Option<&ThemeEntry> = None;
        for t in theme_table.theme_entries.iter() {
            if name == t.name {
                theme_entry = Some(t);
            }
        }
        if theme_entry.is_some() {
            let got = &theme_entry.unwrap().theme;
            assert_eq!(got.highlight,             need.highlight);
            assert_eq!(got.highlight_dir,         need.highlight_dir);
            assert_eq!(got.highlight_background,  need.highlight_background);
            assert_eq!(got.normal,                need.normal);
            assert_eq!(got.normal_background,     need.normal_background);
            assert_eq!(got.bottom_bar,            need.bottom_bar);
            assert_eq!(got.bottom_bar_background, need.bottom_bar_background);
        } else {
            panic!("failed to get trans theme");
        }
    }
}
