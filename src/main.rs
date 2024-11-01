/*═══════════════════════════════════════════════════════════════════════╗
║                          ©  Howard Chu                                 ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

mod browser;
mod canvas;
mod ops;
mod util;
mod widechar_width;

use std::env;
use std::process::exit;

fn main() {
    let mut canvas = canvas::new();
    let mut browser = browser::new();

    // -v
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1].eq("-v") || args[1].eq("--version")) {
        println!("\n  Transgender.rs\n\n    Regex-powered trans\n");
        exit(0);
    }

    util::enter_albuf();
    browser.start_loop(&mut canvas);
}
