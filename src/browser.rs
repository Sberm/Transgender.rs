extern crate libc;

use std::vec::Vec;
use std::io::{stdin, Read};
use std::fs::{read_dir,canonicalize};
use std::path::{Path, PathBuf};
use crate::ops::code;
use crate::canvas;
use self::libc::{termios, STDIN_FILENO, ECHO, ICANON, tcgetattr, tcsetattr, TCSAFLUSH};
use std::mem;
use std::process::exit;

struct Browser {
    cursor: usize,
    current_dir: Vec<String>, // for display
    past_dir: Vec<String>, // for popping back
    past_cursor: Vec<usize>,
    current_path: String,
}

impl Browser {

    fn init(&mut self) {
        let srcdir = PathBuf::from(".");
        let absolute = canonicalize(&srcdir).unwrap().to_str().unwrap().to_string();
        let mut split = absolute.split("/");
        let mut temp = String::from("");
        for s in split {
            temp += s; temp += "/";
            self.past_dir.push(String::from(temp.clone()));
            self.past_cursor.push(0);
        }
        if self.current_path.len() > 0 {
            self.current_path = self.past_dir.pop().unwrap().clone();
            self.past_cursor.pop().unwrap();
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

        for entry in read_dir(&dir_under_cursor).unwrap() {
            let entry = entry.unwrap();
            // ret.push(entry.file_name().into_string().unwrap());
            let s = entry.file_name().into_string();
            match s {
                Ok(v) => {ret.push(v);}
                Err(e) => {ret.push(String::from("[bad string]"));}
            }
        }
        ret
    }

    fn up(&mut self) {
        if self.current_dir.is_empty() == true {
            self.cursor = 0;
            return
        }
        self.cursor = if self.cursor as isize - 1 >= 0 {self.cursor - 1} else {0};
    }

    fn down(&mut self) {
        if self.current_dir.is_empty() == true {
            self.cursor = 0;
            return
        }
        let l = self.current_dir.len();
        self.cursor = if self.cursor + 1 < l {self.cursor + 1} else {l - 1};
    }

    fn left(&mut self) {
        if self.past_dir.is_empty() == true {// < might not be necessary 
            return
        }
        let last_dir = self.past_dir.pop().unwrap();
        self.read_to_current_dir(&last_dir);

        self.current_path = last_dir.clone();

        self.cursor = self.past_cursor.pop().unwrap();
    }

    fn right(&mut self) {

        let mut dir_under_cursor = String::from(self.current_path.clone() + &self.current_dir[self.cursor].clone());
        dir_under_cursor += "/";

        if Path::new(dir_under_cursor.as_str()).is_dir() == false {
            return
        }

        self.past_dir.push(self.current_path.clone());
        self.past_cursor.push(self.cursor);
        self.current_path = dir_under_cursor.clone();
        self.cursor = 0;
        self.read_to_current_dir(&dir_under_cursor);
    }

    fn read_to_current_dir(&mut self, path: &String) {

        self.current_dir.clear();

        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let s = entry.file_name().into_string();
            match s {
                Ok(v) => {self.current_dir.push(v);}
                Err(e) => {self.current_dir.push(String::from("[bad string]"));}
            }
        }
    }
}

fn read_input() -> isize {
    let mut stdin_handle = stdin().lock();  
    let mut byte = [0_u8];  
    stdin_handle.read_exact(&mut byte).unwrap();
    byte[0] as isize
}

fn process_input() -> u32{

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

    /* hjkl */
    if input == 107 {
        return code::UP;
    } else if input == 106 {
        return code::DOWN;
    } else if input == 104 {
        return code::LEFT;
    } else if input == 108 {
        return code::RIGHT;
    } else {
        return code::NOOP;
    }
}

fn raw_input() {
    unsafe {
        let mut termios_:termios = mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut termios_);
        termios_.c_lflag &= !(ECHO | ICANON);
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios_);
    }
}

fn canonical_input() {
    unsafe {
        let mut termios_:termios = mem::zeroed();
        tcgetattr(STDIN_FILENO, &mut termios_);
        termios_.c_lflag |= (ECHO | ICANON);
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &termios_);
    }
}

fn start_loop(browser: &mut Browser, canvas: &mut canvas::Canvas) {
    browser.read_to_current_dir(&String::from("."));
    loop {
        let preview_dir = browser.get_preview();
        canvas.draw(browser.cursor, &browser.current_dir, &preview_dir);
        match process_input() {
            code::UP => {browser.up();}
            code::DOWN => {browser.down();}
            code::LEFT => {browser.left();}
            code::RIGHT => {browser.right();}
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
        current_dir: Vec::new(),
        past_dir: Vec::new(),
        past_cursor: Vec::new(),
        current_path: String::from(""),
    };

    browser.init();

    raw_input();

    ctrlc::set_handler(
        {
            let canvas_static = canvas.clone();
            move || {
                canvas_static.clear_whole();
                canonical_input();

                /* show cursor */
                print!("\x1b[?25h");
                
                /* switch back to normal screen buffer */
                print!("\x1b[?1049l");

                exit(0);
            }
        }
    );

    start_loop(&mut browser, &mut canvas);
}
