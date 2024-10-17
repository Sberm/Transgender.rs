extern crate libc;

use self::libc::{tcgetattr, tcsetattr, termios, ECHO, ICANON, ISIG, STDIN_FILENO, TCSAFLUSH, c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use crate::ops::code;
use std::fs::File;
use std::io::{self, stdin, BufRead, Read, Write};
use std::mem;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

#[allow(dead_code)]
struct TermSize {
    height: c_ushort,
    width: c_ushort,
    a: c_ushort,
    b: c_ushort,
}

pub fn term_size() -> (usize, usize) {
    unsafe {
        let mut sz: TermSize = mem::zeroed();
        ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut sz as *mut _);
        (sz.height as usize, sz.width as usize)
    }
}

#[allow(dead_code)]
pub fn slp(i: u64) {
    sleep(Duration::from_secs(i));
}

pub fn raw_input() {
    unsafe {
        let mut termios_: termios = mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut termios_);
        termios_.c_lflag &= !(ECHO | ICANON | ISIG);
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios_);
    }
}

pub fn canonical_input() {
    unsafe {
        let mut termios_: termios = mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut termios_);
        termios_.c_lflag |= ECHO | ICANON | ISIG;
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios_);
    }
}

pub fn enter_albuf() {
    raw_input();
    print!("\x1b[?25l"); /* hide cursor */
    print!("\x1b[?1049h"); /* use alternate buffer */
    let _ = io::stdout().flush();
}

pub fn exit_albuf() {
    canonical_input();
    print!("\x1b[?25h"); /* show cursor */
    print!("\x1b[?1049l"); /* switch back to normal screen buffer */
    let _ = io::stdout().flush();
}

fn read_input() -> isize {
    let mut stdin_handle = stdin().lock();
    let mut byte = [0_u8];
    stdin_handle
        .read_exact(&mut byte)
        .expect("Failed to read single byte");
    byte[0] as isize
}

pub fn process_input() -> u8 {
    let mut input = read_input();

    /* arrow keys */
    if input == 27 {
        input = read_input();
        if input == 91 {
            input = read_input();
            if input == 65 {
                return code::UP;
            } else if input == 66 {
                return code::DOWN;
            } else if input == 67 {
                return code::RIGHT;
            } else if input == 68 {
                return code::LEFT;
            } else {
                return code::NOOP;
            }
        } else {
            return code::NOOP;
        }
    }

    /* gg */
    if input == 103 {
        input = read_input();
        if input == 103 {
            return code::TOP;
        }
    }

    match input {
        107 => return code::UP,
        106 => return code::DOWN,
        104 => return code::LEFT,
        108 => return code::RIGHT,
        111 => return code::EXIT_CURSOR,
        10 => return code::EXIT_CURSOR,
        105 => return code::EXIT,
        113 => return code::QUIT,
        47 => return code::SEARCH, /* / */
        71 => return code::BOTTOM, /* G */
        110 => return code::NEXT_MATCH,
        _ => return code::NOOP,
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn print_path(str_: &str) {
    eprintln!("\n{}", str_);
}
