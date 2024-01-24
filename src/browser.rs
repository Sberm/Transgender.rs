use std::vec::Vec;
use std::io::stdin;
#[path = "ops.rs"]
mod ops;

struct Browser {
    entries: u32,
    pointer: u32,
    dir_stack: Vec<u32>,
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
    fn display(&self) {
    }
}

fn process_input() -> u32{
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");
    if input_string == "u" {
        return ops::UP;
    }
    else if input_string == "d" {
        return ops::DOWN;
    }
    else if input_string == "l" {
        return ops::LEFT;
    }
    else if input_string == "r" {
        return ops::RIGHT;
    }
    else {
        return 0;
    }
}

pub fn init() {
    let browser = Browser {
        entries: 1,
        pointer: 2,
        dir_stack: Vec::new()
    };
    loop {
        match process_input() {
            ops::UP => {browser.up();}
            ops::DOWN => {browser.down();}
            ops::LEFT => {browser.left();}
            ops::RIGHT => {browser.right();}
            _ => {browser.right();}
        }
        browser.display()
    }
}
