/*═══════════════════════════════════════════════════════════════════════╗
║                          ©  Howard Chu                                 ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

use crate::canvas;
use crate::ops::{code, consts, Mode};
use crate::util;
use regex::RegexBuilder;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::vec::Vec;

/*
 * Directory browser
 */
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
    has_search_input: bool,
    editor: String,
}

impl Browser {
    /*
     *  Construct past directory stack according to the current path
     */
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

    /*
     *  Window display update loop
     */
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
                    self.has_search_input = false;
                    self.mode = Mode::SEARCH;
                    util::set_search_cursor();
                }
                code::NEXT_MATCH => {
                    self.next_match(
                        if self.cursor + 1 < self.current_dir.len() {
                            self.cursor + 1
                        } else {
                            0
                        },
                        false,
                    );
                }
                code::PREV_MATCH => {
                    self.next_match(
                        if self.cursor as isize - 1 >= 0 {
                            self.cursor - 1
                        } else {
                            self.current_dir.len() - 1
                        },
                        true,
                    );
                }
                _ => {
                    continue;
                }
            }
        }
    }

    /*
     *  Get directory content preview window
     *
     * returns
     *  preview window
     */
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

    /*
     *  Set cursor position, centered in the window
     */
    fn set_cursor_pos(&mut self, index: usize) {
        self.cursor = index;
        let (h, _) = util::term_size();
        self.window_start = if self.cursor as isize - h as isize / 2 > 0 {
            self.cursor - h / 2
        } else {
            0
        };
    }

    /*
     *  Next search match, can be a reversed search
     */
    fn next_match(&mut self, start: usize, rev: bool) {
        if self.has_search_input == false {
            return;
        }

        let mut matched = false;
        let mut case_insensitive = true;

        if self.cursor >= self.current_dir.len() {
            return;
        }

        let mut search: String = self.search_txt.iter().collect::<String>();

        /* Check if the case sensitive '\C' is present at the bottom of the search text */
        let len = self.search_txt.iter().count();
        if len > 2 {
            let last_two = self
                .search_txt
                .iter()
                .skip(len - 2)
                .take(2)
                .collect::<String>();
            if last_two.eq("\\C") {
                search = self.search_txt.iter().take(len - 2).collect::<String>();
                case_insensitive = false;
            }
        }

        /* Regex can be invalid while the user is typing */
        let re = match RegexBuilder::new(&search)
            .case_insensitive(case_insensitive)
            .build()
        {
            Ok(re) => re,
            Err(_) => RegexBuilder::new("^$")
                .build()
                .expect("Failed to parse regex for ^$"),
        };

        if rev == false {
            for i in start..self.current_dir.len() {
                if re.is_match(&self.current_dir[i]) {
                    self.set_cursor_pos(i);
                    matched = true;
                    break;
                }
            }
        } else {
            for i in (0..start).rev() {
                if re.is_match(&self.current_dir[i]) {
                    self.set_cursor_pos(i);
                    matched = true;
                    break;
                }
            }
        }

        /* start from 0 */
        if matched == false {
            if rev == false {
                for i in 0..start {
                    if re.is_match(&self.current_dir[i]) {
                        self.set_cursor_pos(i);
                        break;
                    }
                }
            } else {
                for i in (start..self.current_dir.len()).rev() {
                    if re.is_match(&self.current_dir[i]) {
                        self.set_cursor_pos(i);
                        break;
                    }
                }
            }
        }
    }

    fn search(&mut self) {
        let (rc, is_ascii) = match util::read_utf8() {
            Ok((rc, is_ascii)) => (rc, is_ascii),
            Err(_) => ('�', false),
        };

        self.has_search_input = true;

        if is_ascii {
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
                self.mode = Mode::NORMAL;
                util::reset_search_cursor();
                return;
            }
        }

        self.search_txt.push(rc);
        self.next_match(self.cursor, false);
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

        /* So the cursor won't be covered by the bottom line (TODO: But trans still draws that line in Canvas) */
        let display_height = h - 1;

        if self.cursor as isize > (display_height - 1) as isize
            && self.cursor > self.window_start + display_height - 1
        {
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
        let mut index: usize = 0;
        for (i, dir) in self.current_dir.iter().enumerate() {
            if dir.eq(dir_to_restore) {
                self.cursor = i;
                index = i;
                break;
            }
        }
        self.set_cursor_pos(index);
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

    /*
     *  Read the file and directory names in the current directory
     */
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

    /*
     * Goto the directory in the left side window, while quitting trans
     */
    fn exit_cur_dir(&self) {
        util::exit_albuf();
        util::print_path(&self.current_path.to_str().unwrap());
        exit(0);
    }

    /*
     * Goto the directory under the cursor, while quitting trans
     *  or
     * Open the file under the cursor with a text editor
     */
    fn exit_under_cursor(&self) {
        let mut dir = self.current_path.clone();
        dir.push(&self.current_dir[self.cursor]);

        if dir.is_dir() == false {
            if let Ok(_) = Command::new(&self.editor)
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
            util::exit_albuf();
            util::print_path(dir.to_str().unwrap());

            exit(0);
        };

        /* sometimes the editor exits alternate buffer, and enables cursor */
        util::enter_albuf();
        util::hide_cursor();
    }

    fn quit(&self) {
        util::exit_albuf();

        util::print_path(&self.original_path.to_str().unwrap());

        exit(0);
    }
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
        has_search_input: false,
        editor: util::get_editor(),
    };

    browser.init();
    browser
}
