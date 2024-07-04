use std::vec::Vec;
use std::io::{stdin, Read, self, Write, BufRead};
use std::fs::{read_dir,canonicalize};
use std::path::{Path, PathBuf};
use crate::ops::{Mode, code, Ops, consts};
use crate::canvas;
use crate::util;
use std::process::{exit, Command};
use std::fs::File;
use std::env::var;

pub struct Browser {
    cursor: usize,
    window_start: usize,
    current_dir: Vec<String>, // for display TODO: change to pathbuf
    past_dir: Vec<String>,
    past_cursor: Vec<usize>,
    past_window_start: Vec<usize>,
    current_path: String,
    original_path: String,
    mode: Mode, 
    search_txt: Vec<char>,
    ops: Ops,
}

impl Browser {
    pub fn init(&mut self) {
        self.read_to_current_dir(&String::from("."));
        let srcdir = PathBuf::from(".");
        let absolute = canonicalize(&srcdir)
                        .expect("Failed to canonicalize").to_str()
                        .expect("Failed when converting to &str").to_string();
        let split = absolute.split("/");
        let mut temp = String::from("");
        for s in split {
            temp += s; temp += "/";
            self.past_dir.push(String::from(temp.clone()));
            self.past_cursor.push(0);
            self.past_window_start.push(0);
        }
        if self.past_dir.len() > 1 {
            self.current_path = self.past_dir.pop()
                                .expect("Failed to pop from past_dir").clone();
            self.original_path = self.current_path.clone();
            self.past_cursor.pop()
                .expect("Failed to pop from past_cursor");
            self.past_window_start.pop()
                .expect("Failed to pop from past_window_start");
        }
        let (h, _) = canvas::term_size();
        self.cursor = (self.current_dir.len() - 1) / 2;
        self.window_start = if self.cursor as isize - h as isize / 2 > 0 {
            self.cursor - h / 2
        } else {
            0
        };
    } 

    pub fn start_loop(&mut self, canvas: &mut canvas::Canvas) {
        loop {
            let preview_dir = self.get_preview();
            canvas.draw(self.cursor, &self.current_dir, &preview_dir, self.window_start, &self.current_path, self.mode, &self.search_txt);
            if matches!(self.mode, Mode::SEARCH) {
                self.search();
                continue;
            }
            match process_input() {
                code::UP => {self.up();}
                code::DOWN => {self.down();}
                code::LEFT => {self.left();}
                code::RIGHT => {self.right();}
                code::EXIT_CURSOR => {self.exit_under_cursor();}
                code::EXIT => {self.exit_cur_dir();}
                code::QUIT => {self.quit();}
                code::TOP => {self.top();}
                code::BOTTOM => {self.bottom();}
                code::SEARCH => {
                    self.search_txt = Vec::new();
                    self.mode = Mode::SEARCH;
                }
                code::NEXT_MATCH => {self.next_match();}
                _ => {self.right();}
            }
        }
    }

    fn get_preview(&self) -> Vec<String>{
        let mut ret: Vec<String> = Vec::new();

        if self.current_dir.len() == 0 {
            return ret
        }

        let mut dir_under_cursor = String::from(self.current_path.clone() + &self.current_dir[self.cursor].clone());
        dir_under_cursor += "/";

        if Path::new(dir_under_cursor.as_str()).is_dir() == false {
            return ret
        }

        if let Ok(entries) = read_dir(&dir_under_cursor) {
            for entry in entries {
                let entry = entry.expect(&format!("Failed to interate through {}", &dir_under_cursor));
                let s = entry.file_name().into_string();
                match s {
                    Ok(v) => {ret.push(v);}
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
            return
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
                let (h, _) = canvas::term_size();
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
                    let (h, _) = canvas::term_size();
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
        let mut stdin_handle = stdin().lock();  
        let mut c = vec![0_u8];  
        stdin_handle.read_exact(&mut c)
            .expect("Failed to read single byte");
        let rc = c[0] as char;

        /* esc */
        if rc as u8 == 27 {
            self.mode = Mode::NORMAL;
            return;
        }

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
            return
        }

        self.search_txt.push(rc);
    }

    fn top(&mut self) {
        self.cursor = 0;
        self.window_start = 0;
    }

    fn bottom(&mut self) {
        self.cursor = self.current_dir.len() - 1;
        let (h, _) = canvas::term_size();
        self.window_start = if self.current_dir.len() as isize - h as isize + 1 > 0 {
            self.current_dir.len() - h
        } else {
            0
        };
    }

    fn up(&mut self) {
        if self.current_dir.is_empty() == true {
            return
        }
        self.cursor = if self.cursor as isize - 1 >= 0 {self.cursor - 1} else {0};
        if self.cursor < self.window_start {
            self.window_start -= 1;
        }
    }

    fn down(&mut self) {
        if self.current_dir.is_empty() == true {
            return
        }
        let l = self.current_dir.len();
        self.cursor = if self.cursor + 1 < l {self.cursor + 1} else {l - 1};

        let (h, _) = canvas::term_size();

        if self.cursor as isize > (h - 1) as isize && self.cursor > self.window_start + h - 1{
            self.window_start += 1;
        }
    }

    fn left(&mut self) {
        if self.past_dir.is_empty() == true {// < might not be necessary 
            return
        }
        let last_dir = self.past_dir.pop()
            .expect("Failed to pop from past_dir");
        self.read_to_current_dir(&last_dir);

        let temp = self.current_path.clone();
        let (mut splt, _) = temp.rsplit_once('/')
            .expect("Failed to rsplit from the last slash");
        (_, splt) = splt.rsplit_once('/')
            .expect("Failed to rsplit from the last slash");

        self.current_path = last_dir.clone();

        self.cursor = self.past_cursor.pop()
            .expect("Failed to pop from past_cursor");
        self.window_start = self.past_window_start.pop()
            .expect("Failed to pop from past_window_start");

        /* 0 could be good, but it could be because it was pushed in beginning */
        if self.cursor == 0 {
            let mut i = 0;
            for dir in &self.current_dir {
                if dir.as_str() == splt {
                    self.cursor = i;
                    let (h, _) = canvas::term_size();
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

        let mut dir_under_cursor = String::from(self.current_path.clone() + &self.current_dir[self.cursor].clone());
        dir_under_cursor += "/";

        if Path::new(dir_under_cursor.as_str()).is_dir() == false {
            return
        }

        self.past_dir.push(self.current_path.clone());
        self.past_cursor.push(self.cursor);
        self.past_window_start.push(self.window_start);
        self.current_path = dir_under_cursor.clone();
        self.read_to_current_dir(&dir_under_cursor);
        let (h, _) = canvas::term_size();
        self.cursor = (self.current_dir.len() - 1) / 2;
        self.window_start = if self.cursor as isize - h as isize / 2 > 0 {
            self.cursor - h / 2
        } else {
            0
        };
    }

    fn read_to_current_dir(&mut self, path: &String) {
        self.current_dir.clear();

        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let entry = entry
                    .expect(&format!("Failed to interate through {}", path));
                let s = entry.file_name().into_string();
                match s {
                    Ok(v) => {self.current_dir.push(v);}
                    Err(_) => {
                        let str = entry.file_name().to_string_lossy().into_owned();
                        self.current_dir.push(str);
                    }
                }
            }
        } 
    }

    fn exit_cur_dir(&self) {
        util::canonical_input();

        /* show cursor */
        print!("\x1b[?25h");
        
        /* switch back to normal screen buffer */
        print!("\x1b[?1049l");

        print_path(&self.current_path);

        exit(0);
    }

    fn exit_under_cursor(&self) {
        let dir = format!("{}{}", &self.current_path, &self.current_dir[self.cursor]);

        util::canonical_input();
        print!("\x1b[?25h"); // show cursor
        print!("\x1b[?1049l"); // exit alternate buffer
        let _ = io::stdout().flush();

        if Path::new(dir.as_str()).is_dir() == false {
            if let Ok(_) = Command::new(&self.ops.editor).arg(&dir).status() {
            } else {
                Command::new(consts::EDITOR).arg(&dir).status()
                    .expect(&format!("Failed to open {} with default editor {}", dir, consts::EDITOR));
            }
        } else {
            print_path(&dir);
            exit(0);
        };

        util::raw_input();
        print!("\x1b[?25l"); // hide cursor
        print!("\x1b[?1049h"); // use alternate buffer
        let _ = io::stdout().flush();
    }
    
    fn quit(&self) {
        util::canonical_input();

        /* show cursor */
        print!("\x1b[?25h");
        
        /* switch back to normal screen buffer */
        print!("\x1b[?1049l");

        print_path(&self.original_path);

        exit(0);
    }
}

fn print_path(str_: &String) {
    eprintln!("\n{}", str_);
}

fn read_input() -> isize {
    let mut stdin_handle = stdin().lock();  
    let mut byte = [0_u8];  
    stdin_handle.read_exact(&mut byte)
        .expect("Failed to read single byte");
    byte[0] as isize
}

fn process_input() -> u8{
    let mut input = read_input();

    /* arrow keys */
    if input == 27 {
        input = read_input();
        if input == 91 {
            input = read_input();
            if input == 65 {
                return code::UP;
            }
            else if input == 66 {
                return code::DOWN;
            }
            else if input == 67 {
                return code::RIGHT;
            }
            else if input == 68 {
                return code::LEFT;
            } else {
                return code::NOOP;
            }
        } else {
            return code::NOOP;
        }
    }

    // gg
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
        115 => return code::EXIT,
        113 => return code::QUIT,
        47 => return code::SEARCH, // /
        71 => return code::BOTTOM, // G
        110 => return code::NEXT_MATCH,
        _ => return code::NOOP,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_editor() -> String {
    if let Ok(home_dir) = var(consts::HOME_VAR) {
        if let Ok(lines) = read_lines(&format!("{}/{}", home_dir, consts::CONFIG_FILE)) {
            for line in lines.flatten() {
                let trimmed = line.replace(" ", "");

                let kv = trimmed.split("=").collect::<Vec<&str>>();
                if kv.len() != 2 {
                    break;
                }
                if kv[0].eq(consts::EDITOR_KEY) {
                    println!("editor in config {}", kv[1]);
                    return String::from(kv[1]);
                }
            }
        }
    }
    return String::from(consts::EDITOR)
}



pub fn new() -> Browser {
    let mut browser = Browser {
        cursor: 0,
        window_start: 0,
        current_dir: Vec::new(),
        past_dir: Vec::new(),
        past_cursor: Vec::new(),
        past_window_start: Vec::new(),
        current_path: String::from(""),
        original_path: String::from(""),
        mode: Mode::NORMAL,
        search_txt: Vec::new(),
        ops: Ops{editor: get_editor()},
    };

    browser.init();
    browser
}
