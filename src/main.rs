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
    let mut dest_file: Option<String> = None;

    // -v, --version
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let option: &str = &args[1];
        if option.eq("-v") || option.eq("--version") {
            println!("\n  Transgender.rs\n\n    Regex-powered trans\n");
            exit(0);
        } else if option.eq("--sh") {
            let script = r###"
ts() {
  dest_file=$(mktemp -t "ts.XXXX")
  transgender $1 --dest "${dest_file}"
  dest_dir=$(tail -n 1 "${dest_file}")
  cd "${dest_dir}"
  rm -rf "${dest_file}"
}

if [[ "$0" =~ .*bash$ ]];
then
  complete -o dirnames ts
fi

if [[ "$0" == "zsh" ]];
then
  true
fi
"###;
            print!("{}", script);
            exit(0);
        } else {
            // otherwise the first argument will be identified as a path
            let p = Path::new(&args[1]);
            if p.is_dir() {
                path = String::from(p.to_str().expect("Failed to convert an existed path"));
            }
        }

        if args.len() > 2 && option.eq("--dest") {
            dest_file = Some(args[2].clone());
        }
    }

    let mut canvas = canvas::new();
    let mut browser = browser::new(&path, dest_file);

    util::enter_albuf();
    browser.start_loop(&mut canvas);
}
