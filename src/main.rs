mod browser;
mod canvas;
mod util;
mod ops;

use std::io::{self, Write, BufRead};
use std::env::var;
use std::path::{Path};
use std::fs::File;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_editor() -> String {
    if let Ok(home_dir) = var(ops::static::HOME_VAR) {
        if let Ok(lines) = read_lines(&format!("{}/{}", home_dir, ops::static::CONFIG_FILE)) {
            for line in lines.flatten() {
                let trimmed = line.replace(" ", "");

                let kv = trimmed.split("=").collect::<Vec<&str>>();
                if kv.len() != 2 {
                    break;
                }
                if kv[0].eq(ops::static::EDITOR_KEY) {
                    println!("editor in config {}", kv[1]);
                    return String::from(kv[1]);
                }
            }
        }
    }
    return String::from(ops::static::EDITOR)
}


fn main() {
    print!("\x1b[?1049h"); // alternate screen buffer
    util::raw_input();
    let _ = io::stdout().flush();
    
    let mut canvas = canvas::init();

    let mut browser = browser::Browser {
        cursor: 0,
        window_start: 0,
        current_dir: Vec::new(),
        past_dir: Vec::new(),
        past_cursor: Vec::new(),
        past_window_start: Vec::new(),
        current_path: String::from(""),
        original_path: String::from(""),
        mode: ops::Mode::NORMAL,
        search_txt: Vec::new(),
        ops: ops::Ops{editor: get_editor()},
    };

    browser.init();

    browser.start_loop(&mut canvas);
}
