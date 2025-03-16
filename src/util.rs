/*═══════════════════════════════════════════════════════════════════════╗
║                         (C)  Howard Chu                                ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

extern crate libc;

use self::libc::{
    c_ushort, ioctl, tcgetattr, tcsetattr, termios, ECHO, ICANON, ISIG, STDIN_FILENO,
    STDOUT_FILENO, TCSAFLUSH, TIOCGWINSZ,
};
use crate::ops::{consts, Op};
use std::env::var;
use std::fs::File;
use std::io::{self, stdin, BufRead, Read, Write};
use std::mem;
use std::path::{Path, PathBuf};
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

#[inline(always)]
pub fn hide_cursor() {
    print!("\x1b[?25l"); // hide cursor
}

#[inline(always)]
pub fn show_cursor() {
    print!("\x1b[?25h"); // show cursor
}

#[allow(dead_code)]
struct TermSize {
    height: c_ushort,
    width: c_ushort,
    a: c_ushort,
    b: c_ushort,
}

/// Get the height and width of the current terminal window
///
/// returns
///  tuple of (height, width)
pub fn term_size() -> (usize, usize) {
    unsafe {
        let mut sz: TermSize = mem::zeroed();
        ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut sz as *mut _);
        (sz.height as usize, sz.width as usize)
    }
}

#[allow(dead_code)]
pub fn _slp(tm: f64) {
    sleep(Duration::from_millis((tm * 1000.0) as u64));
}

#[allow(dead_code)]
pub fn slp(tm: usize) {
    sleep(Duration::from_secs(tm as u64));
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

pub fn reduce_flick() {
    print!("\x1b[0;0m");
    let _ = io::stdout().flush();
}

pub fn enter_albuf() {
    raw_input();
    hide_cursor();
    print!("\x1b[?1049h"); // use alternate buffer
    let _ = io::stdout().flush();
}

pub fn exit_albuf() {
    canonical_input();
    show_cursor();
    print!("\x1b[?1049l"); // switch back to normal screen buffer
    let _ = io::stdout().flush();
}

/// Read a single ascii byte input
///
/// returns
///  ascii byte
fn read_input() -> isize {
    let mut stdin_handle = stdin().lock();
    let mut byte = [0_u8];
    stdin_handle
        .read_exact(&mut byte)
        .expect("Failed to read single byte");
    byte[0] as isize
}

pub fn process_input() -> Op {
    let mut input = read_input();

    if input == 27 {
        // arrow keys
        match read_input() {
            91 => match read_input() {
                65 => return Op::Up,
                66 => return Op::Down,
                67 => return Op::Right,
                68 => return Op::Left,
                _ => return Op::Noop,
            },
            _ => return Op::Noop,
        }
    }

    if input == 4 {
        // ctrl + D
        return Op::PageDown;
    } else if input == 21 {
        // ctrl + U
        return Op::PageUp;
    }

    // gg
    if input == 103 {
        input = read_input();
        if input == 103 {
            return Op::Top;
        }
    }

    match input {
        107 => return Op::Up,         // k
        106 => return Op::Down,       // j
        104 => return Op::Left,       // h
        108 => return Op::Right,      // l
        111 => return Op::ExitCursor, // o
        10 => return Op::ExitCursor,  // Enter
        105 => return Op::Exit,       // i
        113 => return Op::Quit,       // q
        47 => return Op::Search,      // /
        71 => return Op::Bottom,      // G
        110 => return Op::NextMatch,  // n
        78 => return Op::PrevMatch,   // N
        _ => return Op::Noop,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Print path to stderr (although stdin and stdout are switched in ts shell function) for cd to
/// consume.
pub fn print_path(_path: &PathBuf, dest_file: Option<&PathBuf>) {
    let path = String::from(
        _path
            .as_path()
            .to_str()
            .expect("Failed to output file path"),
    ) + "\n";
    if dest_file.is_some() {
        let mut file = File::create(dest_file.unwrap().as_path()).expect(&format!(
            "Failed to write to temporary destination file {}",
            dest_file
                .expect("Failed to unwrap dest file")
                .as_path()
                .to_str()
                .expect("Failed to print the temporary destination file")
        ));
        let mut __empty = file.write_all(path.as_bytes());
        __empty = file.flush();
    } else {
        println!("\n{}", path);
    }
}

pub fn get_theme() -> String {
    if let Ok(home_dir) = var(consts::HOME_VAR) {
        if let Ok(lines) = read_lines(&format!("{}/{}", home_dir, consts::CONFIG_FILE)) {
            for line in lines.flatten() {
                let trimmed = line.replace(" ", "");

                let kv = trimmed.split("=").collect::<Vec<&str>>();
                if kv.len() != 2 {
                    continue;
                }
                if kv[0].eq(consts::THEME_KEY) {
                    return String::from(kv[1]);
                }
            }
        }
    }
    return String::new();
}

/// Read trans config file to get preferred editor
///
/// returns
///  editor name as a String
pub fn get_editor() -> String {
    if let Ok(home_dir) = var(consts::HOME_VAR) {
        if let Ok(lines) = read_lines(&format!("{}/{}", home_dir, consts::CONFIG_FILE)) {
            for line in lines.flatten() {
                let trimmed = line.replace(" ", "");

                let kv = trimmed.split("=").collect::<Vec<&str>>();
                if kv.len() != 2 {
                    continue;
                }
                if kv[0].eq(consts::EDITOR_KEY) {
                    return String::from(kv[1]);
                }
            }
        }
    }
    return String::from(consts::EDITOR);
}

fn parse_utf8(_raw: &[u8], prev_trunc: &Vec<u8>) -> (Vec<char>, Vec<u8>) {
    let mut res: Vec<char> = Vec::new();
    let mut trunc: Vec<u8> = Vec::new();
    let mut bytes_cnt = 0;
    let mut i = 0;
    let mut raw = prev_trunc.clone();
    raw.extend_from_slice(_raw);
    while i < raw.len() {
        let this_byte = raw[i];
        if this_byte == 0 {
            break;
        }
        if this_byte & 0b10000000 == 0 {
            bytes_cnt = 1;
        } else if this_byte & 0b11000000 == 0b11000000 && this_byte & 0b00100000 == 0 {
            bytes_cnt = 2;
        } else if this_byte & 0b11100000 == 0b11100000 && this_byte & 0b00010000 == 0 {
            bytes_cnt = 3;
        } else if this_byte & 0b11110000 == 0b11110000 && this_byte & 0b00001000 == 0 {
            bytes_cnt = 4;
        }
        // truncate bytes
        if i + bytes_cnt > raw.len() {
            trunc.extend_from_slice(&raw[i..raw.len()]);
            break;
        }
        let s = String::from(
            from_utf8(&raw[i..i + bytes_cnt])
                .expect("Failed to convert array of bytes to UTF8 string"),
        );
        i += bytes_cnt;
        res.push(
            s.chars()
                .nth(0)
                .expect("Failed to get the first & only character"),
        );
    }
    (res, trunc)
}

pub fn read_chars_or_op(prev_trunc: &Vec<u8>) -> (Vec<char>, Vec<u8>, Op) {
    let mut raw = [0_u8; 256];
    let mut _stdin = stdin();
    _stdin.read(&mut raw).expect("Failed to read");
    let (char_vec, trunc) = parse_utf8(&mut raw, prev_trunc);
    if char_vec[0] as usize == 27 {
        if char_vec.len() >= 3 && char_vec[1] as usize == 91 {
            match char_vec[2] as usize {
                65 => return (vec![], trunc, Op::Up),
                66 => return (vec![], trunc, Op::Down),
                67 => return (vec![], trunc, Op::Right),
                68 => return (vec![], trunc, Op::Left),
                _ => {}
            };
        }
    }
    (char_vec, trunc, Op::Noop)
}
