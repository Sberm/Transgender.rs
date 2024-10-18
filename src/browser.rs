use crate::canvas;
use crate::ops::{code, consts, Mode, Ops};
use crate::util;
use std::env::var;
use std::fs::read_dir;
use std::io::{stdin, Read};
use std::path::PathBuf;
use std::process::{exit, Command};
use std::str::from_utf8;
use std::vec::Vec;

pub struct Browser {
    cursor: usize,
    window_start: usize,
    current_dir: Vec<String>, /* String instead of PathBuf for display purposes */
    past_dir: Vec<PathBuf>,
    past_cursor: Vec<usize>,
    past_window_start: Vec<usize>,
    current_path: PathBuf,
    original_path: PathBuf,
    mode: Mode,
    search_txt: Vec<char>,
    ops: Ops,
}

impl Browser {
    pub fn init(&mut self) {
        self.read_to_current_dir(&String::from("."));
        let mut srcdir = PathBuf::from(".")
            .canonicalize()
            .expect("Failed to canonicalize current directory");

        loop {
            self.past_dir.push(srcdir.clone());
            self.past_cursor.push(0);
            self.past_window_start.push(0);
            if !srcdir.pop() {
                break;
            }
        }
        self.past_dir = self
            .past_dir
            .clone()
            .into_iter()
            .rev()
            .collect::<Vec<PathBuf>>();

        if self.past_dir.len() > 1 {
            self.current_path = self
                .past_dir
                .pop()
                .expect("Failed to pop the last element from past_dir")
                .clone();
            self.original_path = self.current_path.clone();
            self.past_cursor
                .pop()
                .expect("Failed to pop from past_cursor");
            self.past_window_start
                .pop()
                .expect("Failed to pop from past_window_start");
        }
        self.top();
    }

    pub fn start_loop(&mut self, canvas: &mut canvas::Canvas) {
        loop {
            let preview_dir = self.get_preview();
            canvas.draw(
                self.cursor,
                &self.current_dir,
                &preview_dir,
                self.window_start,
                &self.current_path,
                self.mode,
                &self.search_txt,
            );
            if matches!(self.mode, Mode::SEARCH) {
                self.search();
                continue;
            }
            match util::process_input() {
                code::UP => {
                    self.up();
                }
                code::DOWN => {
                    self.down();
                }
                code::LEFT => {
                    self.left();
                }
                code::RIGHT => {
                    self.right();
                }
                code::EXIT_CURSOR => {
                    self.exit_under_cursor();
                }
                code::EXIT => {
                    self.exit_cur_dir();
                }
                code::QUIT => {
                    self.quit();
                }
                code::TOP => {
                    self.top();
                }
                code::BOTTOM => {
                    self.bottom();
                }
                code::SEARCH => {
                    self.search_txt = Vec::new();
                    self.mode = Mode::SEARCH;
                    util::set_search_cursor();
                }
                code::NEXT_MATCH => {
                    self.next_match();
                }
                _ => {
                    continue;
                }
            }
        }
    }

    fn get_preview(&self) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();

        if self.current_dir.len() == 0 {
            return ret;
        }

        let mut dir_under_cursor = self.current_path.clone();
        dir_under_cursor.push(&self.current_dir[self.cursor]);
        if dir_under_cursor.is_dir() == false {
            return ret;
        }

        if let Ok(entries) = read_dir(&dir_under_cursor) {
            for entry in entries {
                let entry = entry.expect(&format!(
                    "Failed to interate through {}",
                    dir_under_cursor.to_str().unwrap()
                ));
                let s = entry.file_name().into_string();
                match s {
                    Ok(v) => {
                        ret.push(v);
                    }
                    Err(_) => {
                        let str = entry.file_name().to_string_lossy().into_owned();
                        ret.push(str);
                    }
                }
            }
        }
        ret
    }

    fn next_match(&mut self) {
        let mut flag = false;

        if self.cursor >= self.current_dir.len() {
            return;
        }

        for i in self.cursor + 1..self.current_dir.len() {
            let cd = &self.current_dir[i];
            if self.search_txt.len() > cd.chars().collect::<String>().len() {
                continue;
            }
            flag = true;
            for (j, c) in self.search_txt.iter().enumerate() {
                if j < cd.len() {
                    if *c != cd.chars().nth(j).expect("current_dir string out of bound") {
                        flag = false;
                        break;
                    }
                }
            }
            if flag == true {
                self.cursor = i;
                let (h, _) = util::term_size();
                self.window_start = if self.cursor as isize - h as isize / 2 > 0 {
                    self.cursor - h / 2
                } else {
                    0
                };
                break;
            }
        }

        /* start from 0 */
        if flag == false {
            for i in 0..self.cursor {
                let cd = &self.current_dir[i];
                flag = true;
                if self.search_txt.len() > cd.chars().collect::<String>().len() {
                    continue;
                }
                for (j, c) in self.search_txt.iter().enumerate() {
                    if j < cd.len() {
                        if *c != cd.chars().nth(j).expect("current_dir string out of bound") {
                            flag = false;
                            break;
                        }
                    }
                }
                if flag == true {
                    self.cursor = i;
                    let (h, _) = util::term_size();
                    self.window_start = if self.cursor as isize - h as isize / 2 > 0 {
                        self.cursor - h / 2
                    } else {
                        0
                    };
                    break;
                }
            }
        }
    }

    fn search(&mut self) {
        let mut c_bytes = [0u8; 4];
        let mut bytes_cnt: usize = 0;
        stdin()
            .read(&mut c_bytes[0..1])
            .expect("Failed to read the UTF8 prefix");

        if c_bytes[0] & 0b10000000 == 0 {
            bytes_cnt = 1;
        } else if c_bytes[0] & 0b11000000 == 0b11000000 && c_bytes[0] & 0b00100000 == 0 {
            bytes_cnt = 2;
            stdin()
                .read(&mut c_bytes[1..2])
                .expect("Failed to read 1 byte for UTF8 char");
        } else if c_bytes[0] & 0b11100000 == 0b11100000 && c_bytes[0] & 0b00010000 == 0 {
            bytes_cnt = 3;
            stdin()
                .read(&mut c_bytes[1..3])
                .expect("Failed to read 2 bytes for UTF8 char");
        } else if c_bytes[0] & 0b11110000 == 0b11110000 && c_bytes[0] & 0b00001000 == 0 {
            bytes_cnt = 4;
            stdin()
                .read(&mut c_bytes[1..])
                .expect("Failed to read 3 bytes for UTF8 char");
        }

        let s = if bytes_cnt != 0 {
            from_utf8(&c_bytes[0..bytes_cnt]).expect("Failed to convert bytes string to &str")
        } else {
            "�"
        };

        let rc = s
            .chars()
            .nth(0)
            .expect("Failed to get the first & only character");

        if bytes_cnt == 1 {
            /* esc */
            if rc as u8 == 27 {
                self.mode = Mode::NORMAL;
                util::reset_search_cursor();
                return;
            }

            /* backspace */
            if rc as u8 == 127 {
                if self.search_txt.len() > 0 {
                    self.search_txt.pop().expect("search txt(pop) out of bound");
                }
                return;
            }

            /* enter */
            if rc as u8 == 10 {
                /* search */
                self.next_match();
                self.mode = Mode::NORMAL;
                util::reset_search_cursor();
                return;
            }
        }

        self.search_txt.push(rc);
    }

    fn top(&mut self) {
        self.cursor = 0;
        self.window_start = 0;
    }

    fn bottom(&mut self) {
        self.cursor = self.current_dir.len() - 1;
        let (h, _) = util::term_size();
        self.window_start = if self.current_dir.len() as isize - h as isize + 1 > 0 {
            self.current_dir.len() - h
        } else {
            0
        };
    }

    fn up(&mut self) {
        if self.current_dir.is_empty() == true {
            return;
        }
        self.cursor = if self.cursor as isize - 1 >= 0 {
            self.cursor - 1
        } else {
            0
        };
        if self.cursor < self.window_start {
            self.window_start -= 1;
        }
    }

    fn down(&mut self) {
        if self.current_dir.is_empty() == true {
            return;
        }
        let l = self.current_dir.len();
        self.cursor = if self.cursor + 1 < l {
            self.cursor + 1
        } else {
            l - 1
        };

        let (h, _) = util::term_size();

        if self.cursor as isize > (h - 1) as isize && self.cursor > self.window_start + h - 1 {
            self.window_start += 1;
        }
    }

    fn left(&mut self) {
        let current_path_tmp = self.current_path.clone();
        /* root dir '/' */
        if current_path_tmp.file_name() == None {
            return;
        }

        self.current_path = self
            .past_dir
            .pop()
            .expect("Failed to pop from past_dir in when exiting a directory");
        self.read_to_current_dir(&self.current_path.to_str().unwrap().to_string());

        self.cursor = self
            .past_cursor
            .pop()
            .expect("Failed to pop from past_cursor");
        self.window_start = self
            .past_window_start
            .pop()
            .expect("Failed to pop from past_window_start");

        let dir_to_restore = current_path_tmp
            .file_name()
            .expect("Failed to get file name of current path to restore the directory")
            .to_str()
            .expect("Failed to do to_str()");

        /* 0 could be good, but it could be because it was pushed in beginning */
        if self.cursor == 0 {
            let mut i = 0;
            for dir in &self.current_dir {
                if dir.as_str() == dir_to_restore {
                    self.cursor = i;
                    let (h, _) = util::term_size();
                    if self.cursor > self.window_start + h - 1 {
                        self.window_start = self.cursor - h + 1;
                    }
                    break;
                }
                i += 1;
            }
        }
    }

    fn right(&mut self) {
        if self.current_dir.len() <= 0 {
            return;
        }

        let mut dir_under_cursor = self.current_path.clone();
        dir_under_cursor.push(&self.current_dir[self.cursor]);
        if dir_under_cursor.is_dir() == false {
            return;
        }

        self.past_dir.push(self.current_path.clone());
        self.past_cursor.push(self.cursor);
        self.past_window_start.push(self.window_start);
        self.current_path = dir_under_cursor.clone();
        self.read_to_current_dir(&dir_under_cursor.to_str().unwrap().to_string());
        self.top();
    }

    fn read_to_current_dir(&mut self, path: &String) {
        self.current_dir.clear();

        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let entry = entry.expect(&format!("Failed to interate through {}", path));
                let s = entry.file_name().into_string();
                match s {
                    Ok(v) => {
                        self.current_dir.push(v);
                    }
                    Err(_) => {
                        let str = entry.file_name().to_string_lossy().into_owned();
                        self.current_dir.push(str);
                    }
                }
            }
        }
    }

    fn exit_cur_dir(&self) {
        util::exit_albuf();
        util::print_path(&self.current_path.to_str().unwrap());
        exit(0);
    }

    fn exit_under_cursor(&self) {
        let mut dir = self.current_path.clone();
        dir.push(&self.current_dir[self.cursor]);

        util::exit_albuf();

        if dir.is_dir() == false {
            if let Ok(_) = Command::new(&self.ops.editor)
                .arg(dir.to_str().unwrap())
                .status()
            {
            } else {
                Command::new(consts::EDITOR)
                    .arg(dir.to_str().unwrap())
                    .status()
                    .expect(&format!(
                        "Failed to open {} with default editor {}",
                        dir.to_str().unwrap(),
                        consts::EDITOR
                    ));
            }
        } else {
            util::print_path(dir.to_str().unwrap());
            exit(0);
        };

        util::enter_albuf();
    }

    fn quit(&self) {
        util::exit_albuf();

        util::print_path(&self.original_path.to_str().unwrap());

        exit(0);
    }
}

fn get_editor() -> String {
    if let Ok(home_dir) = var(consts::HOME_VAR) {
        if let Ok(lines) = util::read_lines(&format!("{}/{}", home_dir, consts::CONFIG_FILE)) {
            for line in lines.flatten() {
                let trimmed = line.replace(" ", "");

                let kv = trimmed.split("=").collect::<Vec<&str>>();
                if kv.len() != 2 {
                    break;
                }
                if kv[0].eq(consts::EDITOR_KEY) {
                    return String::from(kv[1]);
                }
            }
        }
    }
    return String::from(consts::EDITOR);
}

pub fn new() -> Browser {
    let mut browser = Browser {
        cursor: 0,
        window_start: 0,
        current_dir: Vec::new(),
        past_dir: Vec::new(),
        past_cursor: Vec::new(),
        past_window_start: Vec::new(),
        current_path: PathBuf::from(""),
        original_path: PathBuf::from(""),
        mode: Mode::NORMAL,
        search_txt: Vec::new(),
        ops: Ops {
            editor: get_editor(),
        },
    };

    browser.init();
    browser
}
