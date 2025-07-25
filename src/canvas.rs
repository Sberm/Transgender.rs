/*═══════════════════════════════════════════════════════════════════════╗
║                         (C)  Howard Chu                                ║
║                                                                        ║
║ Permission to use, copy, modify, and/or distribute this software for   ║
║ any purpose with or without fee is hereby granted, provided that the   ║
║ above copyright notice and this permission notice appear in all copies ║
╚═══════════════════════════════════════════════════════════════════════*/

use crate::browser;
use crate::ops::Mode;
use crate::theme;
use crate::util;
use crate::widechar_width::{WcLookupTable, WcWidth};
#[cfg(not(test))]
use std::io::stdout;
use std::io::Write;

pub struct Canvas {
    pub height: usize,
    pub width: usize,
    pixels: Vec<Vec<char>>,
    theme: theme::Theme,
    utf8_table: WcLookupTable,
    pub bottom_start: usize, // the left border of the bottom bar text
    add_algnmt: bool,
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

    pub fn reset_bottom_bar(&mut self) {
        self.bottom_start = 0;
        self.add_algnmt = false;
    }

    /// Get the index where the bottom line text should be cropped
    fn bottom_line_configure(&mut self, browser: &browser::Browser) -> String {
        let mut bottom_line = String::new();
        let mut special_char = false;

        if matches!(browser.mode, Mode::Search) || matches!(browser.mode, Mode::RevSearch) {
            special_char = true; // we will prepend the slash later
            bottom_line.push_str(&browser.search_txt.iter().collect::<String>());
        } else {
            bottom_line.push_str(
                &browser
                    .current_path
                    .to_str()
                    .expect("couldn't convert current_path to str"),
            );
        }

        let mut width = self.width - (special_char as usize + self.add_algnmt as usize);
        let left_border = self.bottom_start;
        // TODO: possible integer underflow
        let mut right_border = self.bottom_start + width - 1;
        let mut len = 0;

        // find the right_border
        for i in self.bottom_start..browser.search_txt.len() + 1 {
            // the last empty character
            if i == browser.search_txt.len() {
                len += 1;
            } else {
                len += self.get_utf8_len(browser.search_txt[i]);
            }
            if len > width {
                break;
            }
            right_border = i;
        }

        // move the bottom text based on the position of the input cursor
        let mut right_overflow = false;
        if left_border > browser.input_cursor_pos {
            if self.add_algnmt == true {
                width += 1;
                self.add_algnmt = false;
            }
            self.bottom_start = browser.input_cursor_pos;
        } else if right_border < browser.input_cursor_pos {
            if self.add_algnmt == true {
                width += 1;
                self.add_algnmt = false;
            }

            let mut i = browser.input_cursor_pos;
            let mut legal_len = 0;
            len = 0;
            // decide the correct bottom_start, from right to left
            loop {
                if i == browser.search_txt.len() {
                    len += 1;
                } else {
                    len += self.get_utf8_len(browser.search_txt[i]);
                }
                if len > width {
                    break;
                }

                legal_len = len;
                self.bottom_start = i;
                if i == 0 {
                    break;
                }
                i -= 1;
            }
            // this means that a UTF8 full width character causes the cursor to shiver
            if legal_len != width && !self.add_algnmt {
                self.add_algnmt = true;
                width -= 1;
            }
            right_overflow = true;
        }

        let skipped = bottom_line.chars().skip(self.bottom_start);
        let mut to_take: usize = 0;
        if right_overflow {
            to_take = browser.input_cursor_pos - self.bottom_start + 1;
        } else {
            len = 0;
            for c in skipped.clone() {
                len += self.get_utf8_len(c);
                if len > width {
                    break;
                }
                to_take += 1;
            }
        }
        let mut result = skipped.take(to_take).collect::<String>();

        if self.add_algnmt {
            result.insert(0, '>');
        }
        if special_char {
            match browser.mode {
                Mode::Search => result.insert(0, '/'),
                Mode::RevSearch => result.insert(0, '?'),
                _ => {}
            }
        }
        result
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
        is_dir: bool,
    ) {
        if i == cursor && j == 0 {
            str_to_draw.push_str(&self.theme.highlight);
            str_to_draw.push_str(&self.theme.highlight_background);
        } else {
            str_to_draw.push_str(&self.theme.normal);
            str_to_draw.push_str(&self.theme.normal_background);
        }

        // This is the opposite, cursor's highlight will be overwritten by directory color
        if is_dir {
            str_to_draw.push_str(&self.theme.highlight_dir);
        }
    }

    /// Draw file path or search text in the bottom line
    fn draw_bottom_line(&mut self, str_to_draw: &mut String, browser: &browser::Browser) {
        // Goto the bottom line
        str_to_draw.push_str(&csi(&format!("{}H", self.height)));
        str_to_draw.push_str(&csi("0K"));

        str_to_draw.push_str(&self.theme.bottom_bar);
        str_to_draw.push_str(&self.theme.bottom_bar_background);

        // fill the bottom line with color
        str_to_draw.push_str(&(0..self.width).map(|_| " ").collect::<String>());

        str_to_draw.push_str(&csi(&format!("{}H", self.height)));
        str_to_draw.push_str(&csi("0K"));

        let content = self.bottom_line_configure(browser);
        str_to_draw.push_str(&content);

        if matches!(browser.mode, Mode::Search) || matches!(browser.mode, Mode::RevSearch) {
            // show the cursor when searching
            str_to_draw.push_str(&csi("?25h"));
            let mut real_len = 0;
            for i in self.bottom_start..browser.input_cursor_pos {
                real_len += self.get_utf8_len(browser.search_txt[i]);
            }
            // + 1 + 1: one because ansi escape is 1-index, another one because the extra slash
            str_to_draw.push_str(&csi(&format!(
                "{};{}H",
                self.height,
                real_len + 1 + 1 + if self.add_algnmt { 1 } else { 0 }
            )));
        }
    }

    /// core function to display the window
    pub fn draw(&mut self, browser: &browser::Browser, _test_out: Option<&mut String>) {
        #[cfg(not(test))]
        {
            // user may change the size of the terminal
            let (h, w) = util::term_size();
            if self.height != h || self.width != w {
                self.height = h;
                self.width = w;
                self.pixels = vec![vec![' '; self.width]; self.height];
            }
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

        let mut dir_i: usize = browser.window_start;
        let mut ch_i: usize;

        self.clear_pixels();

        // left window
        for i in 0..=self.height - 1 {
            if dir_i >= browser.content.len() {
                break;
            }
            let c_a = browser.content[dir_i].chars().collect::<Vec<char>>();
            ch_i = 0;
            for j in l_w_l..=l_w_r {
                if ch_i >= c_a.len() {
                    break;
                }
                self.set(i, j, c_a[ch_i]);
                ch_i += 1;
            }
            dir_i += 1;
            if dir_i >= browser.content.len() {
                break;
            }
        }

        // right preview window
        dir_i = 0;
        for i in 0..=self.height - 1 {
            if dir_i >= browser.preview.len() {
                break;
            }
            let c_a = browser.preview[dir_i].chars().collect::<Vec<char>>();
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
        let mut actual_len: usize;
        let mut do_preview: bool;
        let mut complement: usize;
        let mut j;
        for i in 0..self.height {
            j = 0;
            actual_len = 0;
            complement = 0;
            do_preview = false;

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

                // Add highlights
                if j == 0 || j == r_w_l {
                    // decide if the directory highlight should be added, this applies to both the left
                    // window and the right preview window
                    let is_dir = if !do_preview {
                        if i + browser.window_start >= browser.content.len() {
                            false
                        } else {
                            let mut tmp_path = browser.current_path.clone();
                            tmp_path.push(&browser.content[i + browser.window_start]);
                            tmp_path.is_dir()
                        }
                    } else {
                        if browser.cursor >= browser.content.len() {
                            false
                        } else {
                            let mut tmp_path = browser.current_path.clone();
                            tmp_path.push(&browser.content[browser.cursor]);
                            if i >= browser.preview.len() {
                                false
                            } else {
                                tmp_path.push(&browser.preview[i]);
                                tmp_path.is_dir()
                            }
                        }
                    };
                    // checks and inserts for both windows
                    self.check_insert_highlight(
                        &mut str_to_draw,
                        i,
                        j,
                        browser.cursor - browser.window_start,
                        is_dir,
                    );
                }
                str_to_draw.push(self.pixels[i][j]);
                j += 1;
            } // loop j

            str_to_draw.push_str(&(0..complement).map(|_| ' ').collect::<String>());
        }

        // Draw bottom line after drawing the directories to prevent overlapping
        self.draw_bottom_line(&mut str_to_draw, &browser);

        #[cfg(not(test))]
        {
            print!("{}", str_to_draw);
            let _ = stdout().flush();
        }
        #[cfg(test)]
        {
            *_test_out.expect("failed to unwrap test output") = str_to_draw.clone();
        }
    }
}

pub fn new(config_path: Option<&str>) -> Canvas {
    Canvas {
        height: 0,
        width: 0,
        pixels: Vec::new(),
        theme: theme::Theme::from(&util::get_theme(config_path)),
        utf8_table: WcLookupTable::new(),
        bottom_start: 0,
        add_algnmt: false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ops::Mode;
    use crate::util::test::{mktemp_conf, CleanupDir, CleanupFile, Rand};
    use std::fs::{create_dir, File};
    use std::path::PathBuf;

    #[test]
    fn test_csi() {
        assert_eq!(csi("foo"), "\x1b[foo");
    }

    #[test]
    fn test_new() {
        let (conf, _file) = mktemp_conf();
        if _file.is_none() {
            panic!("failed to create temp file");
        }
        let mut file = _file.unwrap();
        let _cf = CleanupFile { file: conf.clone() };
        let _ = file.write(b"theme = trans\n");
        let canvas = new(Some(&conf));
        assert_eq!(canvas.height, 0);
        assert_eq!(canvas.width, 0);
        assert_eq!(canvas.pixels.is_empty(), true);
        // trans' highlight value
        assert_eq!(canvas.theme.highlight, "\x1b[0;37m");
        assert_eq!(canvas.utf8_table.table.len(), 65536);
        assert_eq!(canvas.bottom_start, 0);
        assert_eq!(canvas.add_algnmt, false);
    }

    #[test]
    fn test_set() {
        let mut canvas = new(None);
        let mut rand = Rand::new();
        let n = rand.rand_uint(4, 50);
        canvas.width = n;
        canvas.height = n;
        canvas.pixels = vec![vec!['X'; n]; n];
        canvas.set(n / 2, n / 2, 'Y');
        assert_eq!(canvas.pixels[n / 2][n / 2], 'Y');
    }

    #[test]
    fn test_reset_bottom_bar() {
        let mut canvas = new(None);
        canvas.reset_bottom_bar();
        assert_eq!(canvas.bottom_start, 0);
        assert_eq!(canvas.add_algnmt, false);
    }

    #[test]
    fn test_bottom_line_configure() {
        let texts = [
            "Ċ昃.鱁ᔡԝv6tղЈ液ϋxꖷA㣌₡i䔸긫qަ쬸쒽mUǦ裊[⿇::žҟ掕",
            "汉皇重色思倾国，御宇多年求不得。杨家有女初长成，养在深闺人未识。
             天生丽质难自弃，一朝选在君王侧。回眸一笑百媚生，六宫粉黛无颜色。
             春寒赐浴华清池，温泉水滑洗凝脂。侍儿扶起娇无力，始是新承恩泽时。
             云鬓花颜金步摇，芙蓉帐暖度春宵。春宵苦短日高起，从此君王不早朝。
             承欢侍宴无闲暇，春从春游夜专夜。后宫佳丽三千人，三千宠爱在一身。
             金屋妆成娇侍夜，玉楼宴罢醉和春。姊妹弟兄皆列土，可怜光彩生门户。
             遂令天下父母心，不重生男重生女。骊宫高处入青云，仙乐风飘处处闻。
             缓歌慢舞凝丝竹，尽日君王看不足。渔阳鼙鼓动地来，惊破霓裳羽衣曲。",
            "And all the graven images thereof shall be$ beaten to pieces, and all the hires thereof
             shall be burned with the fire, and all the idols thereof will I lay desolate: for she
             gathered it of the hire of an harlot, and they shall return to the hire of an harlot."
        ];
        let texts_configured = [
            "/ᔡԝv6tղЈ液ϋxꖷA㣌₡i䔸긫qަ쬸쒽mUǦ裊[⿇::žҟ",
            "/思倾国，御宇多年求不得。杨家有女初长成",
            "/all the graven images thereof shall be$",
        ];
        let texts_configured_rev = [
            "?ᔡԝv6tղЈ液ϋxꖷA㣌₡i䔸긫qަ쬸쒽mUǦ裊[⿇::žҟ",
            "?思倾国，御宇多年求不得。杨家有女初长成",
            "?all the graven images thereof shall be$",
        ];
        let mut canvas = new(None);
        let width = 40;
        canvas.width = width;
        canvas.bottom_start = 4;
        let mut browser = browser::new(".", None, None);
        browser.mode = Mode::Search;
        browser.input_cursor_pos = canvas.bottom_start;
        let mut i = 0;
        for st in texts.iter() {
            browser.search_txt = st.chars().collect::<Vec<char>>();
            let bottom_line_str = canvas.bottom_line_configure(&browser);
            assert_eq!(bottom_line_str, texts_configured[i]);
            i += 1;
        }
        // reverse search
        browser.mode = Mode::RevSearch;
        browser.input_cursor_pos = canvas.bottom_start;
        i = 0;
        for st in texts.iter() {
            browser.search_txt = st.chars().collect::<Vec<char>>();
            let bottom_line_str = canvas.bottom_line_configure(&browser);
            assert_eq!(bottom_line_str, texts_configured_rev[i]);
            i += 1;
        }
    }

    #[test]
    fn test_line_get_utf8_len() {
        let canvas = new(None);
        assert_eq!(canvas.get_utf8_len('𰻝'), 2);
        assert_eq!(canvas.get_utf8_len('ぎ'), 2);
        assert_eq!(canvas.get_utf8_len(')'), 1);
    }

    #[test]
    fn test_clear_pixels() {
        let mut canvas = new(None);
        let n = 10;
        let v = vec![vec![' '; n]; n];
        canvas.pixels = v.clone();
        canvas.pixels[0][0] = '&';
        canvas.pixels[n - 1][n - 1] = '^';
        canvas.height = n;
        canvas.width = n;
        canvas.clear_pixels();
        assert_eq!(canvas.pixels, v);
    }

    #[test]
    fn test_check_insert_highlight() {
        let canvas = new(None);

        let mut string_to_draw = String::new();
        // cursor is on it, regular file
        canvas.check_insert_highlight(&mut string_to_draw, 0, 0, 0, false);
        assert_eq!(
            string_to_draw,
            format!(
                "{}{}",
                canvas.theme.highlight, canvas.theme.highlight_background
            )
        );

        // cursor is not on it, regular file
        string_to_draw = String::new();
        canvas.check_insert_highlight(&mut string_to_draw, 0, 0, 1, false);
        assert_eq!(
            string_to_draw,
            format!("{}{}", canvas.theme.normal, canvas.theme.normal_background)
        );

        // cursor is on it, directory
        string_to_draw = String::new();
        canvas.check_insert_highlight(&mut string_to_draw, 0, 0, 0, true);
        assert_eq!(
            string_to_draw,
            format!(
                "{}{}{}",
                canvas.theme.highlight,
                canvas.theme.highlight_background,
                canvas.theme.highlight_dir
            )
        );

        // cursor is not on it, directory
        string_to_draw = String::new();
        canvas.check_insert_highlight(&mut string_to_draw, 0, 0, 1, true);
        assert_eq!(
            string_to_draw,
            format!(
                "{}{}{}",
                canvas.theme.normal, canvas.theme.normal_background, canvas.theme.highlight_dir
            )
        );
    }

    #[test]
    fn test_draw_bottom_line() {
        let mut canvas = new(None);
        // in normal mode, print current path
        let mut str_to_draw = String::new();
        let current_path = "dummy_path";
        let current_path_buf = PathBuf::from(current_path);
        let mut browser = browser::new(".", None, None);
        browser.mode = Mode::Normal;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = Vec::new();
        browser.input_cursor_pos = 0;
        canvas.width = current_path.chars().count();
        canvas.draw_bottom_line(&mut str_to_draw, &browser);
        assert_eq!(
            str_to_draw,
            format!(
                "{}{}{}{}{}{}{}{}",
                &csi("0H"),
                &csi("0K"),
                &canvas.theme.bottom_bar,
                &canvas.theme.bottom_bar_background,
                &(0..canvas.width).map(|_| " ").collect::<String>(),
                &csi("0H"),
                &csi("0K"),
                current_path
            )
        );
        // cropped
        let to_crop = 2;
        str_to_draw = String::new();
        canvas.width -= to_crop;
        browser.mode = Mode::Normal;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = Vec::new();
        browser.input_cursor_pos = 0;
        canvas.draw_bottom_line(&mut str_to_draw, &browser);
        assert_eq!(
            str_to_draw,
            format!(
                "{}{}{}{}{}{}{}{}",
                &csi("0H"),
                &csi("0K"),
                &canvas.theme.bottom_bar,
                &canvas.theme.bottom_bar_background,
                &(0..canvas.width).map(|_| " ").collect::<String>(),
                &csi("0H"),
                &csi("0K"),
                current_path
                    .chars()
                    .take(current_path.len() - to_crop)
                    .collect::<String>()
            )
        );
        // search no crop
        str_to_draw = String::new();
        let text = "foobar";
        let search_txt = text.chars().collect::<Vec<char>>();
        canvas.width = search_txt.len() + 3;
        let cursor_pos = search_txt.len() / 2;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom_line(&mut str_to_draw, &browser);
        assert_eq!(
            str_to_draw,
            format!(
                "{}{}{}{}{}{}{}/{}{}{}",
                &csi("0H"),
                &csi("0K"),
                canvas.theme.bottom_bar,
                canvas.theme.bottom_bar_background,
                (0..canvas.width).map(|_| " ").collect::<String>(),
                &csi("0H"),
                &csi("0K"),
                text,
                &csi("?25h"),
                &csi(&format!(
                    "0;{}H",
                    search_txt.len() / 2 + 2 + if canvas.add_algnmt { 1 } else { 0 }
                ))
            )
        );
        // search cropped
        canvas.reset_bottom_bar();
        str_to_draw = String::new();
        let text = "foobarfoobar";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = 0;
        canvas.width = search_txt.len() / 2;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom_line(&mut str_to_draw, &browser);
        assert_eq!(
            str_to_draw,
            format!(
                "{}{}{}{}{}{}{}/{}{}{}",
                &csi("0H"),
                &csi("0K"),
                canvas.theme.bottom_bar,
                canvas.theme.bottom_bar_background,
                (0..canvas.width).map(|_| " ").collect::<String>(),
                &csi("0H"),
                &csi("0K"),
                text.chars()
                    .take(canvas.width - (1 + if canvas.add_algnmt { 1 } else { 0 }))
                    .collect::<String>(),
                &csi("?25h"),
                &csi(&format!(
                    "0;{}H",
                    cursor_pos + 2 + if canvas.add_algnmt { 1 } else { 0 }
                ))
            )
        );
        // search cropped and non-zero cursor placement
        canvas.reset_bottom_bar();
        str_to_draw = String::new();
        let text = "foobarfoobar";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = search_txt.len() - 2; // on the 'a'
        canvas.width = search_txt.len() / 2;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom_line(&mut str_to_draw, &browser);
        assert_eq!(
            str_to_draw,
            format!(
                "{}{}{}{}{}{}{}/{}{}{}",
                &csi("0H"),
                &csi("0K"),
                canvas.theme.bottom_bar,
                canvas.theme.bottom_bar_background,
                (0..canvas.width).map(|_| " ").collect::<String>(),
                &csi("0H"),
                &csi("0K"),
                "fooba",
                &csi("?25h"),
                &csi(&format!("0;{}H", canvas.width))
            )
        );
        // search cropped and non-zero cursor placement and UTF8 character
        canvas.reset_bottom_bar();
        str_to_draw = String::new();
        let text = "从此君王不早朝aaab";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = search_txt.len();
        canvas.width = 18;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom_line(&mut str_to_draw, &browser);
        assert_eq!(
            str_to_draw,
            format!(
                "{}{}{}{}{}{}{}/{}{}{}",
                &csi("0H"),
                &csi("0K"),
                canvas.theme.bottom_bar,
                canvas.theme.bottom_bar_background,
                (0..canvas.width).map(|_| " ").collect::<String>(),
                &csi("0H"),
                &csi("0K"),
                "此君王不早朝aaab",
                &csi("?25h"),
                &csi(&format!("0;{}H", canvas.width))
            )
        );
        // search cropped and non-zero cursor placement and UTF8 character and alignment '>'
        canvas.reset_bottom_bar();
        str_to_draw = String::new();
        let text = "从此君王不早朝aaab";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = search_txt.len();
        canvas.width = 19;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom_line(&mut str_to_draw, &browser);
        assert_eq!(
            str_to_draw,
            format!(
                "{}{}{}{}{}{}{}/{}{}{}",
                &csi("0H"),
                &csi("0K"),
                canvas.theme.bottom_bar,
                canvas.theme.bottom_bar_background,
                (0..canvas.width).map(|_| " ").collect::<String>(),
                &csi("0H"),
                &csi("0K"),
                ">此君王不早朝aaab",
                &csi("?25h"),
                &csi(&format!("0;{}H", canvas.width))
            )
        );
        // reverse search/slash
        canvas.reset_bottom_bar();
        str_to_draw = String::new();
        let text = "从此君王不早朝aaab";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = search_txt.len();
        canvas.width = 18;
        browser.mode = Mode::RevSearch;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom_line(&mut str_to_draw, &browser);
        assert_eq!(
            str_to_draw,
            format!(
                "{}{}{}{}{}{}{}?{}{}{}",
                &csi("0H"),
                &csi("0K"),
                canvas.theme.bottom_bar,
                canvas.theme.bottom_bar_background,
                (0..canvas.width).map(|_| " ").collect::<String>(),
                &csi("0H"),
                &csi("0K"),
                "此君王不早朝aaab",
                &csi("?25h"),
                &csi(&format!("0;{}H", canvas.width))
            )
        );
    }

    fn new_canvas(width: usize, height: usize, conf: Option<&str>) -> Canvas {
        let mut canvas = new(conf);
        canvas.width = width;
        canvas.height = height;
        canvas.pixels = vec![vec![' '; width]; height];
        canvas
    }

    #[test]
    fn test_draw() {
        let (conf, _file) = mktemp_conf();
        if _file.is_none() {
            panic!("failed to create temp file");
        }
        let mut file = _file.unwrap();
        let _cf = CleanupFile { file: conf.clone() };
        let _ = file.write(b"theme = lucius\n");
        let width = 30;
        let height = 14;
        let to_vec = |slice1: &[&str], slice2: &[&str]| {
            let mut v = Vec::new();
            for s in slice1 {
                v.push(s.to_string());
            }
            for s in slice2 {
                v.push(s.to_string());
            }
            v
        };
        let mut cleanups: Vec<CleanupDir> = Vec::new();

        // normal
        //
        let mut canvas = new_canvas(width, height, Some(&conf));
        // create a directory named /tmp/ts-test-draw
        let parent = "/tmp/ts-test-draw";
        cleanups.push(CleanupDir {
            dir: parent.to_owned(),
        });
        let _ = create_dir(parent);
        // put 4 directories in it, and 3 files
        let comp_dir = "zComplicatedDirectoryName";
        let d_depth1 = ["d1", "d2", "d3", "d4", comp_dir];
        let f_depth1 = ["f1", "f2", "f3"];
        for d in d_depth1 {
            let tmp = format!("{}/{}", parent, d);
            let _ = create_dir(&tmp);
        }
        for f in f_depth1 {
            let tmp = format!("{}/{}", parent, f);
            let _ = File::create(&tmp);
        }
        let child = d_depth1[0];
        // create 4 directories as the sub-directories of the first directory, and create 3 files
        // in that parent directory as well
        let d_depth2 = ["dd1", "dd2", "dd3", "dd4"];
        let f_depth2 = ["ff1", "ff2", "ff3"];
        for d in d_depth2 {
            let tmp = format!("{}/{}/{}", parent, child, d);
            let _ = create_dir(&tmp);
        }
        for f in f_depth2 {
            let tmp = format!("{}/{}/{}", parent, child, f);
            let _ = File::create(&tmp);
        }
        // put the cursor on the first directory, and render the result I didn't sort the
        // directories, so d1 is the first entry, no need to change the value of the cursor
        let mut test_out = String::new();
        let mut content = to_vec(&d_depth1, &f_depth1);
        let preview = to_vec(&d_depth2, &f_depth2);
        assert_eq!(canvas.width, width);
        assert_eq!(canvas.height, height);
        let mut browser = browser::new(&parent, None, None);
        let current_path = PathBuf::from(parent);
        browser.cursor = 0;
        browser.content = content.clone();
        browser.preview = preview;
        browser.window_start = 0;
        browser.current_path = current_path.clone();
        browser.mode = Mode::Normal;
        browser.search_txt = Vec::new();
        browser.input_cursor_pos = 0;
        canvas.draw(&browser, Some(&mut test_out));
        assert_eq!(test_out, "\u{1b}[1H\u{1b}[?25l\u{1b}[38;5;187m\u{1b}[48;5;238m\u{1b}[38;5;117md1                \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117mdd1         \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md2                \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117mdd2         \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md3                \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117mdd3         \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md4                \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117mdd4         \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117mzComplicatedDirect\u{1b}[38;5;188m\u{1b}[48;5;236mff1         \u{1b}[38;5;188m\u{1b}[48;5;236mf1                \u{1b}[38;5;188m\u{1b}[48;5;236mff2         \u{1b}[38;5;188m\u{1b}[48;5;236mf2                \u{1b}[38;5;188m\u{1b}[48;5;236mff3         \u{1b}[38;5;188m\u{1b}[48;5;236mf3                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[14H\u{1b}[0K\u{1b}[38;5;188m\u{1b}[48;5;238m                              \u{1b}[14H\u{1b}[0K/tmp/ts-test-draw");

        // search
        //
        // no searching is performed, but move the cursor to the target position by hand just for
        // the sake of it
        let mut pos = 0;
        for d in d_depth1 {
            if d == comp_dir {
                break;
            }
            pos += 1;
        }
        browser.cursor = pos;
        browser.content = content.clone();
        browser.preview = Vec::new();
        browser.window_start = 0;
        browser.current_path = current_path.clone();
        browser.mode = Mode::Search;
        browser.search_txt = comp_dir.chars().collect::<Vec<char>>();
        browser.input_cursor_pos = 0;
        // preview is empty
        canvas.draw(&browser, Some(&mut test_out));
        assert_eq!(test_out, "\u{1b}[1H\u{1b}[?25l\u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md1                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md2                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md3                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md4                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;187m\u{1b}[48;5;238m\u{1b}[38;5;117mzComplicatedDirect\u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236mf1                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236mf2                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236mf3                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[14H\u{1b}[0K\u{1b}[38;5;188m\u{1b}[48;5;238m                              \u{1b}[14H\u{1b}[0K/zComplicatedDirectoryName\u{1b}[?25h\u{1b}[14;2H");

        // UTF8
        //
        // maximum 18 characters in the left window
        let utf8_filename = ":::冬川や家鴨四五羽に足らぬ水:::";
        content.push(utf8_filename.to_owned());
        browser.cursor = pos;
        browser.content = content.clone();
        browser.preview = Vec::new();
        browser.window_start = 0;
        browser.current_path = current_path.clone();
        browser.mode = Mode::Normal;
        browser.search_txt = Vec::new();
        browser.input_cursor_pos = 0;
        canvas.draw(&browser, Some(&mut test_out));
        assert_eq!(test_out, "\u{1b}[1H\u{1b}[?25l\u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md1                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md2                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md3                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m\u{1b}[38;5;117md4                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;187m\u{1b}[48;5;238m\u{1b}[38;5;117mzComplicatedDirect\u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236mf1                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236mf2                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236mf3                \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m:::冬川や家鴨四五 \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m            \u{1b}[14H\u{1b}[0K\u{1b}[38;5;188m\u{1b}[48;5;238m                              \u{1b}[14H\u{1b}[0K/tmp/ts-test-draw");
    }

    #[test]
    fn test_draw_empty_dir() {
        let width = 31;
        let height = 14;

        let (conf, _file) = mktemp_conf();
        let mut file = _file.unwrap();
        let _cf = CleanupFile { file: conf.clone() };
        let _ = file.write(b"theme = lucius\n");

        let mut canvas = new_canvas(width, height, Some(&conf));

        let parent = "/tmp/ts-test-draw-empty";
        let _ = create_dir(parent);
        let _cd = CleanupDir {
            dir: parent.to_owned(),
        };

        // don't pass parent directly to browser to avoid /private/tmp on MacOS
        let mut browser = browser::new(".", None, Some(&conf));
        browser.content = Vec::new();
        browser.current_path = PathBuf::from(parent);
        let mut test_out = String::new();
        // everything is empty (in an empty directory)
        canvas.draw(&browser, Some(&mut test_out));
        assert_eq!(test_out, "\u{1b}[1H\u{1b}[?25l\u{1b}[38;5;187m\u{1b}[48;5;238m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[14H\u{1b}[0K\u{1b}[38;5;188m\u{1b}[48;5;238m                               \u{1b}[14H\u{1b}[0K/tmp/ts-test-draw-empty");
    }
}
