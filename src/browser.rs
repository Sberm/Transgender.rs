use std::vec::Vec;
use std::io::stdin;
use std::fs::{read_dir};
use std::path::Path;
use crate::ops::code;
use crate::canvas;

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
            match process_input() {
                code::UP => {self.up();}
                code::DOWN => {self.down();}
                code::LEFT => {self.left();}
                code::RIGHT => {self.right();}
                _ => {self.right();}
            }

            /* for preview */
            let preview_dir = self.get_preview();

            /*
            println!("\ncurrent:");
            for i in 0..self.current_dir.len() {
                println!("{}", self.current_dir[i]);
            }

            println!("\npreview:");
            for i in 0..preview_dir.len() {
                println!("{}", preview_dir[i]);
            }
            */

            self.canvas.draw(self.cursor, &self.current_dir, &preview_dir);
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
    let mut canvas = canvas::init();

    let mut browser = Browser {
        cursor: 0,
        current_dir: Vec::new(),
        past_dir: Vec::new(),
        canvas: &mut canvas,
    };


    browser.start_loop();
}
