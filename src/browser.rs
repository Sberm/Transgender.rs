use std::vec::Vec;
use std::io::stdin;
use std::fs::{read_dir};
use crate::ops::code;
use crate::canvas;

struct Browser {
    cursor: usize,
    current_dir: Vec<String>, // for display
    past_dir: Vec<String>, // for popping back
}

impl Browser {
    fn start_loop(&mut self, canvas: &mut canvas::Canvas) {
        self.read_to_current_dir(&String::from("."));
        loop {
            match process_input() {
                code::UP => {self.up();}
                code::DOWN => {self.down();}
                code::LEFT => {self.left();}
                code::RIGHT => {self.right();}
                _ => {self.right();}
            }
            canvas.draw();
        }
    }

    fn up(&mut self) {
        self.cursor = if self.cursor as isize - 1 >= 0 {self.cursor - 1} else {0};
    }

    fn down(&mut self) {
        let l = self.current_dir.len();
        self.cursor = if self.cursor + 1 <= l - 1 {self.cursor + 1} else {l - 1};
    }

    fn left(&mut self) {
        if self.past_dir.len() <= 0 {// < might not be necessary 
            return
        }
        let last_dir = self.past_dir.pop();
        self.read_to_current_dir(&last_dir.unwrap());
        self.cursor = 0;
    }

    fn right(&mut self) {
        let dir_under_cursor = self.current_dir[self.cursor].clone();
        self.past_dir.push(self.current_dir[self.cursor].clone());
        self.cursor = 0;
        self.read_to_current_dir(&dir_under_cursor);
    }

    fn read_to_current_dir(&mut self, path: &String) {
        self.current_dir.shrink_to(0);
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            // println!("{:?}", entry.path());
            self.current_dir.push(entry.path().to_str().unwrap().to_string());
        }
    }
}

fn process_input() -> u32{
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .unwrap();

    input_string = input_string.trim().to_string();

    if input_string.eq("u") {
        return code::UP;
    }
    else if input_string.eq("d") {
        return code::DOWN;
    }
    else if input_string.eq("l") {
        return code::LEFT;
    }
    else if input_string.eq("r") {
        return code::RIGHT;
    }
    else {
        return 0;
    }
}


pub fn init() {
    let mut browser = Browser {
        cursor: 0,
        current_dir: Vec::new(),
        past_dir: Vec::new(),
    };

    let mut canvas = canvas::init();

    browser.start_loop(&mut canvas);
}
