use crate::Color;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        pixels.resize_with(pixels.capacity(), Default::default);
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn write_ppm_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        self.write_ppm(&mut file)?;
        Ok(())
    }

    pub fn write_ppm(&self, w: impl std::io::Write) -> std::io::Result<()> {
        let mut w = std::io::BufWriter::new(w);
        write!(w, "P3\n")?; // magic
        write!(w, "{} {}\n", self.width, self.height)?;
        write!(w, "255\n")?; // max color

        // convert a float to a clamped u8
        fn to_int(v: f64) -> u8 {
            if v < 0.0 {
                0
            } else if v >= 1.0 {
                255
            } else {
                (v * 256.0) as u8
            }
        }

        for y in 0..self.height {
            let offset = self.width * y;
            let row = self.pixels[offset..offset + self.width]
                .iter()
                .flat_map(|col| col.iter().map(to_int));
            let mut linelen = 0;
            let mut sep = "";
            for v in row {
                let vlen = match v {
                    0..=9 => 1,
                    10..=100 => 2,
                    _ => 3,
                };
                if linelen + sep.len() + vlen > 70 {
                    write!(w, "\n{}", v)?;
                    linelen = 0;
                } else {
                    write!(w, "{}{}", sep, v)?;
                    linelen += sep.len() + vlen;
                }
                sep = " ";
            }
            write!(w, "\n")?;
        }
        w.flush()?;
        Ok(())
    }
}

impl std::ops::Index<(usize, usize)> for Canvas {
    type Output = Color;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.pixels[idx.0 + idx.1 * self.width]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Canvas {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[idx.0 + idx.1 * self.width]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::Write;

    /// Creating a canvas
    #[test]
    fn creating() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(c[(x, y)], Color::default());
            }
        }
    }

    /// Writing pixels to a canvas
    #[test]
    fn writing() {
        let mut c = Canvas::new(10, 20);
        c[(2, 3)] = Color::new(1, 0, 0);
        assert_eq!(c[(2, 3)], Color::new(1, 0, 0));
    }

    /// Writing a PPM
    #[test]
    fn write_ppm() {
        let mut c = Canvas::new(5, 3);
        c[(0, 0)] = Color::new(1.5, 0, 0);
        c[(2, 1)] = Color::new(0, 0.5, 0);
        c[(4, 2)] = Color::new(-0.5, 0, 1);

        let mut ppm = Vec::new();
        c.write_ppm(&mut ppm).unwrap();

        let mut exp = Vec::new();
        write!(&mut exp, "P3\n").unwrap();
        write!(&mut exp, "5 3\n").unwrap();
        write!(&mut exp, "255\n").unwrap();
        write!(&mut exp, "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n").unwrap();
        write!(&mut exp, "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n").unwrap();
        write!(&mut exp, "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n").unwrap();

        assert_eq!(
            String::from_utf8(ppm).unwrap(),
            String::from_utf8(exp).unwrap()
        );
    }

    /// Writing a PPM with lines greater than 70 bytes
    #[test]
    fn write_wide_ppm() {
        let mut c = Canvas::new(10, 2);
        for x in 0..c.width() {
            for y in 0..c.height() {
                c[(x, y)] = Color::new(1, 0.8, 0.6);
            }
        }

        let mut ppm = Vec::new();
        c.write_ppm(&mut ppm).unwrap();

        let mut exp = Vec::new();
        write!(&mut exp, "P3\n").unwrap();
        write!(&mut exp, "10 2\n").unwrap();
        write!(&mut exp, "255\n").unwrap();
        write!(
            &mut exp,
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n"
        )
        .unwrap();
        write!(
            &mut exp,
            "153 255 204 153 255 204 153 255 204 153 255 204 153\n"
        )
        .unwrap();
        write!(
            &mut exp,
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n"
        )
        .unwrap();
        write!(
            &mut exp,
            "153 255 204 153 255 204 153 255 204 153 255 204 153\n"
        )
        .unwrap();

        assert_eq!(
            String::from_utf8(ppm).unwrap(),
            String::from_utf8(exp).unwrap()
        );
    }
}
