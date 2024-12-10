/*═══════════════════════════════════════════════════════════════════════╗
║                         (C)  Howard Chu                                ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

mod browser;
mod canvas;
mod ops;
mod theme;
mod util;
mod widechar_width;

use std::env;
use std::path::Path;
use std::process::exit;

fn main() {
    let mut path = String::from(".");

    // -v, --version
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let option: &str = &args[1];
        if option.eq("-v") || option.eq("--version") {
            println!("\n  Transgender.rs\n\n    Regex-powered trans\n");
            exit(0);
        } else if option.eq("--sh") {
            let script = r#"
ts () {
  cd "$(transgender $1 3>&1 1>&2 2>&3 3>&- | tail -n 1)"
}

complete -o dirnames ts"#;

            println!("{}", script);
            exit(0);
        } else {
            // otherwise the first argument will be identified as a path
            let p = Path::new(&args[1]);
            if p.exists() {
                path = String::from(p.to_str().expect("Failed to convert an existed path"));
            }
        }
    }

    let mut canvas = canvas::new();
    let mut browser = browser::new(&path);

    util::enter_albuf();
    browser.start_loop(&mut canvas);
}
