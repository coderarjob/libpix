//! Portable Pixmap Image format
//!
//! Supports both Plain and Raw formats of PPM image format. The idea of this library is to be
//! provide developers and students a 'frame buffer' to experiment with graphics, which for some
//! reason is very difficult to find.
//!
//! Do what you want with 'put_pixel'.
//!
//! See the below example and take a loop and the provided examples.
//!
//! # Example
//!
//! ```rust
//! use ppm::{Canvas, PPMFormats};
//!
//! fn main() {
//!     let mut c = Canvas::new(WIDTH, HEIGHT,PPMFormats::RAW);
//!     for x in 0..100 {
//!         for y in 0..100 {
//!             if (x ^ y) % 9 == 0 {
//!                 c.put_pixel(x, y, 0xFFFFFF);
//!             }
//!         }
//!     }
//!     c.save_to_file("example.ppm");
//! }
//! ```
//!
//! Reference: <https://en.wikipedia.org/wiki/Netpbm>

use std::io;
use std::{
    fs::*,
    io::{BufWriter, Write},
};

/// PPM formats
pub enum PPMFormats {
    PLAIN,
    RAW,
}

pub struct Canvas {
    width: usize,
    height: usize,
    format: PPMFormats,
    area: Vec<u32>,
}

/// Provides a buffer to draw pixels on
impl Canvas {
    /// Creates a new Canvas of `width` width and `height` size and `format` format.
    pub fn new(width: usize, height: usize, format: PPMFormats) -> Self {
        Canvas {
            width,
            height,
            format,
            area: vec![0; width * height],
        }
    }

    /// Puts a pixel of `col` color at a particular location
    pub fn put_pixel(&mut self, x: usize, y: usize, col: u32) {
        let i = (y * self.width) + x;
        self.area[i] = col;
    }

    /// Writes the Canvas pixels as PPM format to another buffer
    pub fn save_ppm_data<T: Write>(&self, buffer: &mut T) -> io::Result<()> {
        let magicno = match self.format {
            PPMFormats::RAW => "P6",
            PPMFormats::PLAIN => "P3",
        };
        write!(buffer, "{}", magicno)?;
        writeln!(buffer, "{} {} 255", self.width, self.height)?;
        for color in &self.area {
            let bytes = [
                ((color >> 8 * 2) & 0xFF) as u8,
                ((color >> 8 * 1) & 0xFF) as u8,
                ((color >> 8 * 0) & 0xFF) as u8,
            ];

            if let PPMFormats::PLAIN = self.format {
                for byte in bytes {
                    buffer.write((byte.to_string() + " ").as_bytes())?;
                }
            } else {
                buffer.write(&bytes)?;
            }
        }
        buffer.flush()
    }

    /// Writes the Canvas pixels as PPM format to a file
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut buffer = Vec::new();
        let mut file = BufWriter::new(File::create(filename)?);
        self.save_ppm_data(&mut buffer)?;
        file.write(&buffer)?;
        file.flush()
    }

    /// Fills the Canvas with 'color' color of pixels
    pub fn fill(&mut self, color: u32) {
        self.area.fill(color);
    }
}
