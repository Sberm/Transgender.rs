extern crate libc;

use std::mem;
use self::libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};

pub struct Canvas {
    height: usize,
    width: usize,
    pixels: Vec<Vec<char>>,
}

fn CSI(s: &str) -> String{
    let mut ret: String = String::from("\x1b");
    ret.push_str(s);
    ret
}

impl Canvas {

    fn clear_whole(&self) {

        let mut str_to_draw = String::from("");

        /* clear screen */
        for i in 0..self.height {
            str_to_draw.push_str(&(0..self.width).map(|_| " ").collect::<String>());
        }

        str_to_draw.push_str(&CSI("[1H"));
        print!("{}", str_to_draw);
    }

    pub fn draw(&self) {

        let mut str_to_draw = String::from("");

        for line in &self.pixels {
            let tmp = line.to_owned(); // copy
            let tmp_s = tmp.iter().collect::<String>(); // Vec<char> -> String
            str_to_draw.push_str(&tmp_s); // concat
        }

        str_to_draw.push_str(&CSI("[1H"));

        print!("\x1b[?25l");
        print!("{}", str_to_draw);
    }

    pub fn set(&mut self, i: i32, j: i32, c: char) {
        let i_to_write = self.height as i32 - i;
        let j_to_write = j;
        if 0 <= i_to_write && i_to_write < self.height as i32 &&
            0 <= j_to_write && j_to_write < self.width as i32 {
            self.pixels[i_to_write as usize][j_to_write as usize] = c;
        }
    }

}

pub fn init() -> Canvas {
    let (w, h) = term_size();
    let canvas = Canvas {
        height: h,
        width: w,
        pixels: vec![vec!['*'; w]; h],
    };
    canvas.clear_whole();
    canvas
}

struct TermSize {
    height: c_ushort,
    width: c_ushort,
    a: c_ushort,
    b: c_ushort,
}

fn term_size() -> (usize, usize) {
    unsafe {
        let mut sz: TermSize = mem::zeroed();
        ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut sz as *mut _);
        (sz.height as usize, sz.width as usize)
    }
}
