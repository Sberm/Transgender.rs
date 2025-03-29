/*═══════════════════════════════════════════════════════════════════════╗
║                         (C)  Howard Chu                                ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

use crate::canvas;
use crate::ops::{consts, Mode, Op};
use crate::util;
use regex_lite::RegexBuilder;
use std::collections::VecDeque;
use std::ffi::OsString;
use std::fs::read_dir;
use std::iter::Rev;
use std::ops::Range;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::vec::Vec;

const SEARCH_HISTORY_LEN: usize = 256;

struct Opener {
    comm: OsString,
    args: Vec<OsString>,
}

/// Directory browser
pub struct Browser {
    cursor: usize,
    window_start: usize,
    current_dir: Vec<String>, // String instead of PathBuf for display purposes
    past_dir: Vec<PathBuf>,
    past_cursor: Vec<usize>,
    past_window_start: Vec<usize>,
    current_path: PathBuf,
    original_path: PathBuf,
    mode: Mode,
    search_txt: Vec<char>,
    opener_o: Opener,
    opener_enter: Opener,
    dest_file: Option<PathBuf>,
    search_history: VecDeque<Vec<char>>,
    search_history_index: usize,
    trunc: Vec<u8>,
    input_cursor_pos: usize,
}

pub enum UsizeIter {
    Forward(Range<usize>),
    Backward(Rev<Range<usize>>),
}

impl Iterator for UsizeIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            UsizeIter::Forward(range) => range.next(),
            UsizeIter::Backward(range) => range.next(),
        }
    }
}

impl Browser {
    /// Construct past directory stack according to the current path
    pub fn init(&mut self, path: &str) {
        self.read_current_dir(path);

        let mut srcdir = PathBuf::from(path)
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

        self.past_dir.reverse();

        if self.past_dir.len() >= 1 {
            self.current_path = self
                .past_dir
                .pop()
                .expect("Failed to pop the last element from past_dir")
                .clone();

            self.past_cursor
                .pop()
                .expect("Failed to pop from past_cursor");

            self.past_window_start
                .pop()
                .expect("Failed to pop from past_window_start");
        }

        self.top();
    }

    /// Window display update loop
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
                self.input_cursor_pos,
            );

            if matches!(self.mode, Mode::SEARCH) {
                self.search(canvas);
                continue;
            }
            match util::process_input() {
                Op::Up => {
                    self.up();
                }
                Op::Down => {
                    self.down();
                }
                Op::Left => {
                    self.left();
                }
                Op::Right => {
                    self.right();
                }
                Op::ExitCursorO => {
                    self.exit_under_cursor(Op::ExitCursorO);
                }
                Op::ExitCursorEnter => {
                    self.exit_under_cursor(Op::ExitCursorEnter);
                }
                Op::Exit => {
                    self.exit_cur_dir();
                }
                Op::Quit => {
                    self.quit();
                }
                Op::Top => {
                    self.top();
                }
                Op::Bottom => {
                    self.bottom();
                }
                Op::Search => {
                    self.search_txt = Vec::new();
                    self.mode = Mode::SEARCH;
                }
                Op::NextMatch => {
                    self.next_match(
                        if self.cursor + 1 < self.current_dir.len() {
                            self.cursor + 1
                        } else {
                            0
                        },
                        false,
                    );
                }
                Op::PrevMatch => {
                    self.next_match(
                        if self.cursor as isize - 1 >= 0 {
                            self.cursor - 1
                        } else {
                            self.current_dir.len() - 1
                        },
                        true,
                    );
                }
                Op::PageUp => self.pageup(),
                Op::PageDown => self.pagedown(),
                _ => {
                    continue;
                }
            }
        }
    }

    ///  Get directory content preview window
    fn get_preview(&self) -> Vec<String> {
        let empty: Vec<String> = Vec::new();

        if self.current_dir.len() == 0 {
            return empty;
        }

        let mut _dir = self.current_path.clone();
        _dir.push(&self.current_dir[self.cursor]);
        if _dir.is_dir() == false {
            return empty;
        }
        let dir = _dir.to_str().expect("Failed to construct preview path");

        let mut preview = match read_dir(&dir) {
            Ok(entries) => entries
                .map(|_e| match _e {
                    Ok(e) => match e.file_name().into_string() {
                        Ok(filename) => filename,
                        Err(filename_os) => filename_os.to_string_lossy().to_string(),
                    },
                    Err(_) => String::new(),
                })
                .collect::<Vec<String>>(),
            Err(_) => Vec::new(),
        };

        preview.sort_by(|d1, d2| d1.to_lowercase().cmp(&d2.to_lowercase()));
        preview
    }

    /// set cursor position, centered in the window
    fn set_cursor_pos_centered(&mut self, index: usize) {
        // the bottom line will always be there and cover it, the max display height is always
        // terminal height - 1
        let h = util::term_size().0 - 1;

        self.cursor = index;
        self.window_start = if self.cursor as isize - h as isize / 2 > 0 {
            self.cursor - h / 2
        } else {
            0
        };
    }

    fn save_history(&mut self) {
        if self.search_history.len() >= SEARCH_HISTORY_LEN {
            self.search_history.pop_front();
        }
        // don't save an empty line
        if self.search_txt.len() == 0 {
            return;
        }
        if self.search_history_index < self.search_history.len() {
            let string_a = self.search_history[self.search_history_index]
                .clone()
                .into_iter()
                .collect::<String>();
            let string_b = self.search_txt.clone().into_iter().collect::<String>();
            if string_a == string_b {
                // history could be modified by user, that case we save both instead of overwriting the old
                self.search_history.remove(self.search_history_index);
            }
        }
        self.search_history.push_back(self.search_txt.clone());
        self.search_history_index = self.search_history.len(); // out-of-bound on purpose
    }

    /// Next search match, can be a reversed search
    fn next_match(&mut self, start: usize, rev: bool) {
        if self.search_txt.len() == 0 {
            return;
        }

        let mut matched = false;
        let mut case_insensitive = true;

        if self.cursor >= self.current_dir.len() {
            return;
        }

        let mut search: String = self.search_txt.iter().collect::<String>();

        // Check if the case sensitive '\C' is present at the bottom of the search text
        let len = self.search_txt.iter().count();

        if len > 2 {
            let last_two = self
                .search_txt
                .iter()
                .skip(len - 2)
                .take(2)
                .collect::<String>();
            if last_two.eq("\\C") {
                let mut cnt = 0;
                for c in self.search_txt.iter().rev().skip(2) {
                    if *c == '\\' {
                        cnt += 1;
                    } else {
                        break;
                    }
                }
                // U\\\\\C is case sensitive
                if cnt % 2 == 0 {
                    search = self.search_txt.iter().take(len - 2).collect::<String>();
                    case_insensitive = false;
                }
            }
        }

        // Regex can be invalid while the user is typing
        let re = match RegexBuilder::new(&search)
            .case_insensitive(case_insensitive)
            .build()
        {
            Ok(re) => re,
            Err(_) => RegexBuilder::new("^$")
                .build()
                .expect("Failed to parse regex for ^$"),
        };

        let it1 = if rev == false {
            UsizeIter::Forward(start..self.current_dir.len())
        } else {
            UsizeIter::Backward((0..start + 1).rev())
        };

        for i in it1 {
            if re.is_match(&self.current_dir[i]) {
                self.set_cursor_pos_centered(i);
                matched = true;
                break;
            }
        }

        // starts from 0
        if matched == false {
            let it2 = if rev == false {
                UsizeIter::Forward(0..start)
            } else {
                UsizeIter::Backward((start + 1..self.current_dir.len()).rev())
            };

            for i in it2 {
                if re.is_match(&self.current_dir[i]) {
                    self.set_cursor_pos_centered(i);
                    break;
                }
            }
        }
    }

    fn search(&mut self, canvas: &mut canvas::Canvas) {
        let (_chars, trunc, op) = util::read_chars_or_op(&self.trunc);
        self.trunc = trunc;
        // regular text input
        if op == Op::Noop && _chars.is_some() {
            let mut chars = _chars.expect("unwrap char vec failed");
            let first_char = chars[0] as usize;
            // for example, Ctrl + C = 3, Ctrl + I = 9 these characters cannot be displayed, yet
            // they will take space in the search text
            if first_char < 32 {
                // escape or return (line feed)
                if first_char != 27 && first_char != 10 {
                    return;
                }
            }
            if first_char == 27 {
                // esc
                self.mode = Mode::NORMAL;
                self.search_history_index = self.search_history.len();
                self.input_cursor_pos = 0;
                canvas.reset_bottom_bar();
                return;
            } else if first_char == 127 {
                // backspace
                if self.input_cursor_pos >= 1 {
                    self.search_txt.remove(self.input_cursor_pos - 1);
                    // this is added so user knows what's being deleted.
                    // there could be a problem when the lengths of the UTF-8 characters (the one
                    // being deleted and the new one added on the left for alignment) are not equal,
                    // making the cursor all over the place.
                    if canvas.bottom_start > 0 {
                        canvas.bottom_start -= 1;
                    }
                    self.input_cursor_pos -= 1;
                }
            } else if first_char == 10 {
                // enter
                self.save_history();
                self.mode = Mode::NORMAL;
                self.input_cursor_pos = 0;
                canvas.reset_bottom_bar();
                return;
            } else {
                // input characters
                let mut search_txt_inserted = vec![];
                let chars_len = chars.len();
                search_txt_inserted.extend_from_slice(&self.search_txt[0..self.input_cursor_pos]);
                search_txt_inserted.append(&mut chars);
                search_txt_inserted.extend_from_slice(&self.search_txt[self.input_cursor_pos..]);
                self.search_txt = search_txt_inserted;
                self.input_cursor_pos += chars_len;
            }
        } else if op == Op::Up || op == Op::Down {
            // search history
            // ok to scroll: 1. at the end of history, search input is empty
            //               2. currently in the process of scolling through history
            //
            // when user is not browsing history, search_history_index should be
            // search_history.len()
            if (self.search_history_index == self.search_history.len()
                && self.search_txt.len() == 0)
                || self.search_history_index < self.search_history.len()
            {
                match op {
                    Op::Up => {
                        if self.search_history_index > 0 {
                            self.search_history_index -= 1;
                        }
                    }
                    Op::Down => {
                        if self.search_history_index < self.search_history.len() {
                            self.search_history_index += 1;
                        }
                    }
                    _ => {}
                }
                if self.search_history_index < self.search_history.len() {
                    self.search_txt = self.search_history[self.search_history_index].clone();
                } else {
                    self.search_txt = Vec::new();
                }
                self.input_cursor_pos = self.search_txt.len();
            }
        } else if op == Op::Left || op == Op::Right {
            // left and right arrow
            match op {
                Op::Left => {
                    if self.input_cursor_pos > 0 {
                        self.input_cursor_pos -= 1;
                    }
                }
                Op::Right => {
                    if self.input_cursor_pos + 1 <= self.search_txt.len() {
                        self.input_cursor_pos += 1;
                    }
                }
                _ => {}
            }
        }
        self.next_match(self.cursor, false);
    }

    fn top(&mut self) {
        self.cursor = 0;
        self.window_start = 0;
    }

    fn bottom(&mut self) {
        self.set_cursor_pos_centered(self.current_dir.len() - 1);
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

        let max_len = self.current_dir.len();
        self.cursor = if self.cursor + 1 < max_len {
            self.cursor + 1
        } else {
            max_len - 1
        };

        let display_height = util::term_size().0 - 1;

        if (self.cursor) as isize > (self.window_start + display_height - 1) as isize {
            self.window_start += 1;
        }
    }

    fn left(&mut self) {
        let child = self.current_path.clone();
        // for example, root dir '/' doesn't have a file name
        if child.file_name() == None {
            return;
        }

        // access the parent dir and read its content
        self.current_path = self
            .past_dir
            .pop()
            .expect("Failed to pop from past_dir in when exiting a directory");
        self.read_current_dir(&self.current_path.to_str().unwrap().to_string());

        self.cursor = self
            .past_cursor
            .pop()
            .expect("Failed to pop from past_cursor");

        self.window_start = self
            .past_window_start
            .pop()
            .expect("Failed to pop from past_window_start");

        let child_filename_str = child
            .file_name()
            .expect("Failed to get file name of current path to restore the directory")
            .to_str()
            .expect("Failed to do to_str()");

        // 0 is set in init()
        let mut index: usize = 0;

        // find the child dir in parent directories
        for (i, dir) in self.current_dir.iter().enumerate() {
            if dir.eq(child_filename_str) {
                self.cursor = i;
                index = i;
                break;
            }
        }

        self.set_cursor_pos_centered(index);
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
        self.read_current_dir(
            &dir_under_cursor
                .to_str()
                .expect("Failed to call to_str() for the dir under cursor")
                .to_string(),
        );
        self.top();
    }

    /// Read the file and directory names in the current directory
    fn read_current_dir(&mut self, path: &str) {
        self.current_dir = match read_dir(path) {
            Ok(entries) => entries
                .map(|_e| match _e {
                    Ok(e) => match e.file_name().into_string() {
                        Ok(filename) => filename,
                        Err(filename_os) => filename_os.to_string_lossy().to_string(),
                    },
                    Err(_) => String::new(),
                })
                .collect::<Vec<String>>(),
            Err(_) => Vec::new(),
        };

        self.current_dir
            .sort_by(|d1, d2| d1.to_lowercase().cmp(&d2.to_lowercase()));
    }

    /// quit trans and goto the directory in the left window
    fn exit_cur_dir(&self) {
        util::exit_albuf();
        util::print_path(&self.current_path, (&self.dest_file).as_ref());
        exit(0);
    }

    /// quit trans and goto the directory under the cursor
    ///  or
    /// open the file under the cursor with opener command
    fn exit_under_cursor(&self, op: Op) {
        let mut dir = self.current_path.clone();
        dir.push(&self.current_dir[self.cursor]);
        let opener = match op {
            Op::ExitCursorO => &self.opener_o,
            Op::ExitCursorEnter => &self.opener_enter,
            _ => &self.opener_o,
        };

        if dir.is_dir() == false {
            // reduce color flicking (the flicking color is the bottom bar color)
            util::reduce_flick();

            if let Ok(_) = Command::new(&(*opener).comm)
                .args(&(*opener).args)
                .arg(dir.to_str().unwrap())
                .status()
            {
                // empty, successfully opened with opener
            } else {
                Command::new(consts::OPENER)
                    .arg(dir.to_str().unwrap())
                    .status()
                    .expect(&format!(
                        "Failed to open {} with default opener {}",
                        dir.to_str().unwrap(),
                        consts::OPENER
                    ));
            }
        } else {
            util::exit_albuf();
            util::print_path(&dir, (&self.dest_file).as_ref());
            exit(0);
        };

        // when an opener exits, it also exits the alternate buffer, and enables cursor, need to
        // stay in albuf and hide cursor in trans
        util::enter_albuf();
        util::hide_cursor();
    }

    fn quit(&self) {
        util::exit_albuf();
        util::print_path(&self.original_path, (&self.dest_file).as_ref());
        exit(0);
    }

    fn pageup(&mut self) {
        if self.current_dir.is_empty() == true {
            return;
        }

        let height = util::term_size().0 - 1;
        let half_page = height / 2;

        let pos = if self.cursor as isize - (half_page as isize) < 0 {
            0
        } else {
            self.cursor - half_page
        };

        self.set_cursor_pos_centered(pos);
    }

    fn pagedown(&mut self) {
        if self.current_dir.is_empty() == true {
            return;
        }

        let height = util::term_size().0 - 1;
        let half_page = height / 2;

        let pos = if self.cursor + half_page >= self.current_dir.len() {
            self.current_dir.len() - 1
        } else {
            self.cursor + half_page
        };

        self.set_cursor_pos_centered(pos);
    }
}

impl Opener {
    fn new(comm: OsString, args: Option<Vec<OsString>>) -> Opener {
        let mut opener = Opener {
            comm: comm,
            args: vec![],
        };
        if args.is_some() {
            opener.args = args.unwrap()
        }
        opener
    }
}

pub fn new(path: &str, dest_file: Option<String>) -> Browser {
    let (comm_o, args_o) = util::get_opener(Op::ExitCursorO);
    let (comm_enter, args_enter) = util::get_opener(Op::ExitCursorEnter);

    let mut browser = Browser {
        cursor: 0,
        window_start: 0,
        current_dir: Vec::new(),
        past_dir: Vec::new(),
        past_cursor: Vec::new(),
        past_window_start: Vec::new(),
        current_path: PathBuf::new(),
        original_path: PathBuf::from("."),
        mode: Mode::NORMAL,
        search_txt: Vec::new(),
        opener_o: Opener::new(comm_o, args_o),
        opener_enter: Opener::new(comm_enter, args_enter),
        dest_file: (|dest_file| match dest_file {
            Some(df) => Some(PathBuf::from(&df)),
            None => None,
        })(dest_file),
        search_history: VecDeque::new(),
        search_history_index: 0,
        trunc: Vec::new(),
        input_cursor_pos: 0,
    };
    browser.init(&path);
    browser
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use std::fs::File;
    use std::fs::{create_dir, create_dir_all, remove_dir_all};
    use std::ops::Drop;

    struct CleanupDir {
        dir: String,
    }

    impl Drop for CleanupDir {
        fn drop(&mut self) {
            if remove_dir_all(&format!("/tmp/{}", &self.dir)).is_err() {
                println!("remove dir failed");
            }
        }
    }

    #[test]
    fn test_browser_init() {
        let mut rand = util::test::Rand::new();
        let mut tmp_dirs: Vec<String> = vec![];
        let depth = 4;
        for _ in 0..depth {
            tmp_dirs.push(format!("ts-test-{}", &rand.random_str()));
        }
        // assert_eq! (when failed) and the end of function will call drop() for cleanup
        let _cd = CleanupDir {
            dir: String::from(&tmp_dirs[0]),
        };

        let temp_dir = format!(
            "/tmp/{}/{}/{}/{}",
            tmp_dirs[0], tmp_dirs[1], tmp_dirs[2], tmp_dirs[3]
        );
        create_dir_all(&temp_dir).expect(&format!("create dir {} failed", &temp_dir));
        println!("created dir {}", &temp_dir);
        let b = new(&temp_dir, None); // browser::new()
        let past_dir = &b.past_dir;
        // add / and /tmp, but the last directory is not in past_dir
        if depth + 2 - 1 != past_dir.len() {
            assert_eq!(depth + 1, past_dir.len());
        }
        let ans = ["", "tmp", &tmp_dirs[0], &tmp_dirs[1], &tmp_dirs[2]];
        for i in 0..past_dir.len() {
            let _tmp = past_dir[i].as_path().file_name();
            if _tmp.is_some() {
                let past_dir_name = _tmp.unwrap().to_str().expect("to_str failed");
                println!("comparing tmp_dirs {} past_dir {}", ans[i], past_dir_name);
                assert_eq!(ans[i], past_dir_name);
            }
            // root dir returns None
        }
    }

    #[test]
    fn test_get_preview() {
        let mut rand = util::test::Rand::new();
        let files = [
            format!("f-{}", rand.random_str()),
            format!("f-{}", rand.random_str()),
            format!("f-{}", rand.random_str()),
            format!("f-{}", rand.random_str()),
            format!("f-{}", rand.random_str()),
            format!("f-{}", rand.random_str()),
        ];
        let dirs = [
            format!("d-{}", rand.random_str()),
            format!("d-{}", rand.random_str()),
            format!("d-{}", rand.random_str()),
            format!("d-{}", rand.random_str()),
            format!("d-{}", rand.random_str()),
            format!("d-{}", rand.random_str()),
        ];
        let root_dir = format!("ts-test-{}", rand.random_str());
        println!("creating root dir {}", &root_dir);
        let _r = create_dir(&format!("/tmp/{}", &root_dir));
        if _r.is_err() {
            panic!("create root dir failed {:?}", _r.unwrap());
        }
        let _cd = CleanupDir {
            dir: String::from(&root_dir),
        };

        let mut dirs_files: HashSet<String> = HashSet::new();
        for dir in dirs.iter() {
            let tmp = format!("/tmp/{}/{}", root_dir, dir);
            let r = create_dir(&tmp);
            if r.is_err() {
                panic!("create directory failed");
            }
            dirs_files.insert(dir.to_string());
        }
        for file in files.iter() {
            let tmp = format!("/tmp/{}/{}", root_dir, file);
            let r = File::create(&tmp);
            if r.is_err() {
                panic!("create file failed");
            }
            dirs_files.insert(file.to_string());
        }
        let mut b = new("/tmp", None);
        let mut cur_pos = 0;
        for (i, cd) in b.current_dir.iter().enumerate() {
            if cd == &root_dir {
                cur_pos = i;
                break;
            }
        }
        b.set_cursor_pos_centered(cur_pos);
        let preview = b.get_preview();
        let mut dedup: HashSet<String> = HashSet::new();
        for p in preview {
            println!("preview {}", p);
            if !dirs_files.contains(&p) {
                panic!("incorrect preview");
            }
            dedup.insert(p);
        }
        assert_eq!(dedup.len(), files.len() + dirs.len());
    }
}
