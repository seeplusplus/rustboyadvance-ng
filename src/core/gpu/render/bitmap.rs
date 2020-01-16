//! Rendering for modes 4-5

use super::super::consts::*;
use super::super::Gpu;
use super::super::Rgb15;

use super::{utils, SCREEN_VIEWPORT};

use crate::core::Bus;

impl Gpu {
    pub(in super::super) fn render_mode3(&mut self, bg: usize) {
        let y = self.vcount;

        let pa = self.bg_aff[bg - 2].pa as i32;
        let pc = self.bg_aff[bg - 2].pc as i32;
        let ref_point = self.get_ref_point(bg);

        for x in 0..DISPLAY_WIDTH {
            let t = utils::transform_bg_point(ref_point, x as i32, pa, pc);
            if !SCREEN_VIEWPORT.contains_point(t) {
                self.bg[bg].line[x] = Rgb15::TRANSPARENT;
                continue;
            }

            let pixel_index = index2d!(u32, t.0, t.1, DISPLAY_WIDTH);
            let pixel_ofs = 2 * pixel_index;
            let color = Rgb15(self.vram.read_16(pixel_ofs));
            self.bg[bg].line[x] = color;
        }
    }

    pub(in super::super) fn render_mode4(&mut self, bg: usize) {
        let page_ofs: u32 = match self.dispcnt.display_frame() {
            0 => 0x0600_0000 - VRAM_ADDR,
            1 => 0x0600_a000 - VRAM_ADDR,
            _ => unreachable!(),
        };

        let y = self.vcount;

        let pa = self.bg_aff[bg - 2].pa as i32;
        let pc = self.bg_aff[bg - 2].pc as i32;
        let ref_point = self.get_ref_point(bg);

        for x in 0..DISPLAY_WIDTH {
            let t = utils::transform_bg_point(ref_point, x as i32, pa, pc);
            if !SCREEN_VIEWPORT.contains_point(t) {
                self.bg[bg].line[x] = Rgb15::TRANSPARENT;
                continue;
            }
            let bitmap_index = index2d!(u32, t.0, t.1, DISPLAY_WIDTH);
            let bitmap_ofs = page_ofs + (bitmap_index as u32);
            let index = self.vram.read_8(bitmap_ofs) as u32;
            let color = self.get_palette_color(index, 0, 0);
            self.bg[bg].line[x] = color;
        }
    }
}
