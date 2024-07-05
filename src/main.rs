mod browser;
mod canvas;
mod util;
mod ops;


fn main() {
    let mut canvas = canvas::new();
    let mut browser = browser::new();

    util::enter_albuf();

    browser.start_loop(&mut canvas);
}
