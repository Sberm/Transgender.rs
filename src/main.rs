mod browser;
mod canvas;
mod util;
mod ops;

use std::io::{self, Write};

fn main() {
    print!("\x1b[?1049h"); // alternate screen buffer
    util::raw_input();
    let _ = io::stdout().flush();
    
    let mut canvas = canvas::new();
    let mut browser = browser::new();

    browser.start_loop(&mut canvas);
}
