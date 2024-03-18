extern crate libc;

use std::vec::Vec;
use std::str;
use std::io::{stdin, Read};
use std::fs::{read_dir};
use std::path::Path;
use crate::ops::code;
use crate::canvas;
use self::libc::{termios, STDIN_FILENO, ECHO, ICANON, tcgetattr, tcsetattr};
use std::mem;
use std::process::exit;

struct Browser<'canvas> {
    cursor: usize,
    current_dir: Vec<String>, // for display
    past_dir: Vec<String>, // for popping back
    canvas: &'canvas mut canvas::Canvas,
}

impl Browser<'_> {
    fn start_loop(&mut self) {
        self.read_to_current_dir(&String::from("."));
        loop {
            let preview_dir = self.get_preview();
            self.canvas.draw(self.cursor, &self.current_dir, &preview_dir);
            match process_input() {
                code::UP => {self.up();}
                code::DOWN => {self.down();}
                code::LEFT => {self.left();}
                code::RIGHT => {self.right();}
                _ => {self.right();}
            }
        }
    }

    fn get_preview(&self) -> Vec<String>{
        let mut ret: Vec<String> = Vec::new();

        /* if it's a file */
        if Path::new(&self.current_dir[self.cursor]).is_dir() == false {
            return ret
        }

        for entry in read_dir(&self.current_dir[self.cursor]).unwrap() {
            let entry = entry.unwrap();
            ret.push(entry.path().to_str().unwrap().to_string());
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
        let last_dir = self.past_dir.pop();
        self.read_to_current_dir(&last_dir.unwrap());
        self.cursor = 0;
    }

    fn right(&mut self) {
        let dir_under_cursor = &self.current_dir[self.cursor].clone();

        if Path::new(dir_under_cursor).is_dir() == false {
            return
        }

        self.past_dir.push(self.current_dir[self.cursor].clone());
        self.cursor = 0;
        self.read_to_current_dir(dir_under_cursor);
    }

    fn read_to_current_dir(&mut self, path: &String) {

        self.current_dir.clear();

        // println!("len {}", self.current_dir.len());

        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            self.current_dir.push(entry.path().to_str().unwrap().to_string());
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
        tcsetattr(STDIN_FILENO, 0, &termios_);
    }
}

pub fn init() {
    let mut canvas = canvas::init();

    let mut browser = Browser {
        cursor: 0,
        current_dir: Vec::new(),
        past_dir: Vec::new(),
        canvas: &mut canvas,
    };

    raw_input();

    browser.start_loop();
}
