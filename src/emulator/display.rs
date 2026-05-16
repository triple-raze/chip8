const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

const BYTE_BITS: u8 = 8;

pub struct Display {
    // Each u64 is a Bitmask and it represents one line of pixels.
    pixels: [u64; DISPLAY_HEIGHT], // It'll make rendering much easier because of XORing.
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [0; DISPLAY_HEIGHT],
        }
    }

    pub fn render(&self) -> String {
        const PIXEL_CHAR: char = '\u{2588}';
        const SPACE_CHAR: char = ' ';

        self.pixels
            .iter()
            .map(|&bitmask| {
                let mut line = (0..64)
                    .rev()
                    .map(|offset| {
                        if bitmask >> offset & 1 == 1 {
                            PIXEL_CHAR
                        } else {
                            SPACE_CHAR
                        }
                    })
                    .collect::<String>();

                line.push('\n');
                line
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn draw_sprite(&mut self, pixel_bitmasks: &[u8], offset_x: usize, offset_y: usize) -> bool {
        let mut pixels_overlapped = false;

        for (idx, &bitmask) in pixel_bitmasks.iter().enumerate() {
            // If pixel row is under the display, it will cut out.
            if idx + offset_y >= DISPLAY_HEIGHT {
                break;
            }

            // We will use signed int because it should cut out on the right side of display
            let bitmask_offset = DISPLAY_WIDTH as i8 - offset_x as i8 - BYTE_BITS as i8;
            let extended_bitmask: u64;

            if bitmask_offset > 0 {
                extended_bitmask = (bitmask as u64) << bitmask_offset;
            } else {
                extended_bitmask = (bitmask as u64) >> -bitmask_offset;
            }

            pixels_overlapped = extended_bitmask & self.pixels[idx + offset_y] > 0;

            self.pixels[idx + offset_y] ^= extended_bitmask;
        }

        pixels_overlapped
    }
}
