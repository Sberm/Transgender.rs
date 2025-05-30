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
use std::ffi::OsString;
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

struct TermSize {
    height: c_ushort,
    width: c_ushort,
    _a: c_ushort,
    _b: c_ushort,
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
        107 => return Op::Up,             // k
        106 => return Op::Down,           // j
        104 => return Op::Left,           // h
        108 => return Op::Right,          // l
        111 => return Op::ExitCursorO,    // o
        10 => return Op::ExitCursorEnter, // Enter
        105 => return Op::Exit,           // i
        113 => return Op::Quit,           // q
        47 => return Op::Search,          // /
        63 => return Op::RevSearch,       // ?
        71 => return Op::Bottom,          // G
        110 => return Op::NextMatch,      // n
        78 => return Op::PrevMatch,       // N
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
    let path = String::from(_path.to_str().expect("Failed to output file path")) + "\n";
    if dest_file.is_some() {
        let mut file = File::create(dest_file.unwrap()).expect(&format!(
            "Failed to write to temporary destination file {}",
            dest_file
                .expect("Failed to unwrap dest file")
                .to_str()
                .expect("Failed to print the temporary destination file")
        ));
        let mut __empty = file.write_all(path.as_bytes());
        __empty = file.flush();
    } else {
        println!("\n{}", path);
    }
}

pub fn get_theme(_config_path: Option<&str>) -> String {
    let config_path = if _config_path.is_some() {
        _config_path.unwrap().to_string()
    } else {
        let home_dir = var(consts::HOME_VAR).expect("failed to get HOME env");
        format!("{}/{}", home_dir, consts::CONFIG_FILE)
    };
    if let Ok(lines) = read_lines(&config_path) {
        for line in lines.flatten() {
            let kv = line.split("=").collect::<Vec<&str>>();
            if kv.len() != 2 {
                continue;
            }
            if kv[0].trim().to_lowercase() == consts::THEME_KEY {
                return kv[1].trim().to_lowercase();
            }
        }
    }
    return String::new();
}

/// Read trans config file to get preferred opener
///
/// returns
///  opener's command with arguments
pub fn get_opener(op: Op, _config_path: Option<&str>) -> (OsString, Option<Vec<OsString>>) {
    let key = match op {
        Op::ExitCursorO => Some(consts::O_KEY),
        Op::ExitCursorEnter => Some(consts::ENTER_KEY),
        _ => None,
    };
    let config_path = if _config_path.is_some() {
        _config_path.unwrap().to_string()
    } else {
        let home_dir = var(consts::HOME_VAR).expect("failed to get HOME env");
        format!("{}/{}", home_dir, consts::CONFIG_FILE)
    };
    let mut comm = OsString::from(consts::OPENER);
    let mut args: Option<Vec<OsString>> = None;
    if let Ok(lines) = read_lines(&config_path) {
        for line in lines.flatten() {
            let kv = line.split("=").collect::<Vec<&str>>();
            if kv.len() != 2 {
                continue;
            }
            if key.is_some() {
                if kv[0].trim().to_lowercase() == key.unwrap() {
                    // "key =.*"
                    let comm_op = kv[1].trim().split(" ").collect::<Vec<&str>>();
                    if comm_op.len() != 0 {
                        // key = code.*
                        comm = OsString::from(comm_op[0]);
                        args = Some(
                            comm_op
                                .into_iter()
                                .skip(1)
                                .map(|x| OsString::from(x))
                                .collect(),
                        );
                        return (comm, args);
                    }
                }
            }
            if kv[0].trim().to_lowercase() == consts::EDITOR_KEY
                || kv[0].trim().to_lowercase() == consts::OPENER_KEY
            {
                let comm_op = kv[1].trim().split(" ").collect::<Vec<&str>>();
                if comm_op.len() != 0 {
                    // can be overridden by 'o' or 'enter'
                    comm = OsString::from(comm_op[0]);
                    args = Some(
                        comm_op
                            .into_iter()
                            .skip(1)
                            .map(|x| OsString::from(x))
                            .collect(),
                    );
                }
            }
        }
    }
    (comm, args)
}

/// Parse a byte array to a vector of chars
///
/// returns
///  the parsed char array along with trailing truncated bytes for the next parsing
fn parse_utf8(_raw: &[u8], prev_trunc: &Vec<u8>) -> (Vec<char>, Vec<u8>) {
    let mut res: Vec<char> = Vec::new();
    let mut trunc: Vec<u8> = Vec::new();
    let mut bytes_cnt = 0;
    let mut i = 0;
    let mut raw = prev_trunc.clone();
    raw.extend(_raw);
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
            trunc.extend(&raw[i..raw.len()]);
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

/// read characters
///
/// returns
///  either a vector of characters or an Opcode, along with trailing truncated bytes for the next
///  parsing
pub fn read_chars_or_op(prev_trunc: &Vec<u8>) -> (Option<Vec<char>>, Vec<u8>, Op) {
    let mut raw = [0_u8; 256];
    let mut _stdin = stdin();
    _stdin.read(&mut raw).expect("Failed to read");
    let (char_vec, trunc) = parse_utf8(&mut raw, prev_trunc);
    if char_vec.is_empty() {
        return (None, trunc, Op::Noop);
    }
    if char_vec[0] as usize == 27 {
        if char_vec.len() >= 3 && char_vec[1] as usize == 91 {
            match char_vec[2] as usize {
                65 => return (None, trunc, Op::Up),
                66 => return (None, trunc, Op::Down),
                67 => return (None, trunc, Op::Right),
                68 => return (None, trunc, Op::Left),
                _ => {}
            };
        }
    }
    (Some(char_vec), trunc, Op::Noop)
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::fs::{create_dir, exists, remove_dir_all, remove_file, File};
    use std::io::Write;
    use std::time::SystemTime;

    pub struct Rand {
        pub x_pre: Option<u128>,
        pub y_pre: Option<u128>,
    }

    const M: u128 = 7919;
    const A: u128 = 7907;
    const C: u128 = 7901;

    impl Rand {
        pub fn new() -> Rand {
            Rand {
                x_pre: None,
                y_pre: None,
            }
        }

        pub fn rand_uint(&mut self, min: usize, max: usize) -> usize {
            assert!(max >= min);
            let mut x = if self.x_pre.is_none() {
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("empty duration")
                    .as_nanos()
            } else {
                self.x_pre.unwrap()
            };
            x = (A * x + C) % ((max - min) as u128);
            self.x_pre = Some(x);
            return x as usize + min;
        }

        pub fn rand_str(&mut self) -> String {
            let len = 16;
            let mut rand_str = Vec::from([' '; 16]);
            let alnums = [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5',
                '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            ];
            let mut x = if self.x_pre.is_none() {
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("empty duration")
                    .as_nanos()
            } else {
                self.x_pre.unwrap()
            };
            let mut y = if self.y_pre.is_none() {
                (x * x) % M
            } else {
                self.y_pre.unwrap()
            };
            let al_len = alnums.len() as u128;
            for i in 0..len {
                rand_str[i] = alnums[((x + y) % al_len) as usize];
                x = (A * x + C) % M;
                y = (A * y + C) % M;
            }
            self.x_pre = Some(x);
            self.y_pre = Some(y);
            return rand_str.into_iter().collect::<String>();
        }
    }

    pub struct CleanupDir {
        pub dir: String,
    }

    impl Drop for CleanupDir {
        fn drop(&mut self) {
            if remove_dir_all(&self.dir).is_err() {
                panic!("remove dir {} failed", self.dir);
            }
        }
    }

    pub struct CleanupFile {
        pub file: String,
    }

    impl Drop for CleanupFile {
        fn drop(&mut self) {
            if remove_file(&self.file).is_err() {
                println!("remove file {} failed", self.file);
            }
        }
    }

    fn random_dirs_nfiles() -> (Vec<String>, Vec<String>) {
        let mut rand = Rand::new();
        let file_nr = rand.rand_uint(2, 15);
        let dir_nr = rand.rand_uint(2, 15);
        let mut files = vec![];
        let mut dirs = vec![];
        for _ in 0..file_nr {
            files.push(format!("f-{}", rand.rand_str()));
        }
        for _ in 0..dir_nr {
            dirs.push(format!("d-{}", rand.rand_str()));
        }
        return (files, dirs);
    }

    // /tmp/ts-test-XXX
    //                 /f-XXX
    //                 /f-XXX
    //                 /d-XXX
    //                 /d-XXX
    //
    // on MacOS, /tmp is a symlink to /private/tmp
    // files, dirs, root_dir, cleanup
    pub fn random_dir_wcontent() -> (Vec<String>, Vec<String>, String, CleanupDir) {
        let mut rand = Rand::new();
        let (files, dirs) = random_dirs_nfiles();
        let mut root_dir: String;
        loop {
            root_dir = format!("ts-test-{}", rand.rand_str());
            let _root_dir = format!("/tmp/{}", &root_dir);
            if !exists(&_root_dir).expect("don't know if tmp dir exists") {
                if create_dir(&format!("/tmp/{}", &root_dir)).is_err() {
                    continue;
                }
                break;
            }
        }
        let _cd = CleanupDir {
            dir: String::from("/tmp/".to_owned() + &root_dir),
        };
        for dir in dirs.iter() {
            let tmp = format!("/tmp/{}/{}", root_dir, dir);
            let r = create_dir(&tmp);
            if r.is_err() {
                panic!("create directory failed");
            }
        }
        for file in files.iter() {
            let tmp = format!("/tmp/{}/{}", root_dir, file);
            let r = File::create(&tmp);
            if r.is_err() {
                panic!("create file failed");
            }
        }
        (files, dirs, root_dir, _cd)
    }

    pub fn mktemp_conf() -> (String, Option<File>) {
        let mut rand = Rand::new();
        let mut conf;
        let file: Option<File>;
        loop {
            let _conf = format!("ts-temp-conf-{}", rand.rand_str());
            conf = "/tmp/".to_owned() + &_conf;
            if !exists(&conf).expect("don't know if the config file exists") {
                let _f = File::create(&conf);
                if _f.is_err() {
                    continue;
                }
                file = Some(_f.unwrap());
                break;
            }
        }
        (conf, file)
    }

    #[test]
    fn test_get_theme() {
        let (conf, _file) = mktemp_conf();
        if _file.is_none() {
            panic!("failed to create temp file");
        }
        let mut file = _file.unwrap();
        let _cf = CleanupFile { file: conf.clone() };
        let target = "lucius";
        let _ = file.write(&("theme = ".to_owned() + target).into_bytes());
        assert_eq!(get_theme(Some(&conf)), target);
    }

    #[test]
    fn test_get_opener() {
        let (conf, _file) = mktemp_conf();
        if _file.is_none() {
            panic!("failed to create temp file");
        }
        let mut file = _file.unwrap();
        let _cf = CleanupFile { file: conf.clone() };
        let _ = file.write(&("o = vim -R -es -m -b -A -V -D -q ".to_owned() + &conf).into_bytes());
        let (comm, args) = get_opener(Op::ExitCursorO, Some(&conf));
        assert_eq!(comm, "vim");
        assert_eq!(
            args.expect("failed to get args for assertion"),
            ["-R", "-es", "-m", "-b", "-A", "-V", "-D", "-q", &conf]
        );
    }

    #[test]
    fn test_parse_utf8() {
        let raw: [u8; 4] = [232, 145, 137, 232];
        let (cs, trunc) = parse_utf8(&raw, &Vec::new());
        assert_eq!(cs, ['葉']);
        assert_eq!(trunc, [232]);
    }
}
