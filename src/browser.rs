use std::vec::Vec;
use std::io::stdin;
use crate::ops::code;


struct Browser {
    entries: u32,
    pointer: u32,
    dir_stack: Vec<u32>,
    global_v: i32,
}

impl Browser {
    fn up(&self) {
        println!("I'm up");
    }
    fn down(&self) {
        println!("I'm down");

    }
    fn left(&self) {
        println!("I'm left");

    }
    fn right(&self) {
        println!("I'm right");

    }
    fn display(&mut self) {
        self.global_v += 1;
        print!("\x1b[2J");
        print!("\x1b[H");
        println!("global_v {}", self.global_v)
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
        entries: 1,
        pointer: 2,
        dir_stack: Vec::new(),
        global_v: 2
    };
    loop {
        match process_input() {
            code::UP => {browser.up();}
            code::DOWN => {browser.down();}
            code::LEFT => {browser.left();}
            code::RIGHT => {browser.right();}
            _ => {browser.right();}
        }
        browser.display()
    }
}
