pub struct U8Range {
    pub l: usize,
    pub r: usize,
    pub is_wide: bool,
}

pub const UTF8_TBL: [U8Range; 308] = [
    U8Range {
        l: 0x0000,
        r: 0x007F,
        is_wide: false,
    },
    U8Range {
        l: 0x0080,
        r: 0x00FF,
        is_wide: false,
    },
    U8Range {
        l: 0x00100,
        r: 0x0017F,
        is_wide: false,
    },
    U8Range {
        l: 0x00180,
        r: 0x0024F,
        is_wide: false,
    },
    U8Range {
        l: 0x0250,
        r: 0x02AF,
        is_wide: false,
    },
    U8Range {
        l: 0x02B0,
        r: 0x02FF,
        is_wide: false,
    },
    U8Range {
        // Combining Diacritical Marks (problem: some are even 3-character-wide)
        l: 0x0300,
        r: 0x036F,
        is_wide: true,
    },
    U8Range {
        l: 0x0370,
        r: 0x03FF,
        is_wide: false,
    },
    U8Range {
        // Cyrillic (problem: most are non-wide, only one or two are wide)
        l: 0x0400,
        r: 0x04FF,
        is_wide: false,
    },
    U8Range {
        l: 0x0500,
        r: 0x052F,
        is_wide: false,
    },
    U8Range {
        l: 0x0530,
        r: 0x058F,
        is_wide: false,
    },
    U8Range {
        l: 0x0590,
        r: 0x05FF,
        is_wide: false,
    },
    U8Range {
        // Arabic (problem: most are non-wide, only one or two are wide)
        l: 0x0600,
        r: 0x06FF,
        is_wide: false,
    },
    U8Range {
        // Syriac (problem: most are non-wide, only one or two are wide)
        l: 0x0700,
        r: 0x074F,
        is_wide: false,
    },
    U8Range {
        l: 0x0750,
        r: 0x077F,
        is_wide: false,
    },
    U8Range {
        // Thaana (problem: most are non-wide, only one or two are wide)
        l: 0x0780,
        r: 0x07BF,
        is_wide: false,
    },
    U8Range {
        // NKo (problem: most are non-wide, only one or two are wide)
        l: 0x07C0,
        r: 0x07FF,
        is_wide: false,
    },
    U8Range {
        // Samaritan (problem: most are non-wide, only one or two are wide)
        l: 0x0800,
        r: 0x083F,
        is_wide: false,
    },
    U8Range {
        // Mandaic (problem: most are non-wide, only one or two are wide)
        l: 0x0840,
        r: 0x085F,
        is_wide: false,
    },
    U8Range {
        l: 0x0860,
        r: 0x086F,
        is_wide: false,
    },
    U8Range {
        l: 0x08A0,
        r: 0x08FF,
        is_wide: false,
    },
    U8Range {
        l: 0x0900,
        r: 0x097F,
        is_wide: false,
    },
    U8Range {
        l: 0x0980,
        r: 0x09FF,
        is_wide: false,
    },
    U8Range {
        // Gurmukhi (problem: most are non-wide, only one or two are wide)
        l: 0x0A00,
        r: 0x0A7F,
        is_wide: false,
    },
    U8Range {
        l: 0x0A80,
        r: 0x0AFF,
        is_wide: false,
    },
    U8Range {
        l: 0x0B00,
        r: 0x0B7F,
        is_wide: false,
    },
    U8Range {
        l: 0x0B80,
        r: 0x0BFF,
        is_wide: false,
    },
    U8Range {
        // Telugu (problem: most are non-wide, only one or two are wide)
        l: 0x0C00,
        r: 0x0C7F,
        is_wide: false,
    },
    U8Range {
        // Kannada (problem: most are non-wide, only one or two are wide)
        l: 0x0C80,
        r: 0x0CFF,
        is_wide: false,
    },
    U8Range {
        l: 0x0D00,
        r: 0x0D7F,
        is_wide: false,
    },
    U8Range {
        // Sinhala (problem: most are non-wide, only one or two are wide)
        l: 0x0D80,
        r: 0x0DFF,
        is_wide: false,
    },
    U8Range {
        // Thai (problem: most are non-wide, only one or two are wide)
        l: 0x0E00,
        r: 0x0E7F,
        is_wide: false,
    },
    U8Range {
        // Lao (problem: most are non-wide, only one or two are wide)
        l: 0x0E80,
        r: 0x0EFF,
        is_wide: false,
    },
    U8Range {
        // Tibetan (problem: most are non-wide, only one or two are wide)
        l: 0x0F00,
        r: 0x0FFF,
        is_wide: false,
    },
    U8Range {
        l: 0x1000,
        r: 0x109F,
        is_wide: false,
    },
    U8Range {
        l: 0x10A0,
        r: 0x10FF,
        is_wide: false,
    },
    U8Range {
        // Hangul Jamo
        l: 0x1100,
        r: 0x11FF,
        is_wide: true,
    },
    U8Range {
        l: 0x1200,
        r: 0x137F,
        is_wide: false,
    },
    U8Range {
        l: 0x1380,
        r: 0x139F,
        is_wide: false,
    },
    U8Range {
        l: 0x13A0,
        r: 0x13FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1400,
        r: 0x167F,
        is_wide: false,
    },
    U8Range {
        l: 0x1680,
        r: 0x169F,
        is_wide: false,
    },
    U8Range {
        l: 0x16A0,
        r: 0x16FF,
        is_wide: false,
    },
    U8Range {
        // Tagalog (problem: most are non-wide, only one or two are wide)
        l: 0x1700,
        r: 0x171F,
        is_wide: false,
    },
    U8Range {
        l: 0x1720,
        r: 0x173F,
        is_wide: false,
    },
    U8Range {
        l: 0x1740,
        r: 0x175F,
        is_wide: false,
    },
    U8Range {
        // Tagbanwa (problem: most are non-wide, only one or two are wide)
        l: 0x1760,
        r: 0x177F,
        is_wide: false,
    },
    U8Range {
        l: 0x1780,
        r: 0x17FF,
        is_wide: false,
    },
    U8Range {
        // Mongolian (problem: most are non-wide, only one or two are wide)
        l: 0x1800,
        r: 0x18AF,
        is_wide: false,
    },
    U8Range {
        l: 0x18B0,
        r: 0x18FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1900,
        r: 0x194F,
        is_wide: false,
    },
    U8Range {
        l: 0x1950,
        r: 0x197F,
        is_wide: false,
    },
    U8Range {
        l: 0x1980,
        r: 0x19DF,
        is_wide: false,
    },
    U8Range {
        l: 0x19E0,
        r: 0x19FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1A00,
        r: 0x1A1F,
        is_wide: false,
    },
    U8Range {
        // Tai Tham (problem: not all wide)
        l: 0x1A20,
        r: 0x1AAF,
        is_wide: true,
    },
    U8Range {
        // Combining Diacritical Marks Extended (problem: not all wide (but only one not wide))
        l: 0x1AB0,
        r: 0x1AFF,
        is_wide: true,
    },
    U8Range {
        l: 0x1B00,
        r: 0x1B7F,
        is_wide: false,
    },
    U8Range {
        l: 0x1B80,
        r: 0x1BBF,
        is_wide: false,
    },
    U8Range {
        l: 0x1BC0,
        r: 0x1BFF,
        is_wide: false,
    },
    U8Range {
        l: 0x1C00,
        r: 0x1C4F,
        is_wide: false,
    },
    U8Range {
        l: 0x1C50,
        r: 0x1C7F,
        is_wide: false,
    },
    U8Range {
        l: 0x1C80,
        r: 0x1C8F,
        is_wide: false,
    },
    U8Range {
        l: 0x1C90,
        r: 0x1CBF,
        is_wide: false,
    },
    U8Range {
        l: 0x1CC0,
        r: 0x1CCF,
        is_wide: false,
    },
    U8Range {
        // Vedic Extensions (problem: not all wide)
        l: 0x1CD0,
        r: 0x1CFF,
        is_wide: true,
    },
    U8Range {
        l: 0x1D00,
        r: 0x1D7F,
        is_wide: false,
    },
    U8Range {
        l: 0x1D80,
        r: 0x1DBF,
        is_wide: false,
    },
    U8Range {
        // Combining Diacritical Marks Supplement
        l: 0x1DC0,
        r: 0x1DFF,
        is_wide: true,
    },
    U8Range {
        l: 0x1E00,
        r: 0x1EFF,
        is_wide: false,
    },
    U8Range {
        l: 0x1F00,
        r: 0x1FFF,
        is_wide: false,
    },
    U8Range {
        l: 0x2000,
        r: 0x206F,
        is_wide: false,
    },
    U8Range {
        l: 0x2070,
        r: 0x209F,
        is_wide: false,
    },
    U8Range {
        l: 0x20A0,
        r: 0x20CF,
        is_wide: false,
    },
    U8Range {
        // Combining Diacritical Marks for Symbols (problem: not all wide)
        l: 0x20D0,
        r: 0x20FF,
        is_wide: true,
    },
    U8Range {
        l: 0x2100,
        r: 0x214F,
        is_wide: false,
    },
    U8Range {
        l: 0x2150,
        r: 0x218F,
        is_wide: false,
    },
    U8Range {
        l: 0x2190,
        r: 0x21FF,
        is_wide: false,
    },
    U8Range {
        l: 0x2200,
        r: 0x22FF,
        is_wide: false,
    },
    U8Range {
        // Miscellaneous Technical (problem: emojis in, not all wide)
        l: 0x2300,
        r: 0x23FF,
        is_wide: true,
    },
    U8Range {
        l: 0x2400,
        r: 0x243F,
        is_wide: false,
    },
    U8Range {
        l: 0x2440,
        r: 0x245F,
        is_wide: false,
    },
    U8Range {
        l: 0x2460,
        r: 0x24FF,
        is_wide: false,
    },
    U8Range {
        l: 0x2500,
        r: 0x257F,
        is_wide: false,
    },
    U8Range {
        l: 0x2580,
        r: 0x259F,
        is_wide: false,
    },
    U8Range {
        // Geometric Shapes (problem: not all wide)
        l: 0x25A0,
        r: 0x25FF,
        is_wide: true,
    },
    U8Range {
        // Miscellaneous Symbols (problem: emojis in, not all wide)
        l: 0x2600,
        r: 0x26FF,
        is_wide: true,
    },
    U8Range {
        // Dingbats (problem: emojis in, not all wide)
        l: 0x2700,
        r: 0x27BF,
        is_wide: true,
    },
    U8Range {
        l: 0x27C0,
        r: 0x27EF,
        is_wide: false,
    },
    U8Range {
        l: 0x27F0,
        r: 0x27FF,
        is_wide: false,
    },
    U8Range {
        l: 0x2800,
        r: 0x28FF,
        is_wide: false,
    },
    U8Range {
        l: 0x2900,
        r: 0x297F,
        is_wide: false,
    },
    U8Range {
        l: 0x2980,
        r: 0x29FF,
        is_wide: false,
    },
    U8Range {
        l: 0x2A00,
        r: 0x2AFF,
        is_wide: false,
    },
    U8Range {
        // Miscellaneous Symbols and Arrows (problem: emojis in, some are not wide)
        l: 0x2B00,
        r: 0x2BFF,
        is_wide: true,
    },
    U8Range {
        l: 0x2C00,
        r: 0x2C5F,
        is_wide: false,
    },
    U8Range {
        l: 0x2C60,
        r: 0x2C7F,
        is_wide: false,
    },
    U8Range {
        l: 0x2C80,
        r: 0x2CFF,
        is_wide: false,
    },
    U8Range {
        l: 0x2D00,
        r: 0x2D2F,
        is_wide: false,
    },
    U8Range {
        l: 0x2D30,
        r: 0x2D7F,
        is_wide: false,
    },
    U8Range {
        l: 0x2D80,
        r: 0x2DDF,
        is_wide: false,
    },
    U8Range {
        // Cyrillic Extended-A
        l: 0x2DE0,
        r: 0x2DFF,
        is_wide: true,
    },
    U8Range {
        l: 0x2E00,
        r: 0x2E7F,
        is_wide: false,
    },
    U8Range {
        // CJK Radicals Supplement
        l: 0x2E80,
        r: 0x2EFF,
        is_wide: true,
    },
    U8Range {
        // Kangxi Radicals
        l: 0x2F00,
        r: 0x2FDF,
        is_wide: true,
    },
    U8Range {
        // Ideographic Description Characters
        l: 0x2FF0,
        r: 0x2FFF,
        is_wide: true,
    },
    U8Range {
        // CJK Symbols and Punctuation
        l: 0x3000,
        r: 0x303F,
        is_wide: true,
    },
    U8Range {
        // Hiragana
        l: 0x3040,
        r: 0x309F,
        is_wide: true,
    },
    U8Range {
        // Katakana
        l: 0x30A0,
        r: 0x30FF,
        is_wide: true,
    },
    U8Range {
        // Bopomofo
        l: 0x3100,
        r: 0x312F,
        is_wide: true,
    },
    U8Range {
        // Hangul Compatibility Jamo
        l: 0x3130,
        r: 0x318F,
        is_wide: true,
    },
    U8Range {
        // Kanbun
        l: 0x3190,
        r: 0x319F,
        is_wide: true,
    },
    U8Range {
        // Bopomofo Extended
        l: 0x31A0,
        r: 0x31BF,
        is_wide: true,
    },
    U8Range {
        // CJK Strokes
        l: 0x31C0,
        r: 0x31EF,
        is_wide: true,
    },
    U8Range {
        // Katakana Phonetic Extensions
        l: 0x31F0,
        r: 0x31FF,
        is_wide: true,
    },
    U8Range {
        // Enclosed CJK Letters and Months
        l: 0x3200,
        r: 0x32FF,
        is_wide: true,
    },
    U8Range {
        // CJK Compatibility
        l: 0x3300,
        r: 0x33FF,
        is_wide: true,
    },
    U8Range {
        // CJK Unified Ideographs Extension A
        l: 0x3400,
        r: 0x4DBF,
        is_wide: true,
    },
    U8Range {
        l: 0x4DC0,
        r: 0x4DFF,
        is_wide: false,
    },
    U8Range {
        // CJK Unified Ideographs
        l: 0x4E00,
        r: 0x9FFF,
        is_wide: true,
    },
    U8Range {
        // Yi Syllables
        l: 0xA000,
        r: 0xA48F,
        is_wide: true,
    },
    U8Range {
        // Yi Radicals
        l: 0xA490,
        r: 0xA4CF,
        is_wide: true,
    },
    U8Range {
        l: 0xA4D0,
        r: 0xA4FF,
        is_wide: false,
    },
    U8Range {
        l: 0xA500,
        r: 0xA63F,
        is_wide: false,
    },
    U8Range {
        l: 0xA640,
        r: 0xA69F,
        is_wide: false,
    },
    U8Range {
        l: 0xA6A0,
        r: 0xA6FF,
        is_wide: false,
    },
    U8Range {
        l: 0xA700,
        r: 0xA71F,
        is_wide: false,
    },
    U8Range {
        l: 0xA720,
        r: 0xA7FF,
        is_wide: false,
    },
    U8Range {
        l: 0xA800,
        r: 0xA82F,
        is_wide: false,
    },
    U8Range {
        l: 0xA830,
        r: 0xA83F,
        is_wide: false,
    },
    U8Range {
        l: 0xA840,
        r: 0xA87F,
        is_wide: false,
    },
    U8Range {
        l: 0xA880,
        r: 0xA8DF,
        is_wide: false,
    },
    U8Range {
        l: 0xA8E0,
        r: 0xA8FF,
        is_wide: false,
    },
    U8Range {
        l: 0xA900,
        r: 0xA92F,
        is_wide: false,
    },
    U8Range {
        l: 0xA930,
        r: 0xA95F,
        is_wide: false,
    },
    U8Range {
        // Hangul Jamo Extended-A
        l: 0xA960,
        r: 0xA97F,
        is_wide: true,
    },
    U8Range {
        l: 0xA980,
        r: 0xA9DF,
        is_wide: false,
    },
    U8Range {
        l: 0xA9E0,
        r: 0xA9FF,
        is_wide: false,
    },
    U8Range {
        l: 0xAA00,
        r: 0xAA5F,
        is_wide: false,
    },
    U8Range {
        l: 0xAA60,
        r: 0xAA7F,
        is_wide: false,
    },
    U8Range {
        l: 0xAA80,
        r: 0xAADF,
        is_wide: false,
    },
    U8Range {
        l: 0xAAE0,
        r: 0xAAFF,
        is_wide: false,
    },
    U8Range {
        l: 0xAB00,
        r: 0xAB2F,
        is_wide: false,
    },
    U8Range {
        l: 0xAB30,
        r: 0xAB6F,
        is_wide: false,
    },
    U8Range {
        l: 0xAB70,
        r: 0xABBF,
        is_wide: false,
    },
    U8Range {
        l: 0xABC0,
        r: 0xABFF,
        is_wide: false,
    },
    U8Range {
        // Hangul Syllables
        l: 0xAC00,
        r: 0xD7AF,
        is_wide: true,
    },
    U8Range {
        l: 0xD7B0,
        r: 0xD7FF,
        is_wide: false,
    },
    U8Range {
        l: 0xD800,
        r: 0xDB7F,
        is_wide: false,
    },
    U8Range {
        l: 0xDB80,
        r: 0xDBFF,
        is_wide: false,
    },
    U8Range {
        l: 0xDC00,
        r: 0xDFFF,
        is_wide: false,
    },
    U8Range {
        l: 0xE000,
        r: 0xF8FF,
        is_wide: false,
    },
    U8Range {
        // CJK Compatibility Ideographs
        l: 0xF900,
        r: 0xFAFF,
        is_wide: true,
    },
    U8Range {
        l: 0xFB00,
        r: 0xFB4F,
        is_wide: false,
    },
    U8Range {
        l: 0xFB50,
        r: 0xFDFF,
        is_wide: false,
    },
    U8Range {
        l: 0xFE00,
        r: 0xFE0F,
        is_wide: false,
    },
    U8Range {
        // Vertical Forms
        l: 0xFE10,
        r: 0xFE1F,
        is_wide: true,
    },
    U8Range {
        l: 0xFE20,
        r: 0xFE2F,
        is_wide: false,
    },
    U8Range {
        // CJK Compatibility Forms
        l: 0xFE30,
        r: 0xFE4F,
        is_wide: true,
    },
    U8Range {
        // Small Form Variants
        l: 0xFE50,
        r: 0xFE6F,
        is_wide: true,
    },
    U8Range {
        l: 0xFE70,
        r: 0xFEFF,
        is_wide: false,
    },
    U8Range {
        // Halfwidth and Fullwidth Forms (Chinese punctuation)
        l: 0xFF00,
        r: 0xFFEF,
        is_wide: true,
    },
    U8Range {
        l: 0xFFF0,
        r: 0xFFFF,
        is_wide: false,
    },
    U8Range {
        l: 0x10000,
        r: 0x1007F,
        is_wide: false,
    },
    U8Range {
        l: 0x10080,
        r: 0x100FF,
        is_wide: false,
    },
    U8Range {
        l: 0x10100,
        r: 0x1013F,
        is_wide: false,
    },
    U8Range {
        l: 0x10140,
        r: 0x1018F,
        is_wide: false,
    },
    U8Range {
        l: 0x10190,
        r: 0x101CF,
        is_wide: false,
    },
    U8Range {
        l: 0x101D0,
        r: 0x101FF,
        is_wide: false,
    },
    U8Range {
        l: 0x10280,
        r: 0x1029F,
        is_wide: false,
    },
    U8Range {
        l: 0x102A0,
        r: 0x102DF,
        is_wide: false,
    },
    U8Range {
        l: 0x102E0,
        r: 0x102FF,
        is_wide: false,
    },
    U8Range {
        l: 0x10300,
        r: 0x1032F,
        is_wide: false,
    },
    U8Range {
        l: 0x10330,
        r: 0x1034F,
        is_wide: false,
    },
    U8Range {
        l: 0x10350,
        r: 0x1037F,
        is_wide: false,
    },
    U8Range {
        l: 0x10380,
        r: 0x1039F,
        is_wide: false,
    },
    U8Range {
        l: 0x103A0,
        r: 0x103DF,
        is_wide: false,
    },
    U8Range {
        l: 0x10400,
        r: 0x1044F,
        is_wide: false,
    },
    U8Range {
        l: 0x10450,
        r: 0x1047F,
        is_wide: false,
    },
    U8Range {
        l: 0x10480,
        r: 0x104AF,
        is_wide: false,
    },
    U8Range {
        l: 0x104B0,
        r: 0x104FF,
        is_wide: false,
    },
    U8Range {
        l: 0x10500,
        r: 0x1052F,
        is_wide: false,
    },
    U8Range {
        l: 0x10530,
        r: 0x1056F,
        is_wide: false,
    },
    U8Range {
        l: 0x10600,
        r: 0x1077F,
        is_wide: false,
    },
    U8Range {
        l: 0x10800,
        r: 0x1083F,
        is_wide: false,
    },
    U8Range {
        l: 0x10840,
        r: 0x1085F,
        is_wide: false,
    },
    U8Range {
        l: 0x10860,
        r: 0x1087F,
        is_wide: false,
    },
    U8Range {
        l: 0x10880,
        r: 0x108AF,
        is_wide: false,
    },
    U8Range {
        l: 0x108E0,
        r: 0x108FF,
        is_wide: false,
    },
    U8Range {
        l: 0x10900,
        r: 0x1091F,
        is_wide: false,
    },
    U8Range {
        l: 0x10920,
        r: 0x1093F,
        is_wide: false,
    },
    U8Range {
        l: 0x10980,
        r: 0x1099F,
        is_wide: false,
    },
    U8Range {
        l: 0x109A0,
        r: 0x109FF,
        is_wide: false,
    },
    U8Range {
        l: 0x10A00,
        r: 0x10A5F,
        is_wide: false,
    },
    U8Range {
        l: 0x10A60,
        r: 0x10A7F,
        is_wide: false,
    },
    U8Range {
        l: 0x10A80,
        r: 0x10A9F,
        is_wide: false,
    },
    U8Range {
        l: 0x10AC0,
        r: 0x10AFF,
        is_wide: false,
    },
    U8Range {
        l: 0x10B00,
        r: 0x10B3F,
        is_wide: false,
    },
    U8Range {
        l: 0x10B40,
        r: 0x10B5F,
        is_wide: false,
    },
    U8Range {
        l: 0x10B60,
        r: 0x10B7F,
        is_wide: false,
    },
    U8Range {
        l: 0x10B80,
        r: 0x10BAF,
        is_wide: false,
    },
    U8Range {
        l: 0x10C00,
        r: 0x10C4F,
        is_wide: false,
    },
    U8Range {
        l: 0x10C80,
        r: 0x10CFF,
        is_wide: false,
    },
    U8Range {
        l: 0x10D00,
        r: 0x10D3F,
        is_wide: false,
    },
    U8Range {
        l: 0x10E60,
        r: 0x10E7F,
        is_wide: false,
    },
    U8Range {
        l: 0x10E80,
        r: 0x10EBF,
        is_wide: false,
    },
    U8Range {
        l: 0x10F00,
        r: 0x10F2F,
        is_wide: false,
    },
    U8Range {
        l: 0x10F30,
        r: 0x10F6F,
        is_wide: false,
    },
    U8Range {
        l: 0x10FB0,
        r: 0x10FDF,
        is_wide: false,
    },
    U8Range {
        l: 0x10FE0,
        r: 0x10FFF,
        is_wide: false,
    },
    U8Range {
        l: 0x11000,
        r: 0x1107F,
        is_wide: false,
    },
    U8Range {
        l: 0x11080,
        r: 0x110CF,
        is_wide: false,
    },
    U8Range {
        l: 0x110D0,
        r: 0x110FF,
        is_wide: false,
    },
    U8Range {
        l: 0x11100,
        r: 0x1114F,
        is_wide: false,
    },
    U8Range {
        l: 0x11150,
        r: 0x1117F,
        is_wide: false,
    },
    U8Range {
        l: 0x11180,
        r: 0x111DF,
        is_wide: false,
    },
    U8Range {
        l: 0x111E0,
        r: 0x111FF,
        is_wide: false,
    },
    U8Range {
        l: 0x11200,
        r: 0x1124F,
        is_wide: false,
    },
    U8Range {
        l: 0x11280,
        r: 0x112AF,
        is_wide: false,
    },
    U8Range {
        l: 0x112B0,
        r: 0x112FF,
        is_wide: false,
    },
    U8Range {
        l: 0x11300,
        r: 0x1137F,
        is_wide: false,
    },
    U8Range {
        l: 0x11400,
        r: 0x1147F,
        is_wide: false,
    },
    U8Range {
        l: 0x11480,
        r: 0x114DF,
        is_wide: false,
    },
    U8Range {
        l: 0x11580,
        r: 0x115FF,
        is_wide: false,
    },
    U8Range {
        l: 0x11600,
        r: 0x1165F,
        is_wide: false,
    },
    U8Range {
        l: 0x11660,
        r: 0x1167F,
        is_wide: false,
    },
    U8Range {
        l: 0x11680,
        r: 0x116CF,
        is_wide: false,
    },
    U8Range {
        l: 0x11700,
        r: 0x1173F,
        is_wide: false,
    },
    U8Range {
        l: 0x11800,
        r: 0x1184F,
        is_wide: false,
    },
    U8Range {
        l: 0x118A0,
        r: 0x118FF,
        is_wide: false,
    },
    U8Range {
        l: 0x11900,
        r: 0x1195F,
        is_wide: false,
    },
    U8Range {
        l: 0x119A0,
        r: 0x119FF,
        is_wide: false,
    },
    U8Range {
        l: 0x11A00,
        r: 0x11A4F,
        is_wide: false,
    },
    U8Range {
        l: 0x11A50,
        r: 0x11AAF,
        is_wide: false,
    },
    U8Range {
        l: 0x11AC0,
        r: 0x11AFF,
        is_wide: false,
    },
    U8Range {
        l: 0x11C00,
        r: 0x11C6F,
        is_wide: false,
    },
    U8Range {
        l: 0x11C70,
        r: 0x11CBF,
        is_wide: false,
    },
    U8Range {
        l: 0x11D00,
        r: 0x11D5F,
        is_wide: false,
    },
    U8Range {
        l: 0x11D60,
        r: 0x11DAF,
        is_wide: false,
    },
    U8Range {
        l: 0x11EE0,
        r: 0x11EFF,
        is_wide: false,
    },
    U8Range {
        l: 0x11FB0,
        r: 0x11FBF,
        is_wide: false,
    },
    U8Range {
        l: 0x11FC0,
        r: 0x11FFF,
        is_wide: false,
    },
    U8Range {
        // Cuneiform (may need 3 blocks to display)
        l: 0x12000,
        r: 0x123FF,
        is_wide: false,
    },
    U8Range {
        l: 0x12400,
        r: 0x1247F,
        is_wide: false,
    },
    U8Range {
        l: 0x12480,
        r: 0x1254F,
        is_wide: false,
    },
    U8Range {
        l: 0x13000,
        r: 0x1342F,
        is_wide: false,
    },
    U8Range {
        l: 0x13430,
        r: 0x1343F,
        is_wide: false,
    },
    U8Range {
        l: 0x14400,
        r: 0x1467F,
        is_wide: false,
    },
    U8Range {
        l: 0x16800,
        r: 0x16A3F,
        is_wide: false,
    },
    U8Range {
        l: 0x16A40,
        r: 0x16A6F,
        is_wide: false,
    },
    U8Range {
        l: 0x16AD0,
        r: 0x16AFF,
        is_wide: false,
    },
    U8Range {
        l: 0x16B00,
        r: 0x16B8F,
        is_wide: false,
    },
    U8Range {
        l: 0x16E40,
        r: 0x16E9F,
        is_wide: false,
    },
    U8Range {
        l: 0x16F00,
        r: 0x16F9F,
        is_wide: false,
    },
    U8Range {
        l: 0x16FE0,
        r: 0x16FFF,
        is_wide: false,
    },
    U8Range {
        // Tangut (didn't see the symbols)
        l: 0x17000,
        r: 0x187FF,
        is_wide: true,
    },
    U8Range {
        // Tangut Components
        l: 0x18800,
        r: 0x18AFF,
        is_wide: true,
    },
    U8Range {
        l: 0x18B00,
        r: 0x18CFF,
        is_wide: false,
    },
    U8Range {
        // Tangut Supplement
        l: 0x18D00,
        r: 0x18D8F,
        is_wide: true,
    },
    U8Range {
        // Kana Supplement
        l: 0x1B000,
        r: 0x1B0FF,
        is_wide: true,
    },
    U8Range {
        l: 0x1B100,
        r: 0x1B12F,
        is_wide: false,
    },
    U8Range {
        l: 0x1B130,
        r: 0x1B16F,
        is_wide: false,
    },
    U8Range {
        l: 0x1B170,
        r: 0x1B2FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1BC00,
        r: 0x1BC9F,
        is_wide: false,
    },
    U8Range {
        l: 0x1BCA0,
        r: 0x1BCAF,
        is_wide: false,
    },
    U8Range {
        l: 0x1D000,
        r: 0x1D0FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1D100,
        r: 0x1D1FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1D200,
        r: 0x1D24F,
        is_wide: false,
    },
    U8Range {
        l: 0x1D2E0,
        r: 0x1D2FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1D300,
        r: 0x1D35F,
        is_wide: false,
    },
    U8Range {
        l: 0x1D360,
        r: 0x1D37F,
        is_wide: false,
    },
    U8Range {
        l: 0x1D400,
        r: 0x1D7FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1D800,
        r: 0x1DAAF,
        is_wide: false,
    },
    U8Range {
        l: 0x1E000,
        r: 0x1E02F,
        is_wide: false,
    },
    U8Range {
        l: 0x1E100,
        r: 0x1E14F,
        is_wide: false,
    },
    U8Range {
        l: 0x1E2C0,
        r: 0x1E2FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1E800,
        r: 0x1E8DF,
        is_wide: false,
    },
    U8Range {
        l: 0x1E900,
        r: 0x1E95F,
        is_wide: false,
    },
    U8Range {
        l: 0x1EC70,
        r: 0x1ECBF,
        is_wide: false,
    },
    U8Range {
        l: 0x1ED00,
        r: 0x1ED4F,
        is_wide: false,
    },
    U8Range {
        l: 0x1EE00,
        r: 0x1EEFF,
        is_wide: false,
    },
    U8Range {
        l: 0x1F000,
        r: 0x1F02F,
        is_wide: false,
    },
    U8Range {
        l: 0x1F030,
        r: 0x1F09F,
        is_wide: false,
    },
    U8Range {
        l: 0x1F0A0,
        r: 0x1F0FF,
        is_wide: false,
    },
    U8Range {
        // Problem: part of them are emojis, but some of them are half-width, make them all wide here
        l: 0x1F100,
        r: 0x1F1FF,
        is_wide: true,
    },
    U8Range {
        // Enclosed Ideographic Supplement
        l: 0x1F200,
        r: 0x1F2FF,
        is_wide: true,
    },
    U8Range {
        // Emojis (Miscellaneous Symbols and Pictographs)
        l: 0x1F300,
        r: 0x1F5FF,
        is_wide: true,
    },
    U8Range {
        // Emojis
        l: 0x1F600,
        r: 0x1F64F,
        is_wide: true,
    },
    U8Range {
        l: 0x1F650,
        r: 0x1F67F,
        is_wide: false,
    },
    U8Range {
        // Emojis (Transport and Map Symbols)
        l: 0x1F680,
        r: 0x1F6FF,
        is_wide: true,
    },
    U8Range {
        l: 0x1F700,
        r: 0x1F77F,
        is_wide: false,
    },
    U8Range {
        l: 0x1F780,
        r: 0x1F7FF,
        is_wide: false,
    },
    U8Range {
        l: 0x1F800,
        r: 0x1F8FF,
        is_wide: false,
    },
    U8Range {
        // Emojis (Supplemental Symbols and Pictographs)
        l: 0x1F900,
        r: 0x1F9FF,
        is_wide: true,
    },
    U8Range {
        l: 0x1FA00,
        r: 0x1FA6F,
        is_wide: false,
    },
    U8Range {
        // Symbols and Pictographs Extended-A
        l: 0x1FA70,
        r: 0x1FAFF,
        is_wide: true,
    },
    U8Range {
        l: 0x1FB00,
        r: 0x1FBFF,
        is_wide: false,
    },
    U8Range {
        // CJK Ex B
        l: 0x20000,
        r: 0x2A6DF,
        is_wide: true,
    },
    U8Range {
        l: 0x2A700,
        r: 0x2B73F,
        is_wide: true,
    },
    U8Range {
        l: 0x2B740,
        r: 0x2B81F,
        is_wide: true,
    },
    U8Range {
        l: 0x2B820,
        r: 0x2CEAF,
        is_wide: true,
    },
    U8Range {
        // CJK Ex F
        l: 0x2CEB0,
        r: 0x2EBEF,
        is_wide: true,
    },
    U8Range {
        l: 0x2F800,
        r: 0x2FA1F,
        is_wide: true,
    },
    U8Range {
        // CJK Ex G
        l: 0x30000,
        r: 0x3134F,
        is_wide: true,
    },
    U8Range {
        l: 0xE0000,
        r: 0xE007F,
        is_wide: false,
    },
    U8Range {
        l: 0xE0100,
        r: 0xE01EF,
        is_wide: false,
    },
    U8Range {
        l: 0xF0000,
        r: 0xFFFFF,
        is_wide: false,
    },
    U8Range {
        l: 0x100000,
        r: 0x10FFFF,
        is_wide: false,
    },
];
