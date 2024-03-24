extern crate libc;

use std::vec::Vec;
use std::io::{stdin, Read};
use std::fs::{read_dir,canonicalize};
use std::path::{Path, PathBuf};
use crate::ops::code;
use crate::canvas;
use self::libc::{termios, STDIN_FILENO, ECHO, ICANON, ISIG, tcgetattr, tcsetattr, TCSAFLUSH};
use std::mem;
use std::process::exit;

struct Browser {
    cursor: usize,
    window_start: usize,
    current_dir: Vec<String>, // for display
    past_dir: Vec<String>, // for popping back
    past_cursor: Vec<usize>,
    past_window_start: Vec<usize>,
    current_path: String,
    original_path: String,
}

impl Browser {

    fn init(&mut self) {
        self.read_to_current_dir(&String::from("."));
        let srcdir = PathBuf::from(".");
        let absolute = canonicalize(&srcdir).unwrap().to_str().unwrap().to_string();
        let split = absolute.split("/");
        let mut temp = String::from("");
        for s in split {
            temp += s; temp += "/";
            self.past_dir.push(String::from(temp.clone()));
            self.past_cursor.push(0);
            self.past_window_start.push(0);
        }
        if self.past_dir.len() > 1 {
            self.current_path = self.past_dir.pop().unwrap().clone();
            self.original_path = self.current_path.clone();
            self.past_cursor.pop().unwrap();
            self.past_window_start.pop().unwrap();
        }
        let (h, _) = canvas::term_size();
        self.cursor = (self.current_dir.len() - 1) / 2;
        self.window_start = if self.cursor as isize - h as isize / 2 > 0 {
            self.cursor - h / 2
        } else {
            0
        }
    } 

    fn get_preview(&self) -> Vec<String>{
        let mut ret: Vec<String> = Vec::new();

        if self.current_dir.len() == 0 {
            return ret
        }

        let mut dir_under_cursor = String::from(self.current_path.clone() + &self.current_dir[self.cursor].clone());
        dir_under_cursor += "/";

        /* if it's a file */
        if Path::new(dir_under_cursor.as_str()).is_dir() == false {
            return ret
        }

        if let Ok(entries) = read_dir(dir_under_cursor) {
            for entry in entries {
                let entry = entry.unwrap();
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
        }
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
        let last_dir = self.past_dir.pop().unwrap();
        self.read_to_current_dir(&last_dir);

        let temp = self.current_path.clone();
        let (mut splt, _) = temp.rsplit_once('/').unwrap();
        (_, splt) = splt.rsplit_once('/').unwrap();

        self.current_path = last_dir.clone();

        self.cursor = self.past_cursor.pop().unwrap();
        self.window_start = self.past_window_start.pop().unwrap();

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
        }
    }

    fn read_to_current_dir(&mut self, path: &String) {

        self.current_dir.clear();

        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let entry = entry.unwrap();
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
        canonical_input();

        /* show cursor */
        print!("\x1b[?25h");
        
        /* switch back to normal screen buffer */
        print!("\x1b[?1049l");

        print_file_name(&self.current_path);
        // eprintln!("{}", self.current_path);

        exit(0);
    }

    fn exit_under_cursor(&self) {
        canonical_input();

        /* show cursor */
        print!("\x1b[?25h");
        
        /* switch back to normal screen buffer */
        print!("\x1b[?1049l");

        let dir = format!("{}{}", &self.current_path, &self.current_dir[self.cursor]);

        if Path::new(dir.as_str()).is_dir() == false {
            // eprintln!("{}", self.current_path);
            print_file_name(&self.current_path);
        } else {
            // eprintln!("{}", dir);
            print_file_name(&dir);
        }

        exit(0);
    }
    
    fn quit(&self) {
        canonical_input();

        /* show cursor */
        print!("\x1b[?25h");
        
        /* switch back to normal screen buffer */
        print!("\x1b[?1049l");

        // eprintln!("{}", self.original_path);
        print_file_name(&self.original_path);

        exit(0);
    }
}

fn print_file_name(str_: &String) {
    eprintln!(r"{}", str_);
}

fn read_input() -> isize {
    let mut stdin_handle = stdin().lock();  
    let mut byte = [0_u8];  
    stdin_handle.read_exact(&mut byte).unwrap();
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

    // G
    if input == 71 {
        return code::BOTTOM;
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
        _ => return code::NOOP,
    }
}

fn raw_input() {
    unsafe {
        let mut termios_:termios = mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut termios_);
        termios_.c_lflag &= !(ECHO | ICANON | ISIG);
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios_);
    }
}

fn canonical_input() {
    unsafe {
        let mut termios_:termios = mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut termios_);
        termios_.c_lflag |= ECHO | ICANON | ISIG;
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios_);
    }
}

fn start_loop(browser: &mut Browser, canvas: &mut canvas::Canvas) {
    loop {
        let preview_dir = browser.get_preview();
        canvas.draw(browser.cursor, &browser.current_dir, &preview_dir, browser.window_start, &browser.current_path);
        match process_input() {
            code::UP => {browser.up();}
            code::DOWN => {browser.down();}
            code::LEFT => {browser.left();}
            code::RIGHT => {browser.right();}
            code::EXIT_CURSOR => {browser.exit_under_cursor();}
            code::EXIT => {browser.exit_cur_dir();}
            code::QUIT => {browser.quit();}
            code::TOP => {browser.top();}
            code::BOTTOM => {browser.bottom();}
            _ => {browser.right();}
        }
    }
}

pub fn init() {

    /* use alternate screen buffer */
    print!("\x1b[?1049h");
    
    let mut canvas = canvas::init();

    let mut browser = Browser {
        cursor: 0,
        window_start: 0,
        current_dir: Vec::new(),
        past_dir: Vec::new(),
        past_cursor: Vec::new(),
        past_window_start: Vec::new(),
        current_path: String::from(""),
        original_path: String::from(""),
    };

    browser.init();

    raw_input();

    start_loop(&mut browser, &mut canvas);
}
