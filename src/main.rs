mod browser;
mod canvas;
mod ops;
mod util;
mod utf8;

fn main() {
    let mut canvas = canvas::new();
    let mut browser = browser::new();

    util::enter_albuf();
    browser.start_loop(&mut canvas);
}
