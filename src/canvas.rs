/*═══════════════════════════════════════════════════════════════════════╗
║                         (C)  Howard Chu                                ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

extern crate libc;

use crate::ops::Mode;
use crate::theme;
use crate::util;
use crate::widechar_width::{WcLookupTable, WcWidth};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pixels: Vec<Vec<char>>,
    theme: theme::Theme,
    utf8_table: WcLookupTable,
}

impl Clone for Canvas {
    fn clone(&self) -> Canvas {
        Canvas {
            height: self.height,
            width: self.width,
            pixels: Vec::new(),
            theme: theme::Theme::default(),
            utf8_table: WcLookupTable::new(),
        }
    }
}

fn csi(s: &str) -> String {
    let mut ret: String = String::from("\x1b[");
    ret.push_str(s);
    ret
}

impl Canvas {
    /// Set the internel pixel (char) representation
    fn set(&mut self, i: usize, j: usize, c: char) {
        if i < self.height && j < self.width {
            self.pixels[i][j] = c;
        }
    }

    /// Get the index where the bottom line text should be cropped
    fn bottom_line_configure(
        &self,
        current_path: &str,
        search_txt: &Vec<char>,
        mode: Mode,
    ) -> String {
        let mut bottom_line = String::new();

        if matches!(mode, Mode::SEARCH) {
            bottom_line.push_str("/");
            bottom_line.push_str(&search_txt.into_iter().collect::<String>());
        } else {
            bottom_line.push_str(current_path);
        }

        let mut display_len: usize = 0;
        let mut take_to: usize = 0;

        for c in bottom_line.chars() {
            let len = self.get_utf8_len(c);
            display_len += len;

            if display_len >= self.width {
                break;
            }

            take_to += 1;
        }

        bottom_line.chars().take(take_to).collect::<String>()
    }

    /// return whether this character is a full-width character that displays as two blocks in the
    /// terminal
    fn get_utf8_len(&self, c: char) -> usize {
        match self.utf8_table.classify(c) {
            WcWidth::One => 1,
            WcWidth::Two => 2,
            WcWidth::NonPrint => 0,
            WcWidth::Combining => 0,
            WcWidth::Ambiguous => 1,
            WcWidth::PrivateUse => 0,
            WcWidth::Unassigned => 0,
            WcWidth::WidenedIn9 => 2,
            WcWidth::NonCharacter => 0,
        }
    }

    /// Clear the internel character array
    fn clear_pixels(&mut self) {
        let c: char = ' ';
        for i in 0..self.height {
            for j in 0..self.width {
                self.pixels[i][j] = c;
            }
        }
    }

    /// Check if trans needs to highlight this text, if so, highlight
    fn check_insert_highlight(
        &self,
        str_to_draw: &mut String,
        i: usize,
        j: usize,
        cursor: usize,
        r_w_l: usize,
        is_dir: bool,
    ) {
        if j != 0 && j != r_w_l {
            return;
        }

        if i == 0 && j == 0 {
            str_to_draw.push_str(&self.theme.normal);
            str_to_draw.push_str(&self.theme.normal_background);
        }

        // This will be overwritten by highlight, if cursor is on it
        if !is_dir {
            str_to_draw.push_str(&self.theme.normal);
        }

        if i == cursor && j == 0 {
            str_to_draw.push_str(&self.theme.highlight);
            str_to_draw.push_str(&self.theme.highlight_background);
        } else {
            str_to_draw.push_str(&self.theme.normal_background);
        }

        // This is the opposite, cursor's highlight will be overwritten by directory color
        if is_dir {
            str_to_draw.push_str(&self.theme.highlight_dir);
        }
    }

    /// Draw file path or search text in the bottom line
    fn draw_bottom_line(
        &self,
        str_to_draw: &mut String,
        mode: Mode,
        current_path: &str,
        search_txt: &Vec<char>,
    ) {
        // Goto the bottom line
        str_to_draw.push_str(&csi(&format!("{}H", self.height)));
        str_to_draw.push_str(&csi("0K"));

        str_to_draw.push_str(&self.theme.normal);
        str_to_draw.push_str(&self.theme.normal_background);

        str_to_draw.push_str(&self.theme.bottom_bar);
        str_to_draw.push_str(&self.theme.bottom_bar_background);

        // fill the bottom line with color
        str_to_draw.push_str(&(0..self.width).map(|_| " ").collect::<String>());

        str_to_draw.push_str(&csi(&format!("{}H", self.height)));
        str_to_draw.push_str(&csi("0K"));
        str_to_draw.push_str(&self.bottom_line_configure(current_path, search_txt, mode));

        if matches!(mode, Mode::SEARCH) {
            // show the cursor when searching
            str_to_draw.push_str(&csi("?25h"));
        }
    }

    /// core function to display the window
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
        str_to_draw.push_str(&csi("?25l")); // hide cursor

        // l_w_l: left window's left
        let l_w_l: usize = 0;
        let l_w_r: usize = (self.width / 10 * 6 - 1) as usize;

        let r_w_l: usize = l_w_r + 1;
        let r_w_r: usize = self.width - 1;
        let preview_width: usize = self.width - r_w_l;

        let mut dir_i: usize = window_start;
        let mut ch_i: usize;

        self.clear_pixels();

        // No files in directory
        if current_dir.len() == 0 {
            str_to_draw.push_str(&self.theme.normal);
            str_to_draw.push_str(&self.theme.normal_background);

            // Empty lines still need to be drawn
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

        // left window
        for i in 0..=self.height - 1 {
            let c_a = current_dir[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in l_w_l..=l_w_r {
                if ch_i >= c_a.len() {
                    break;
                }
                self.set(i, j, c_a[ch_i]);
                ch_i += 1;
            }
            dir_i += 1;
            if dir_i >= current_dir.len() {
                break;
            }
        }

        // right preview window
        dir_i = 0;

        for i in 0..=self.height - 1 {
            if dir_i >= preview_dir.len() {
                break;
            }
            let c_a = preview_dir[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in r_w_l..=r_w_r {
                if ch_i >= c_a.len() {
                    break;
                }
                self.set(i, j, c_a[ch_i]);
                ch_i += 1;
            }
            dir_i += 1;
        }

        // after setting the pixels, format str_to_draw
        let mut actual_len: usize = 0;
        let mut do_preview: bool = false;
        let mut complement: usize = 0;

        for i in 0..self.height {
            let mut j = 0;
            // Iterate the column, j is jumpable so make it a loop instead of a for
            loop {
                if j >= self.width {
                    break;
                }

                let len = self.get_utf8_len(self.pixels[i][j]);

                // for a zero-width character such as a combining character, spaces in pixels is
                // not enough, insert more spaces (complement) for alignment
                if len == 0 {
                    actual_len += 1;
                    complement += 1;
                } else {
                    actual_len += len;
                }

                //  If the actual_len reaches over the capcity of the left window, discard this
                //  character and update the preview window.
                if actual_len > l_w_r + 1 && !do_preview {
                    // If the last character of this window is wide and that causes overflow,
                    // discard it, insert a white space so it aligns.
                    //
                    // actual_len == l_w_r + 2 means it has to be a wide character from the left
                    // window that just subtly causes the overflow, but not because it's time to
                    // preview the right window (for example, left window is exactly filled, and we
                    // got one more wide character on the left, no space left to insert it so it's
                    // time to switch to preview)
                    if j <= l_w_r
                        && actual_len == l_w_r + 2
                        && self.get_utf8_len(self.pixels[i][j]) > 1
                    {
                        str_to_draw.push(' ');
                    }

                    str_to_draw.push_str(&(0..complement).map(|_| ' ').collect::<String>());

                    j = r_w_l;
                    actual_len = 0;
                    complement = 0;
                    do_preview = true;

                    continue;
                }

                if do_preview && actual_len > preview_width {
                    // Same last wide character discard filling logic as above
                    if actual_len == preview_width + 1 && self.get_utf8_len(self.pixels[i][j]) > 1 {
                        str_to_draw.push(' ');
                    }
                    break;
                }

                // decide if the directory highlight should be added, this applies to both the left
                // window and the right preview window
                let is_dir = if !do_preview {
                    if i + window_start >= current_dir.len() {
                        false
                    } else {
                        let mut tmp_path = current_path.clone();
                        tmp_path.push(&current_dir[i + window_start]);
                        tmp_path.is_dir()
                    }
                } else {
                    let mut tmp_path = current_path.clone();
                    tmp_path.push(&current_dir[cursor]);
                    if i >= preview_dir.len() {
                        false
                    } else {
                        tmp_path.push(&preview_dir[i]);
                        tmp_path.is_dir()
                    }
                };

                // checks both windows
                self.check_insert_highlight(
                    &mut str_to_draw,
                    i,
                    j,
                    cursor - window_start,
                    r_w_l,
                    is_dir,
                );

                str_to_draw.push(self.pixels[i][j]);
                j += 1;
            } // loop

            str_to_draw.push_str(&(0..complement).map(|_| ' ').collect::<String>());

            actual_len = 0;
            complement = 0;
            do_preview = false;
        }

        // Draw bottom line after drawing the directories to prevent overlapping
        self.draw_bottom_line(
            &mut str_to_draw,
            mode,
            &current_path.to_str().unwrap().to_string(),
            search_txt,
        );

        print!("{}", str_to_draw);
        let _ = io::stdout().flush();
    }
}

pub fn new() -> Canvas {
    Canvas {
        height: 0,
        width: 0,
        pixels: Vec::new(),
        theme: theme::Theme::from(&util::get_theme()),
        utf8_table: WcLookupTable::new(),
    }
}
