extern crate libc;

use self::libc::{termios, STDIN_FILENO, ECHO, ICANON, ISIG, tcgetattr, tcsetattr, TCSAFLUSH};
use std::mem;
use std::io::{self, Write};
use std::time::Duration;
use std::thread::sleep;

#[allow(dead_code)]
pub fn slp(i: u64) {
    sleep(Duration::from_secs(i));
}

pub fn raw_input() {
    unsafe {
        let mut termios_:termios = mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut termios_);
        termios_.c_lflag &= !(ECHO | ICANON | ISIG);
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios_);
    }
}

pub fn canonical_input() {
    unsafe {
        let mut termios_:termios = mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut termios_);
        termios_.c_lflag |= ECHO | ICANON | ISIG;
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios_);
    }
}

pub fn enter_albuf() {
    raw_input();
    print!("\x1b[?25l"); // hide cursor
    print!("\x1b[?1049h"); // use alternate buffer
    let _ = io::stdout().flush();
}

pub fn exit_albuf() {
    canonical_input();
    print!("\x1b[?25h"); // show cursor
    print!("\x1b[?1049l"); // switch back to normal screen buffer
    let _ = io::stdout().flush();
}
