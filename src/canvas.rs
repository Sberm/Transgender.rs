/*═══════════════════════════════════════════════════════════════════════╗
║                          ©  Howard Chu                                 ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

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
    /*
     * Get the index where the bottom line of text should be cropped
     *
     * returns
     *  The index at which the bottom line's text content should be cropped
     */
    fn bottom_line_slice(&self, s: &str) -> usize {
        /* Make sure the bottom line doesn't overflow */
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

    /*
     * Is it a full-width character that displays as two blocks in the terminal
     *
     * returns
     *  whether this unicode character is full-width
     */
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

    /*
     * Clear the internel character array
     */
    fn clear_pixels(&mut self) {
        let c: char = ' ';
        for i in 0..self.height {
            for j in 0..self.width {
                self.pixels[i][j] = c;
            }
        }
    }

    /*
     * Check if trans needs to highlight this text, if so, highlight.
     */
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

    /*
     * Draw file path or search text in the bottom line
     */
    fn draw_bottom_line(
        &self,
        str_to_draw: &mut String,
        mode: Mode,
        current_path: &str,
        search_txt: &Vec<char>,
    ) {
        /* Goto the bottom line */
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

    /*
     * Core function to display the window
     */
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

        /* Write pixel */
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

        /* No files in directory */
        if current_dir.len() == 0 {
            str_to_draw.push_str(&self.normal);
            str_to_draw.push_str(&self.normal_bg);

            /* Empty lines still need to be drawn */
            for line in &self.pixels {
                let tmp_s = line.iter().collect::<String>();
                str_to_draw.push_str(&tmp_s);
            }

            self.draw_bottom_line(
                &mut str_to_draw,
                mode,
                &current_path.to_str().unwrap().to_string(),
                search_txt,
            );

            print!("{}", str_to_draw);
            let _ = io::stdout().flush();
            return;
        }

        /* Left side */
        for i in write_bottom..=write_top {
            let c_a = current_dir[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in l_w_l..=l_w_r {
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

        /* Right side preview window */
        dir_i = 0;

        for i in write_bottom..=write_top {
            if dir_i >= preview_dir.len() {
                break;
            }
            let c_a = preview_dir[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in r_w_l..=r_w_r {
                if ch_i >= c_a.len() {
                    break;
                }
                self.set(write_top - i, j, c_a[ch_i]);
                ch_i += 1;
            }
            dir_i += 1;
        }

        let mut font_len: usize = 0;
        let mut do_preview: bool = false;
        let mut is_dir: bool = false;

        for i in 0..self.height {
            let mut j = 0;
            /* Iterate the column, j is jumpable so make it a loop instead of a for */
            loop {
                if j >= self.width {
                    break;
                }

                if self.is_wide(self.pixels[i][j] as usize) {
                    font_len += 2;
                } else {
                    font_len += 1;
                }

                /*
                 * If the font_len reaches over the capcity of the left side window, discard this
                 * character and update the preview window.
                 */
                if font_len > l_w_r + 1 && !do_preview {
                    /*
                     * If the last character of this window is wide and that causes overflow,
                     * discard it, insert a white space so it aligns.
                     *
                     * font_len == l_w_r + 2 means it has to be a wide character from the left
                     * window that just subtly causes the overflow, but not because it's time to
                     * preview the right window (for example, left window is exactly filled, and we
                     * got one more wide character on the left, no space left to insert it so it's
                     * time to switch to preview)
                     */
                    if j <= l_w_r
                        && font_len == l_w_r + 2
                        && self.is_wide(self.pixels[i][j] as usize)
                    {
                        str_to_draw.push(' ');
                    }

                    j = r_w_l;
                    font_len = 0;
                    do_preview = true;
                    continue;
                }

                if do_preview && font_len > preview_width {
                    // Same last wide character discard filling logic as above
                    if font_len == preview_width + 1 && self.is_wide(self.pixels[i][j] as usize) {
                        str_to_draw.push(' ');
                    }
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
            do_preview = false;
        }

        /* Draw bottom line after drawing the directories */
        self.draw_bottom_line(
            &mut str_to_draw,
            mode,
            &current_path.to_str().unwrap().to_string(),
            search_txt,
        );

        print!("{}", str_to_draw);
        let _ = io::stdout().flush();
    }

    /*
     * Set the internel pixel (char) representation
     */
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
