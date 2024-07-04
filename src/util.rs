extern crate libc;

use self::libc::{termios, STDIN_FILENO, ECHO, ICANON, ISIG, tcgetattr, tcsetattr, TCSAFLUSH};
use std::mem;

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
