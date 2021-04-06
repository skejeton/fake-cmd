pub fn unicode_to_cp437(chr: char) -> u8 {
    match chr as usize {
        0x0 => 0, 0x263A => 1, 0x263B => 2, 0x2665 => 3, 0x2666 => 4, 0x2663 => 5, 0x2660 => 6, 0x2022 => 7, 0x25D8 => 8, 0x25CB => 9, 0x25D9 => 10, 0x2642 => 11, 0x2640 => 12, 0x266A => 13, 0x266B => 14, 0x263C => 15, 0x25BA => 16, 0x25C4 => 17, 0x2195 => 18, 0x203C => 19, 0xB6 => 20, 0xA7 => 21, 0x25AC => 22, 0x21A8 => 23, 0x2191 => 24, 0x2193 => 25, 0x2192 => 26, 0x2190 => 27, 0x221F => 28, 0x2194 => 29, 0x25B2 => 30, 0x25BC => 31, 0x20 => 32, 0x21 => 33, 0x22 => 34, 0x23 => 35, 0x24 => 36, 0x25 => 37, 0x26 => 38, 0x27 => 39, 0x28 => 40, 0x29 => 41, 0x2A => 42, 0x2B => 43, 0x2C => 44, 0x2D => 45, 0x2E => 46, 0x2F => 47, 0x30 => 48, 0x31 => 49, 0x32 => 50, 0x33 => 51, 0x34 => 52, 0x35 => 53, 0x36 => 54, 0x37 => 55, 0x38 => 56, 0x39 => 57, 0x3A => 58, 0x3B => 59, 0x3C => 60, 0x3D => 61, 0x3E => 62, 0x3F => 63, 0x40 => 64, 0x41 => 65, 0x42 => 66, 0x43 => 67, 0x44 => 68, 0x45 => 69, 0x46 => 70, 0x47 => 71, 0x48 => 72, 0x49 => 73, 0x4A => 74, 0x4B => 75, 0x4C => 76, 0x4D => 77, 0x4E => 78, 0x4F => 79, 0x50 => 80, 0x51 => 81, 0x52 => 82, 0x53 => 83, 0x54 => 84, 0x55 => 85, 0x56 => 86, 0x57 => 87, 0x58 => 88, 0x59 => 89, 0x5A => 90, 0x5B => 91, 0x5C => 92, 0x5D => 93, 0x5E => 94, 0x5F => 95, 0x60 => 96, 0x61 => 97, 0x62 => 98, 0x63 => 99, 0x64 => 100, 0x65 => 101, 0x66 => 102, 0x67 => 103, 0x68 => 104, 0x69 => 105, 0x6A => 106, 0x6B => 107, 0x6C => 108, 0x6D => 109, 0x6E => 110, 0x6F => 111, 0x70 => 112, 0x71 => 113, 0x72 => 114, 0x73 => 115, 0x74 => 116, 0x75 => 117, 0x76 => 118, 0x77 => 119, 0x78 => 120, 0x79 => 121, 0x7A => 122, 0x7B => 123, 0x7C => 124, 0x7D => 125, 0x7E => 126, 0x2302 => 127, 0xC7 => 128, 0xFC => 129, 0xE9 => 130, 0xE2 => 131, 0xE4 => 132, 0xE0 => 133, 0xE5 => 134, 0xE7 => 135, 0xEA => 136, 0xEB => 137, 0xE8 => 138, 0xEF => 139, 0xEE => 140, 0xEC => 141, 0xC4 => 142, 0xC5 => 143, 0xC9 => 144, 0xE6 => 145, 0xC6 => 146, 0xF4 => 147, 0xF6 => 148, 0xF2 => 149, 0xFB => 150, 0xF9 => 151, 0xFF => 152, 0xD6 => 153, 0xDC => 154, 0xA2 => 155, 0xA3 => 156, 0xA5 => 157, 0x20A7 => 158, 0x192 => 159, 0xE1 => 160, 0xED => 161, 0xF3 => 162, 0xFA => 163, 0xF1 => 164, 0xD1 => 165, 0xAA => 166, 0xBA => 167, 0xBF => 168, 0x2310 => 169, 0xAC => 170, 0xBD => 171, 0xBC => 172, 0xA1 => 173, 0xAB => 174, 0xBB => 175, 0x2591 => 176, 0x2592 => 177, 0x2593 => 178, 0x2502 => 179, 0x2524 => 180, 0x2561 => 181, 0x2562 => 182, 0x2556 => 183, 0x2555 => 184, 0x2563 => 185, 0x2551 => 186, 0x2557 => 187, 0x255D => 188, 0x255C => 189, 0x255B => 190, 0x2510 => 191, 0x2514 => 192, 0x2534 => 193, 0x252C => 194, 0x251C => 195, 0x2500 => 196, 0x253C => 197, 0x255E => 198, 0x255F => 199, 0x255A => 200, 0x2554 => 201, 0x2569 => 202, 0x2566 => 203, 0x2560 => 204, 0x2550 => 205, 0x256C => 206, 0x2567 => 207, 0x2568 => 208, 0x2564 => 209, 0x2565 => 210, 0x2559 => 211, 0x2558 => 212, 0x2552 => 213, 0x2553 => 214, 0x256B => 215, 0x256A => 216, 0x2518 => 217, 0x250C => 218, 0x2588 => 219, 0x2584 => 220, 0x258C => 221, 0x2590 => 222, 0x2580 => 223, 0x3B1 => 224, 0xDF => 225, 0x393 => 226, 0x3C0 => 227, 0x3A3 => 228, 0x3C3 => 229, 0xB5 => 230, 0x3C4 => 231, 0x3A6 => 232, 0x398 => 233, 0x3A9 => 234, 0x3B4 => 235, 0x221E => 236, 0x3C6 => 237, 0x3B5 => 238, 0x2229 => 239, 0x2261 => 240, 0xB1 => 241, 0x2265 => 242, 0x2264 => 243, 0x2320 => 244, 0x2321 => 245, 0xF7 => 246, 0x2248 => 247, 0xB0 => 248, 0x2219 => 249, 0xB7 => 250, 0x221A => 251, 0x207F => 252, 0xB2 => 253, 0x25A0 => 254, 0xA0 => 255,
        c @ 0..=255 => c as u8,
        _ => 0,
    }
}

pub fn cp437_to_unicode(b: u8) -> char {
    std::char::from_u32(match b {
        b'\n' => '\n' as u32,
        b'\r' => '\r' as u32,
        b'\t' => '\t' as u32,
0 => 0x0, 1 => 0x263A, 2 => 0x263B, 3 => 0x2665, 4 => 0x2666, 5 => 0x2663, 6 => 0x2660, 7 => 0x2022, 8 => 0x25D8, 9 => 0x25CB, 10 => 0x25D9, 11 => 0x2642, 12 => 0x2640, 13 => 0x266A, 14 => 0x266B, 15 => 0x263C, 16 => 0x25BA, 17 => 0x25C4, 18 => 0x2195, 19 => 0x203C, 20 => 0xB6, 21 => 0xA7, 22 => 0x25AC, 23 => 0x21A8, 24 => 0x2191, 25 => 0x2193, 26 => 0x2192, 27 => 0x2190, 28 => 0x221F, 29 => 0x2194, 30 => 0x25B2, 31 => 0x25BC, 32 => 0x20, 33 => 0x21, 34 => 0x22, 35 => 0x23, 36 => 0x24, 37 => 0x25, 38 => 0x26, 39 => 0x27, 40 => 0x28, 41 => 0x29, 42 => 0x2A, 43 => 0x2B, 44 => 0x2C, 45 => 0x2D, 46 => 0x2E, 47 => 0x2F, 48 => 0x30, 49 => 0x31, 50 => 0x32, 51 => 0x33, 52 => 0x34, 53 => 0x35, 54 => 0x36, 55 => 0x37, 56 => 0x38, 57 => 0x39, 58 => 0x3A, 59 => 0x3B, 60 => 0x3C, 61 => 0x3D, 62 => 0x3E, 63 => 0x3F, 64 => 0x40, 65 => 0x41, 66 => 0x42, 67 => 0x43, 68 => 0x44, 69 => 0x45, 70 => 0x46, 71 => 0x47, 72 => 0x48, 73 => 0x49, 74 => 0x4A, 75 => 0x4B, 76 => 0x4C, 77 => 0x4D, 78 => 0x4E, 79 => 0x4F, 80 => 0x50, 81 => 0x51, 82 => 0x52, 83 => 0x53, 84 => 0x54, 85 => 0x55, 86 => 0x56, 87 => 0x57, 88 => 0x58, 89 => 0x59, 90 => 0x5A, 91 => 0x5B, 92 => 0x5C, 93 => 0x5D, 94 => 0x5E, 95 => 0x5F, 96 => 0x60, 97 => 0x61, 98 => 0x62, 99 => 0x63, 100 => 0x64, 101 => 0x65, 102 => 0x66, 103 => 0x67, 104 => 0x68, 105 => 0x69, 106 => 0x6A, 107 => 0x6B, 108 => 0x6C, 109 => 0x6D, 110 => 0x6E, 111 => 0x6F, 112 => 0x70, 113 => 0x71, 114 => 0x72, 115 => 0x73, 116 => 0x74, 117 => 0x75, 118 => 0x76, 119 => 0x77, 120 => 0x78, 121 => 0x79, 122 => 0x7A, 123 => 0x7B, 124 => 0x7C, 125 => 0x7D, 126 => 0x7E, 127 => 0x2302, 128 => 0xC7, 129 => 0xFC, 130 => 0xE9, 131 => 0xE2, 132 => 0xE4, 133 => 0xE0, 134 => 0xE5, 135 => 0xE7, 136 => 0xEA, 137 => 0xEB, 138 => 0xE8, 139 => 0xEF, 140 => 0xEE, 141 => 0xEC, 142 => 0xC4, 143 => 0xC5, 144 => 0xC9, 145 => 0xE6, 146 => 0xC6, 147 => 0xF4, 148 => 0xF6, 149 => 0xF2, 150 => 0xFB, 151 => 0xF9, 152 => 0xFF, 153 => 0xD6, 154 => 0xDC, 155 => 0xA2, 156 => 0xA3, 157 => 0xA5, 158 => 0x20A7, 159 => 0x192, 160 => 0xE1, 161 => 0xED, 162 => 0xF3, 163 => 0xFA, 164 => 0xF1, 165 => 0xD1, 166 => 0xAA, 167 => 0xBA, 168 => 0xBF, 169 => 0x2310, 170 => 0xAC, 171 => 0xBD, 172 => 0xBC, 173 => 0xA1, 174 => 0xAB, 175 => 0xBB, 176 => 0x2591, 177 => 0x2592, 178 => 0x2593, 179 => 0x2502, 180 => 0x2524, 181 => 0x2561, 182 => 0x2562, 183 => 0x2556, 184 => 0x2555, 185 => 0x2563, 186 => 0x2551, 187 => 0x2557, 188 => 0x255D, 189 => 0x255C, 190 => 0x255B, 191 => 0x2510, 192 => 0x2514, 193 => 0x2534, 194 => 0x252C, 195 => 0x251C, 196 => 0x2500, 197 => 0x253C, 198 => 0x255E, 199 => 0x255F, 200 => 0x255A, 201 => 0x2554, 202 => 0x2569, 203 => 0x2566, 204 => 0x2560, 205 => 0x2550, 206 => 0x256C, 207 => 0x2567, 208 => 0x2568, 209 => 0x2564, 210 => 0x2565, 211 => 0x2559, 212 => 0x2558, 213 => 0x2552, 214 => 0x2553, 215 => 0x256B, 216 => 0x256A, 217 => 0x2518, 218 => 0x250C, 219 => 0x2588, 220 => 0x2584, 221 => 0x258C, 222 => 0x2590, 223 => 0x2580, 224 => 0x3B1, 225 => 0xDF, 226 => 0x393, 227 => 0x3C0, 228 => 0x3A3, 229 => 0x3C3, 230 => 0xB5, 231 => 0x3C4, 232 => 0x3A6, 233 => 0x398, 234 => 0x3A9, 235 => 0x3B4, 236 => 0x221E, 237 => 0x3C6, 238 => 0x3B5, 239 => 0x2229, 240 => 0x2261, 241 => 0xB1, 242 => 0x2265, 243 => 0x2264, 244 => 0x2320, 245 => 0x2321, 246 => 0xF7, 247 => 0x2248, 248 => 0xB0, 249 => 0x2219, 250 => 0xB7, 251 => 0x221A, 252 => 0x207F, 253 => 0xB2, 254 => 0x25A0, 255 => 0xA0
    }).unwrap()
}

#[cfg(target_os="windows")]
pub fn from_cp437_if_windows(bytes: &[u8]) -> String {
    bytes.iter().copied().map(|e| cp437_to_unicode(e)).collect::<String>()
}

#[cfg(not(target_os="windows"))]
pub fn from_cp437_if_windows(bytes: &[u8]) -> String {
    unsafe { std::str::from_utf8_unchecked(bytes).to_string() }
}
