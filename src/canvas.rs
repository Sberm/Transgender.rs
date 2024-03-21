extern crate libc;

use std::mem;
use self::libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use std::path::Path;
use std::any::type_name;
use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};

pub fn slp() {
    sleep(Duration::from_secs(3));
}

pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pixels: Vec<Vec<char>>,
}

impl Clone for Canvas {
    fn clone(&self) -> Canvas {
        Canvas {
            height: self.height,
            width: self.width,
            pixels: vec![],
        }
    }
}

fn CSI(s: &str) -> String{
    let mut ret: String = String::from("\x1b");
    ret.push_str(s);
    ret
}

pub struct Extra {
    pos: usize,
    extra_str: String,
}

fn check_if_wide(c: char) -> bool{
    if c as usize > 256 &&
       c != '�' &&
       c != 'ξ' {
        return true;
    } else {
        return false;
    }
}

impl Canvas {

    pub fn clear_whole(&self) {
        let mut str_to_draw = String::from("");

        str_to_draw.push_str(&(0..(self.width * self.height) as isize).map(|_| " ").collect::<String>());

        str_to_draw.push_str(&CSI("[1H"));

        print!("{}", str_to_draw);
    }

    fn clear_pixels(&mut self) {
        let c: char = ' ';
        for i in 0..self.height {
            for j in 0..self.width {
                self.pixels[i][j] = c;
            }
        }
    }

    fn check_insert_highlight(&self, str_to_draw: &mut String, i: usize, j: usize, cursor: usize, r_w_l: usize, is_dir_bool: bool) {

        //let highlight = CSI("[0;30m");
        //let highlight_bg = CSI("[47m");
        //let normal = CSI("[0;37m");
        //let normal_bg = CSI("[40m");
        
        let highlight = CSI("[0;30m");
        let highlight_dir = CSI("[38;5;13m");
        let highlight_bg = CSI("[48;5;175m");
        let normal = CSI("[0;37m");
        let normal_bg = CSI("[48;5;31m");

        if i == cursor && j == 0{
            if is_dir_bool {
                str_to_draw.push_str(&highlight_dir);
            } else {
                str_to_draw.push_str(&highlight);
            }
            str_to_draw.push_str(&highlight_bg);
        } else if i == cursor && j == r_w_l {
            str_to_draw.push_str(&normal);
            str_to_draw.push_str(&normal_bg);
        } else if i == 0 && j == 0 {
            str_to_draw.push_str(&normal_bg);
        }

    }

    pub fn draw(&mut self, cursor: usize, current_dir: &Vec<String>, preview_dir: &Vec<String>, window_start: usize, current_path: &String) {


        (self.height, self.width) = term_size();
        self.pixels = vec![vec![' '; self.width]; self.height];

        /* write pixel */

        let w_t: usize = self.height - 1;
        let w_b: usize = 0;

        let l_w_l: usize = 0;
        let l_w_r: usize = (self.width / 10 * 6 - 1) as usize;

        let r_w_l: usize = l_w_r + 1;
        let r_w_r: usize = self.width - 1;
        let preview_width: usize = self.width - r_w_l;

        let mut dir_i: usize = window_start;
        let mut ch_i: usize = 0;

        self.clear_pixels();

        let mut str_to_draw = String::from("");
        let mut back: bool = false;

        let mut extra: Vec<Extra> = Vec::new();

        /* no content */
        if current_dir.len() == 0 {
            for line in &self.pixels {
                let tmp_s = line.iter().collect::<String>(); // Vec<char> -> String
                str_to_draw.push_str(&tmp_s); // concat
            }
            str_to_draw.push_str(&CSI("[1H"));
            print!("{}", str_to_draw);
            return
        }

        /* left side */
        for i in w_b..=w_t {
            let c_a = current_dir[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in l_w_l..l_w_r {
                if ch_i >= c_a.len() {
                    break
                }
                self.set(w_t - i, j, c_a[ch_i]);
                ch_i += 1;
            }
            dir_i += 1;
            if dir_i >= current_dir.len() {
                break
            }
        }

        let mut offset = 0;

        /* right side(preview) */

        dir_i = 0;
        ch_i = 0;

        for i in w_b..=w_t {
            if dir_i >= preview_dir.len() {
                break
            }
            let c_a = preview_dir[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in r_w_l..r_w_r {
                if ch_i >= c_a.len() {
                    break
                }
                self.set(w_t - i, j, c_a[ch_i]);
                ch_i += 1;
            }
            dir_i += 1;
        }

        let mut i:usize = 0; 
        let mut j:usize = 0;
        let mut font_len:usize = 0;
        let mut do_preview: bool = false;
        let mut is_dir_bool:bool = false;

        is_dir_bool = true;

        loop {
            if i >= self.height {
                break;
            }
            j = 0;
            loop {
                if j >= self.width {
                    break;
                }
                if check_if_wide(self.pixels[i][j]) {
                    font_len += 2;
                } else {
                    font_len += 1;
                } 

                if font_len - 1 > l_w_r && !do_preview {
                    j = r_w_l;
                    font_len = 0;
                    do_preview = true;
                    continue;
                }

                if do_preview && font_len > preview_width {
                    break;
                }

                if i == cursor && j == 0 && Path::new(&(current_path.to_owned() + current_dir[cursor].as_str())).is_dir() == true {
                    is_dir_bool = true;
                }
                self.check_insert_highlight(&mut str_to_draw, i, j, cursor - window_start, r_w_l, is_dir_bool);
                str_to_draw.push(self.pixels[i][j]);
                is_dir_bool = false;

                j += 1;
            }
            font_len = 0;
            i += 1;
            do_preview = false;
        }


        str_to_draw.push_str(&CSI("[1H"));

        print!("{}", str_to_draw);
    }

    fn set(&mut self, i: usize, j: usize, c: char) {
        let i_to_write: i32 = self.height as i32 - 1 - i as i32;
        let j_to_write: usize = j;
        if 0 <= i_to_write && i_to_write < self.height as i32 && j_to_write < self.width {
            self.pixels[i_to_write as usize][j_to_write] = c;
        }
    }

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn init() -> Canvas {
    let (h, w) = term_size();
    let canvas = Canvas {
        height: h,
        width: w,
        pixels: vec![vec![' '; w]; h],
    };

    /* clear space for printing */
    canvas.clear_whole();

    /* hide cursor */
    print!("\x1b[?25l");

    canvas
}

struct TermSize {
    height: c_ushort,
    width: c_ushort,
    a: c_ushort,
    b: c_ushort,
}

pub fn term_size() -> (usize, usize) {
    unsafe {
        let mut sz: TermSize = mem::zeroed();
        ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut sz as *mut _);
        (sz.height as usize, sz.width as usize)
    }
}
