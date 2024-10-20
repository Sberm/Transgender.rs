mod browser;
mod canvas;
mod ops;
mod utf8;
mod util;
use std::env;
use std::process::exit;

fn main() {
    let mut canvas = canvas::new();
    let mut browser = browser::new();

    /* -v */
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && (args[1].eq("-v") || args[1].eq("--version")) {
        println!("\n  Transgender.rs\n\n    Vanilla trans\n");
        exit(0);
    }

    util::enter_albuf();
    browser.start_loop(&mut canvas);
}
