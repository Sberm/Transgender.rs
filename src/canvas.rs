extern crate libc;

use std::mem;
use self::libc::{c_ushort, ioctl, STDOUT_FILENO, TIOCGWINSZ};
use std::any::type_name;
use std::time::Duration;
use std::thread::sleep;

fn slp() {
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

impl Canvas {

    pub fn clear_whole(&self) {
        let mut str_to_draw = String::from("");

        str_to_draw.push_str(&(0..(self.width * self.height) as isize).map(|_| " ").collect::<String>());

        str_to_draw.push_str(&CSI("[1H"));

        print!("{}", str_to_draw);
    }

    fn clear_pixels(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.pixels[i][j] = ' ';
            }
        }
    }

    pub fn draw(&mut self, cursor: usize, current_dir: &Vec<String>, preview_dir: &Vec<String>) {

        /* write pixel */

        let w_t: usize = self.height - 1;
        let w_b: usize = 0;

        let l_w_l: usize = 0;
        let l_w_r: usize = (self.width as isize / 10 * 6 - 1) as usize;

        let r_w_l: usize = l_w_r + 1;
        let r_w_r: usize = self.width - 1;

        /* left side */
        let mut dir_i: usize = 0;
        let mut ch_i: usize = 0;

        self.clear_pixels();

        let mut str_to_draw = String::from("");
        let mut back: bool = false;

        let mut extra: Vec<Extra> = Vec::new();

        /* no content */
        if (current_dir.len() == 0) {
            for line in &self.pixels {
                let tmp_s = line.iter().collect::<String>(); // Vec<char> -> String
                str_to_draw.push_str(&tmp_s); // concat
            }
            str_to_draw.push_str(&CSI("[1H"));
            print!("{}", str_to_draw);
            return
        }

        for i in w_b..w_t {
            let c_a = current_dir[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in l_w_l..l_w_r {
                if ch_i >= c_a.len() {
                    break
                }
                self.set(w_t - i, j, c_a[ch_i]);
                // self.set(w_t - i, j, '*');
                ch_i += 1;
            }

            ch_i = 0;
            dir_i += 1;
            if dir_i >= current_dir.len() {
                break
            }
        }

        let mut offset = 0;

        let highlight = CSI("[0;30m");
        let highlight_bg = CSI("[47m");
        let normal = CSI("[0;37m");
        let normal_bg = CSI("[40m");

        extra.push(Extra{
            pos: cursor * self.width,
            extra_str: highlight.clone(),
        });
        offset += highlight.len();

        extra.push(Extra{
            pos: cursor * self.width + offset,
            extra_str: highlight_bg.clone(),
        });
        offset += highlight_bg.len();

        extra.push(Extra{
            pos: cursor * self.width + r_w_l + offset,
            extra_str: normal.clone(),
        });
        offset += normal.len();

        extra.push(Extra{
            pos: cursor * self.width + r_w_l + offset,
            extra_str: normal_bg.clone(),
        });

        /* right side(preview) */

        dir_i = 0;
        ch_i = 0;

        for i in w_b..w_t {
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
                // self.set(w_t - i, j, '*');
                ch_i += 1;
            }
            ch_i = 0;
            dir_i += 1;
        }

        /* start drawing */

        //for line in &self.pixels {
            //let tmp_s = line.iter().collect::<String>(); // Vec<char> -> String
            //str_to_draw.push_str(&tmp_s); // concat
        //}
        for i in 0..self.height {
            for j in 0..self.width {
                str_to_draw.push(self.pixels[i][j]);
            }
        }

        for ex in &extra {
            str_to_draw.insert_str(ex.pos, &ex.extra_str);
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
        pixels: vec![vec!['*'; w]; h],
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

fn term_size() -> (usize, usize) {
    unsafe {
        let mut sz: TermSize = mem::zeroed();
        ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut sz as *mut _);
        (sz.height as usize, sz.width as usize)
    }
}
