extern crate libc;

use crate::ops::{consts, Mode, Theme};
use crate::utf8;
use crate::util;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pixels: Vec<Vec<char>>,
    highlight: String,
    highlight_dir: String,
    highlight_bg: String,
    normal: String,
    normal_bg: String,
}

impl Clone for Canvas {
    fn clone(&self) -> Canvas {
        Canvas {
            height: self.height,
            width: self.width,
            pixels: Vec::new(),
            highlight: String::new(),
            highlight_dir: String::new(),
            highlight_bg: String::new(),
            normal: String::new(),
            normal_bg: String::new(),
        }
    }
}

fn csi(s: &str) -> String {
    let mut ret: String = String::from("\x1b[");
    ret.push_str(s);
    ret
}

impl Canvas {
    fn bottom_line_slice(&self, s: &str) -> usize {
        /* Make sure bottom line doesn't overflow */
        let mut display_len: usize = 0;
        let mut slice_to: usize = 0;

        for c in s.chars() {
            if self.is_wide(c.clone() as usize) {
                display_len += 2;
            } else {
                display_len += 1;
            }
            if display_len > self.width - 1 {
                break;
            }
            slice_to += 1;
        }

        slice_to
    }

    fn is_wide(&self, c: usize) -> bool {
        let mut l = 0;
        let mut r = utf8::UTF8_TBL.len() - 1;

        while l < r {
            let m = (l + r) / 2;
            if utf8::UTF8_TBL[m].l <= c && utf8::UTF8_TBL[m].r >= c {
                return utf8::UTF8_TBL[m].is_wide;
            } else if utf8::UTF8_TBL[m].l > c {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        return utf8::UTF8_TBL[l].is_wide;
    }

    fn clear_pixels(&mut self) {
        let c: char = ' ';
        for i in 0..self.height {
            for j in 0..self.width {
                self.pixels[i][j] = c;
            }
        }
    }

    fn check_insert_highlight(
        &self,
        str_to_draw: &mut String,
        i: usize,
        j: usize,
        cursor: usize,
        r_w_l: usize,
        is_dir: bool,
    ) {
        if i == 0 && j == 0 {
            str_to_draw.push_str(&self.normal);
            str_to_draw.push_str(&self.normal_bg);
        }

        if i == cursor && j == 0 {
            if is_dir {
                str_to_draw.push_str(&self.highlight_dir);
            } else {
                str_to_draw.push_str(&self.highlight);
            }
            str_to_draw.push_str(&self.highlight_bg);
        } else if i == cursor && j == r_w_l {
            str_to_draw.push_str(&self.normal);
            str_to_draw.push_str(&self.normal_bg);
        }
    }

    fn draw_bottom_line(
        &self,
        str_to_draw: &mut String,
        mode: Mode,
        current_path: &str,
        search_txt: &Vec<char>,
    ) {
        /* Goto bottom line */
        str_to_draw.push_str(&csi(&format!("{}H", self.height)));
        str_to_draw.push_str(&csi("0K"));

        if matches!(mode, Mode::SEARCH) {
            let search_txt_str = search_txt.into_iter().collect::<String>();

            str_to_draw.push_str("/");
            str_to_draw.push_str(
                &search_txt
                    .iter()
                    .take(self.bottom_line_slice(&search_txt_str))
                    .collect::<String>(),
            );
        } else {
            let current_path_sliced = current_path
                .chars()
                .take(self.bottom_line_slice(current_path))
                .collect::<String>();

            str_to_draw.push_str(&current_path_sliced);
        }
    }

    pub fn draw(
        &mut self,
        cursor: usize,
        current_dir: &Vec<String>,
        preview_dir: &Vec<String>,
        window_start: usize,
        current_path: &PathBuf,
        mode: Mode,
        search_txt: &Vec<char>,
    ) {
        let (h, w) = util::term_size();
        if self.height != h || self.width != w {
            self.height = h;
            self.width = w;
            self.pixels = vec![vec![' '; self.width]; self.height];
        }

        let mut str_to_draw = String::from("");
        str_to_draw.push_str(&csi("1H"));

        /* write pixel */
        let write_top: usize = self.height - 1;
        let write_bottom: usize = 0;

        let l_w_l: usize = 0;
        let l_w_r: usize = (self.width / 10 * 6 - 1) as usize;

        let r_w_l: usize = l_w_r + 1;
        let r_w_r: usize = self.width - 1;
        let preview_width: usize = self.width - r_w_l;

        let mut dir_i: usize = window_start;
        let mut ch_i: usize;

        self.clear_pixels();

        /* no content */
        if current_dir.len() == 0 {
            str_to_draw.push_str(&self.normal);
            str_to_draw.push_str(&self.normal_bg);

            for line in &self.pixels {
                let tmp_s = line.iter().collect::<String>(); // Vec<char> -> String
                str_to_draw.push_str(&tmp_s); // concat
            }

            str_to_draw.push_str(&csi("1H"));
            print!("{}", str_to_draw);
            let _ = io::stdout().flush();
            return;
        }

        /* left side */
        for i in write_bottom..=write_top {
            let c_a = current_dir[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in l_w_l..l_w_r {
                if ch_i >= c_a.len() {
                    break;
                }
                self.set(write_top - i, j, c_a[ch_i]);
                ch_i += 1;
            }
            dir_i += 1;
            if dir_i >= current_dir.len() {
                break;
            }
        }

        /* right side(preview) */
        dir_i = 0;

        for i in write_bottom..=write_top {
            if dir_i >= preview_dir.len() {
                break;
            }
            let c_a = preview_dir[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in r_w_l..r_w_r {
                if ch_i >= c_a.len() {
                    break;
                }
                self.set(write_top - i, j, c_a[ch_i]);
                ch_i += 1;
            }
            dir_i += 1;
        }

        let mut i: usize = 0;
        let mut j: usize;
        let mut font_len: usize = 0;
        let mut do_preview: bool = false;
        let mut is_dir: bool = false;

        loop {
            if i >= self.height {
                break;
            }
            j = 0;
            loop {
                if j >= self.width {
                    break;
                }
                if self.is_wide(self.pixels[i][j] as usize) {
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

                let mut tmp_path = current_path.clone();
                tmp_path.push(&current_dir[cursor]);
                if i + window_start == cursor && j == 0 && tmp_path.is_dir() == true {
                    is_dir = true;
                }

                self.check_insert_highlight(
                    &mut str_to_draw,
                    i,
                    j,
                    cursor - window_start,
                    r_w_l,
                    is_dir,
                );
                str_to_draw.push(self.pixels[i][j]);
                is_dir = false;

                j += 1;
            }
            font_len = 0;
            i += 1;
            do_preview = false;
        }

        /* draw bottom line after drawing the directories */
        self.draw_bottom_line(
            &mut str_to_draw,
            mode,
            &current_path.to_str().unwrap().to_string(),
            search_txt,
        );

        print!("{}", str_to_draw);
        let _ = io::stdout().flush();
    }

    fn set(&mut self, i: usize, j: usize, c: char) {
        let i_to_write: i32 = self.height as i32 - 1 - i as i32;
        let j_to_write: usize = j;
        if 0 <= i_to_write && i_to_write < self.height as i32 && j_to_write < self.width {
            self.pixels[i_to_write as usize][j_to_write] = c;
        }
    }
}

pub fn new() -> Canvas {
    let mut canvas = Canvas {
        height: 0,
        width: 0,
        pixels: Vec::new(),
        highlight: String::from(consts::HIGHLIGHT_TRANS),
        highlight_dir: String::from(consts::HIGHLIGHT_DIR_TRANS),
        highlight_bg: String::from(consts::HIGHLIGHT_BG_TRANS),
        normal: String::from(consts::NORMAL_TRANS),
        normal_bg: String::from(consts::NORMAL_BG_TRANS),
    };

    if matches!(util::get_theme(), Theme::DARK) {
        canvas.highlight = String::from(consts::HIGHLIGHT_DARK);
        canvas.highlight_dir = String::from(consts::HIGHLIGHT_DIR_DARK);
        canvas.highlight_bg = String::from(consts::HIGHLIGHT_BG_DARK);
        canvas.normal = String::from(consts::NORMAL_DARK);
        canvas.normal_bg = String::from(consts::NORMAL_BG_DARK);
    }

    canvas
}
