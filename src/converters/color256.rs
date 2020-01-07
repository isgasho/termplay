use image::{GenericImage, Pixel};
use std::{
    cmp::{min, max},
    io::{self, Write}
};

#[derive(Clone, Copy, Debug)]
pub struct Color256;

impl super::Converter for Color256 {
    fn display<W, I, P>(&self, fmt: &mut W, image: &I) -> io::Result<()>
        where W: Write,
              I: GenericImage<Pixel = P>,
              P: Pixel<Subpixel = u8>
    {
        for y in 0..image.height() {
            if y > 0 {
                write!(fmt, "\r\n")?;
            }
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y).to_rgb().0;
                write!(fmt, "\x1b[48;5;{}m ", lookup_color(pixel[0], pixel[1], pixel[2]))?;
            }
            write!(fmt, "\x1b[0m")?;
        }
        Ok(())
    }
}

pub fn lookup_color(r: u8, g: u8, b: u8) -> u8 {
    let mut min: (u8, u32) = (0, ::std::u32::MAX);

    for (i, &(r2, g2, b2)) in COLORS.iter().enumerate() {
        let diff: u32 = abs_sub(r, r2) as u32 + abs_sub(g, g2) as u32 + abs_sub(b, b2) as u32;
        if diff < min.1 {
            min = (i as u8, diff);
        }
    }

    min.0
}
fn abs_sub(x: u8, y: u8) -> u8 {
    max(x, y) - min(x, y)
}

const COLORS: &[(u8, u8, u8)] = &[
    (0,0,0),
    (128,0,0),
    (0,128,0),
    (128,128,0),
    (0,0,128),
    (128,0,128),
    (0,128,128),
    (192,192,192),
    (128,128,128),
    (255,0,0),
    (0,255,0),
    (255,255,0),
    (0,0,255),
    (255,0,255),
    (0,255,255),
    (255,255,255),
    (0,0,0),
    (0,0,95),
    (0,0,135),
    (0,0,175),
    (0,0,215),
    (0,0,255),
    (0,95,0),
    (0,95,95),
    (0,95,135),
    (0,95,175),
    (0,95,215),
    (0,95,255),
    (0,135,0),
    (0,135,95),
    (0,135,135),
    (0,135,175),
    (0,135,215),
    (0,135,255),
    (0,175,0),
    (0,175,95),
    (0,175,135),
    (0,175,175),
    (0,175,215),
    (0,175,255),
    (0,215,0),
    (0,215,95),
    (0,215,135),
    (0,215,175),
    (0,215,215),
    (0,215,255),
    (0,255,0),
    (0,255,95),
    (0,255,135),
    (0,255,175),
    (0,255,215),
    (0,255,255),
    (95,0,0),
    (95,0,95),
    (95,0,135),
    (95,0,175),
    (95,0,215),
    (95,0,255),
    (95,95,0),
    (95,95,95),
    (95,95,135),
    (95,95,175),
    (95,95,215),
    (95,95,255),
    (95,135,0),
    (95,135,95),
    (95,135,135),
    (95,135,175),
    (95,135,215),
    (95,135,255),
    (95,175,0),
    (95,175,95),
    (95,175,135),
    (95,175,175),
    (95,175,215),
    (95,175,255),
    (95,215,0),
    (95,215,95),
    (95,215,135),
    (95,215,175),
    (95,215,215),
    (95,215,255),
    (95,255,0),
    (95,255,95),
    (95,255,135),
    (95,255,175),
    (95,255,215),
    (95,255,255),
    (135,0,0),
    (135,0,95),
    (135,0,135),
    (135,0,175),
    (135,0,215),
    (135,0,255),
    (135,95,0),
    (135,95,95),
    (135,95,135),
    (135,95,175),
    (135,95,215),
    (135,95,255),
    (135,135,0),
    (135,135,95),
    (135,135,135),
    (135,135,175),
    (135,135,215),
    (135,135,255),
    (135,175,0),
    (135,175,95),
    (135,175,135),
    (135,175,175),
    (135,175,215),
    (135,175,255),
    (135,215,0),
    (135,215,95),
    (135,215,135),
    (135,215,175),
    (135,215,215),
    (135,215,255),
    (135,255,0),
    (135,255,95),
    (135,255,135),
    (135,255,175),
    (135,255,215),
    (135,255,255),
    (175,0,0),
    (175,0,95),
    (175,0,135),
    (175,0,175),
    (175,0,215),
    (175,0,255),
    (175,95,0),
    (175,95,95),
    (175,95,135),
    (175,95,175),
    (175,95,215),
    (175,95,255),
    (175,135,0),
    (175,135,95),
    (175,135,135),
    (175,135,175),
    (175,135,215),
    (175,135,255),
    (175,175,0),
    (175,175,95),
    (175,175,135),
    (175,175,175),
    (175,175,215),
    (175,175,255),
    (175,215,0),
    (175,215,95),
    (175,215,135),
    (175,215,175),
    (175,215,215),
    (175,215,255),
    (175,255,0),
    (175,255,95),
    (175,255,135),
    (175,255,175),
    (175,255,215),
    (175,255,255),
    (215,0,0),
    (215,0,95),
    (215,0,135),
    (215,0,175),
    (215,0,215),
    (215,0,255),
    (215,95,0),
    (215,95,95),
    (215,95,135),
    (215,95,175),
    (215,95,215),
    (215,95,255),
    (215,135,0),
    (215,135,95),
    (215,135,135),
    (215,135,175),
    (215,135,215),
    (215,135,255),
    (215,175,0),
    (215,175,95),
    (215,175,135),
    (215,175,175),
    (215,175,215),
    (215,175,255),
    (215,215,0),
    (215,215,95),
    (215,215,135),
    (215,215,175),
    (215,215,215),
    (215,215,255),
    (215,255,0),
    (215,255,95),
    (215,255,135),
    (215,255,175),
    (215,255,215),
    (215,255,255),
    (255,0,0),
    (255,0,95),
    (255,0,135),
    (255,0,175),
    (255,0,215),
    (255,0,255),
    (255,95,0),
    (255,95,95),
    (255,95,135),
    (255,95,175),
    (255,95,215),
    (255,95,255),
    (255,135,0),
    (255,135,95),
    (255,135,135),
    (255,135,175),
    (255,135,215),
    (255,135,255),
    (255,175,0),
    (255,175,95),
    (255,175,135),
    (255,175,175),
    (255,175,215),
    (255,175,255),
    (255,215,0),
    (255,215,95),
    (255,215,135),
    (255,215,175),
    (255,215,215),
    (255,215,255),
    (255,255,0),
    (255,255,95),
    (255,255,135),
    (255,255,175),
    (255,255,215),
    (255,255,255),
    (8,8,8),
    (18,18,18),
    (28,28,28),
    (38,38,38),
    (48,48,48),
    (58,58,58),
    (68,68,68),
    (78,78,78),
    (88,88,88),
    (98,98,98),
    (108,108,108),
    (118,118,118),
    (128,128,128),
    (138,138,138),
    (148,148,148),
    (158,158,158),
    (168,168,168),
    (178,178,178),
    (188,188,188),
    (198,198,198),
    (208,208,208),
    (218,218,218),
    (228,228,228),
    (238,238,238)
];
