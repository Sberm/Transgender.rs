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
    theme: theme::Theme,
    utf8_table: WcLookupTable,
    pub bottom_left: usize, // index in search_txt
    pub bottom_right: usize,
    pad: bool,
}

fn csi(s: &str) -> String {
    let mut ret: String = String::from("\x1b[");
    ret.push_str(s);
    ret
}

fn is_dir(do_preview: bool, i: usize, browser: &browser::Browser) -> bool {
    if !do_preview {
        if i + browser.window_start >= browser.content.len() {
            return false;
        } else {
            let mut tmp_path = browser.current_path.clone();
            tmp_path.push(&browser.content[i + browser.window_start]);
            return tmp_path.is_dir();
        }
    } else {
        if browser.cursor >= browser.content.len() {
            return false;
        } else {
            let mut tmp_path = browser.current_path.clone();
            tmp_path.push(&browser.content[browser.cursor]);
            if i >= browser.preview.len() {
                return false;
            } else {
                tmp_path.push(&browser.preview[i]);
                return tmp_path.is_dir();
            }
        }
    }
}

enum FitDirection {
    Forward,
    Backward,
}

impl Canvas {
    /// Set the internel pixel (char) representation
    fn set_pixel(&self, pixels: &mut Vec<Vec<char>>, i: usize, j: usize, c: char) {
        if i < self.height && j < self.width {
            pixels[i][j] = c;
        }
    }

    pub fn reset_bottom(&mut self) {
        self.bottom_left = 0;
        self.bottom_right = 0;
        self.pad = false;
    }

    fn fit(&mut self, bottom: &Vec<char>, width: usize, order: FitDirection) {
        let mut len = 0;
        let mut len_good = 0;
        match order {
            FitDirection::Forward => {
                for i in self.bottom_left..bottom.len() + 1 {
                    // the last empty character
                    if i == bottom.len() {
                        len += 1;
                    } else {
                        len += self.utf8_len(bottom[i]);
                    }
                    if len > width {
                        break;
                    }
                    self.bottom_right = i;
                }
            }
            FitDirection::Backward => {
                for i in (0..=self.bottom_right).rev() {
                    if i == bottom.len() {
                        len += 1;
                    } else {
                        len += self.utf8_len(bottom[i]);
                    }
                    if len > width {
                        break;
                    }
                    len_good = len;
                    self.bottom_left = i;
                }
                if !self.pad && len_good + 1 == width {
                    self.pad = true;
                }
            }
        }
    }

    fn get_bottom(&mut self, browser: &browser::Browser) -> String {
        let mut special_char = false;
        // unconditional, I know
        let cur_path_v = browser
            .current_path
            .to_str()
            .expect("couldn't convert current_path to str")
            .chars()
            .collect::<Vec<char>>();
        let bottom_v =
            if matches!(browser.mode, Mode::Search) || matches!(browser.mode, Mode::RevSearch) {
                special_char = true; // we will prepend the slash later
                &browser.search_txt
            } else {
                &cur_path_v
            };

        // 1 is the special character for search mode, / or ?
        if self.width < 1 {
            println!("width too small");
            util::slp(2);
            browser.exit_cur_dir();
        }
        let width_no_spec_char = self.width - special_char as usize;
        // right = 0 means reset is needed
        if self.bottom_right == 0 {
            self.fit(bottom_v, width_no_spec_char, FitDirection::Forward);
        }

        // when cursor goes out of bound
        if browser.input_cursor_pos < self.bottom_left {
            // pad is only refreshed when borders are crossed
            self.pad = false;
            self.bottom_left = browser.input_cursor_pos;
            self.fit(bottom_v, width_no_spec_char, FitDirection::Forward);
        } else if browser.input_cursor_pos > self.bottom_right {
            self.pad = false;
            self.bottom_right = browser.input_cursor_pos;
            self.fit(bottom_v, width_no_spec_char, FitDirection::Backward);
        } else {
            // left <= cursor <= right
            self.fit(
                bottom_v,
                width_no_spec_char - self.pad as usize,
                FitDirection::Forward,
            );
        }

        // construct bottom
        let mut bottom = String::new();
        if special_char {
            match browser.mode {
                Mode::Search => bottom.push('/'),
                Mode::RevSearch => bottom.push('?'),
                _ => {}
            }
        }
        if self.pad {
            bottom.push('>');
        }
        assert!(self.bottom_right >= self.bottom_left);
        bottom.push_str(
            &bottom_v
                .into_iter()
                .skip(self.bottom_left)
                .take(self.bottom_right - self.bottom_left + 1)
                .collect::<String>(),
        );
        bottom
    }

    /// return whether this character is a full-width character that displays as two blocks in the
    /// terminal
    fn utf8_len(&self, c: char) -> usize {
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
    fn draw_bottom(&mut self, str_to_draw: &mut String, browser: &browser::Browser) {
        // Goto the bottom line
        str_to_draw.push_str(&csi(&format!("{}H", self.height)));
        str_to_draw.push_str(&csi("0K"));

        str_to_draw.push_str(&self.theme.bottom_bar);
        str_to_draw.push_str(&self.theme.bottom_bar_background);

        // fill the bottom line with color
        str_to_draw.push_str(&(0..self.width).map(|_| " ").collect::<String>());

        str_to_draw.push_str(&csi(&format!("{}H", self.height)));
        str_to_draw.push_str(&csi("0K"));

        let content = self.get_bottom(browser);
        str_to_draw.push_str(&content);

        if matches!(browser.mode, Mode::Search) || matches!(browser.mode, Mode::RevSearch) {
            // show the cursor when searching
            str_to_draw.push_str(&csi("?25h"));
            let mut real_len = 0;
            for i in self.bottom_left..browser.input_cursor_pos {
                real_len += self.utf8_len(browser.search_txt[i]);
            }
            // + 1 + 1: one because ansi escape is 1-index, another one because the extra slash
            str_to_draw.push_str(&csi(&format!(
                "{};{}H",
                self.height,
                real_len + 1 + 1 + if self.pad { 1 } else { 0 }
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
            }
        }

        let mut pixels = vec![vec![' '; self.width]; self.height];
        let mut str_to_draw = String::from("");

        str_to_draw.push_str(&csi("1H"));
        str_to_draw.push_str(&csi("?25l")); // hide cursor

        // l_w_l: left window's left
        let l_w_l: usize = 0;
        if self.width / 10 * 6 < 1 {
            println!("width is too small to divide");
            util::slp(2);
            browser.exit_cur_dir();
        }
        let l_w_r: usize = (self.width / 10 * 6 - 1) as usize;

        let r_w_l: usize = l_w_r + 1;
        if self.width < 1 {
            println!("width too small, unable to proceed drawing");
            util::slp(1);
            browser.exit_cur_dir();
        }
        let r_w_r: usize = self.width - 1;
        let preview_width: usize = self.width - r_w_l;

        let mut dir_i: usize = browser.window_start;
        let mut ch_i: usize;

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
                self.set_pixel(&mut pixels, i, j, c_a[ch_i]);
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
                self.set_pixel(&mut pixels, i, j, c_a[ch_i]);
                ch_i += 1;
            }
            dir_i += 1;
        }

        // after setting the pixels, format str_to_draw
        let mut real_len: usize;
        let mut do_preview: bool;
        let mut complement: usize;
        let mut j;
        for i in 0..self.height {
            j = 0;
            real_len = 0;
            complement = 0;
            do_preview = false;

            // Iterate the column, j is jumpable so make it a loop instead of a for
            loop {
                if j >= self.width {
                    break;
                }

                let len = self.utf8_len(pixels[i][j]);

                // for a zero-width character such as a combining character, spaces in pixels is
                // not enough, insert more spaces (complement) for paddings
                if len == 0 {
                    real_len += 1;
                    complement += 1;
                } else {
                    real_len += len;
                }

                let left_win_len = l_w_r + 1;
                //  If the real_len reaches over the capcity of the left window, discard this
                //  character and update the preview window.
                if real_len > left_win_len && !do_preview {
                    // If the last character of this window is wide and it causes overflow,
                    // discard it, insert a white space so it paddings.
                    if j <= l_w_r && real_len == left_win_len + 1 && self.utf8_len(pixels[i][j]) > 1
                    {
                        str_to_draw.push(' ');
                    }

                    str_to_draw.push_str(&(0..complement).map(|_| ' ').collect::<String>());

                    j = r_w_l;
                    real_len = 0;
                    complement = 0;
                    do_preview = true;

                    continue;
                }

                if do_preview && real_len > preview_width {
                    // Same last wide character discard logic as above
                    if real_len == preview_width + 1 && self.utf8_len(pixels[i][j]) > 1 {
                        str_to_draw.push(' ');
                    }
                    break;
                }

                // Add highlights
                if j == 0 || j == r_w_l {
                    // decide if the directory highlight should be added, this applies to both the left
                    // window and the right preview window
                    let is_dir = is_dir(do_preview, i, browser);
                    // checks and inserts for both windows
                    self.check_insert_highlight(
                        &mut str_to_draw,
                        i,
                        j,
                        browser.cursor - browser.window_start,
                        is_dir,
                    );
                }
                str_to_draw.push(pixels[i][j]);
                j += 1;
            } // j
            str_to_draw.push_str(&(0..complement).map(|_| ' ').collect::<String>());
        } // i

        // Draw bottom line after drawing the directories to prevent overlapping
        self.draw_bottom(&mut str_to_draw, &browser);

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
        theme: theme::Theme::from(&util::get_theme(config_path)),
        utf8_table: WcLookupTable::new(),
        bottom_left: 0,
        bottom_right: 0,
        pad: false,
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
        // trans' highlight value
        assert_eq!(canvas.theme.highlight, "\x1b[0;37m");
        assert_eq!(canvas.utf8_table.table.len(), 65536);
        assert_eq!(canvas.bottom_left, 0);
        assert_eq!(canvas.pad, false);
    }

    #[test]
    fn test_set() {
        let mut canvas = new(None);
        let mut rand = Rand::new();
        let n = rand.rand_uint(4, 50);
        canvas.width = n;
        canvas.height = n;
        let mut pixels = vec![vec!['X'; n]; n];
        canvas.set_pixel(&mut pixels, n / 2, n / 2, 'Y');
        assert_eq!(pixels[n / 2][n / 2], 'Y');
    }

    #[test]
    fn test_reset_bottom() {
        let mut canvas = new(None);
        canvas.reset_bottom();
        assert_eq!(canvas.bottom_left, 0);
        assert_eq!(canvas.bottom_right, 0);
        assert_eq!(canvas.pad, false);
    }

    #[test]
    fn test_get_bottom() {
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
        canvas.bottom_left = 4;
        let mut browser = browser::new(".", None, None);
        browser.mode = Mode::Search;
        browser.input_cursor_pos = canvas.bottom_left;
        let mut i = 0;
        for st in texts.iter() {
            browser.search_txt = st.chars().collect::<Vec<char>>();
            // this readjust the bottom_right
            canvas.bottom_right = 0;
            let bottom = canvas.get_bottom(&browser);
            assert_eq!(bottom, texts_configured[i]);
            i += 1;
        }
        // reverse search
        browser.mode = Mode::RevSearch;
        browser.input_cursor_pos = canvas.bottom_left;
        i = 0;
        for st in texts.iter() {
            browser.search_txt = st.chars().collect::<Vec<char>>();
            canvas.bottom_right = 0;
            let bottom = canvas.get_bottom(&browser);
            assert_eq!(bottom, texts_configured_rev[i]);
            i += 1;
        }
    }

    #[test]
    fn test_line_utf8_len() {
        let canvas = new(None);
        assert_eq!(canvas.utf8_len('𰻝'), 2);
        assert_eq!(canvas.utf8_len('ぎ'), 2);
        assert_eq!(canvas.utf8_len(')'), 1);
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
    fn test_draw_bottom() {
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
        canvas.draw_bottom(&mut str_to_draw, &browser);
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
        canvas.reset_bottom();
        let to_crop = 2;
        str_to_draw = String::new();
        canvas.width -= to_crop;
        browser.mode = Mode::Normal;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = Vec::new();
        browser.input_cursor_pos = 0;
        canvas.draw_bottom(&mut str_to_draw, &browser);
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
        canvas.reset_bottom();
        str_to_draw = String::new();
        let text = "foobar";
        let search_txt = text.chars().collect::<Vec<char>>();
        canvas.width = search_txt.len() + 3;
        let cursor_pos = search_txt.len() / 2;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom(&mut str_to_draw, &browser);
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
                    search_txt.len() / 2 + 2 + if canvas.pad { 1 } else { 0 }
                ))
            )
        );
        // search cropped
        canvas.reset_bottom();
        str_to_draw = String::new();
        let text = "foobarfoobar";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = 0;
        canvas.width = search_txt.len() / 2;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom(&mut str_to_draw, &browser);
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
                    .take(canvas.width - (1 + if canvas.pad { 1 } else { 0 }))
                    .collect::<String>(),
                &csi("?25h"),
                &csi(&format!(
                    "0;{}H",
                    cursor_pos + 2 + if canvas.pad { 1 } else { 0 }
                ))
            )
        );
        // search cropped and non-zero cursor placement
        canvas.reset_bottom();
        str_to_draw = String::new();
        let text = "foobarfoobar";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = search_txt.len() - 2; // on the 'a'
        canvas.width = search_txt.len() / 2;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom(&mut str_to_draw, &browser);
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
        canvas.reset_bottom();
        str_to_draw = String::new();
        let text = "从此君王不早朝aaab";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = search_txt.len();
        canvas.width = 18;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom(&mut str_to_draw, &browser);
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
        // search cropped and non-zero cursor placement and UTF8 characters and paddings '>'
        canvas.reset_bottom();
        str_to_draw = String::new();
        let text = "从此君王不早朝aaab";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = search_txt.len();
        canvas.width = 19;
        browser.mode = Mode::Search;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom(&mut str_to_draw, &browser);
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
        canvas.reset_bottom();
        str_to_draw = String::new();
        let text = "从此君王不早朝aaab";
        let search_txt = text.chars().collect::<Vec<char>>();
        let cursor_pos = search_txt.len();
        canvas.width = 18;
        browser.mode = Mode::RevSearch;
        browser.current_path = current_path_buf.clone();
        browser.search_txt = search_txt.clone();
        browser.input_cursor_pos = cursor_pos;
        canvas.draw_bottom(&mut str_to_draw, &browser);
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
        canvas.reset_bottom();
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
        canvas.reset_bottom();
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
        canvas.reset_bottom();
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
        canvas.reset_bottom();
        canvas.draw(&browser, Some(&mut test_out));
        assert_eq!(test_out, "\u{1b}[1H\u{1b}[?25l\u{1b}[38;5;187m\u{1b}[48;5;238m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[38;5;188m\u{1b}[48;5;236m                  \u{1b}[38;5;188m\u{1b}[48;5;236m             \u{1b}[14H\u{1b}[0K\u{1b}[38;5;188m\u{1b}[48;5;238m                               \u{1b}[14H\u{1b}[0K/tmp/ts-test-draw-empty");
    }
}
