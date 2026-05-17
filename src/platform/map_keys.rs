use minifb::{Key, Window};

#[rustfmt::skip] 
const KEY_MAP: [(Key, u8); 16] = [
    (Key::Key1, 0x1), (Key::Key2, 0x2), (Key::Key3, 0x3), (Key::Key4, 0xC),
    (Key::Q, 0x4), (Key::W, 0x5), (Key::E, 0x6), (Key::R, 0xD),
    (Key::A, 0x7), (Key::S, 0x8), (Key::D, 0x9), (Key::F, 0xE),
    (Key::Z, 0xA), (Key::X, 0x0), (Key::C, 0xB), (Key::V, 0xF),
];

pub fn map_keys<F>(window: &Window, mut callback: F)
where
    F: FnMut(u8, bool),
{
    for (host_key, emulator_key) in KEY_MAP {
        if window.is_key_down(host_key) {
            callback(emulator_key, true);
        } else if window.is_key_released(host_key) {
            callback(emulator_key, false);
        }
    }
}
