pub mod ppm {
    use std::io;
    use std::{
        fs::*,
        io::{BufWriter, Write},
    };

    /// Reference: https://en.wikipedia.org/wiki/Netpbm#Description
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

    impl Canvas {
        pub fn new(width: usize, height: usize, format: PPMFormats) -> Self {
            Canvas {
                width,
                height,
                format,
                area: vec![0; width * height],
            }
        }

        pub fn put_pixel(&mut self, x: usize, y: usize, col: u32) {
            let i = (y * self.width) + x;
            self.area[i] = col;
        }

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

        pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
            let mut buffer = Vec::new();
            let mut file = BufWriter::new(File::create(filename)?);
            self.save_ppm_data(&mut buffer)?;
            file.write(&buffer)?;
            file.flush()
        }

        pub fn fill(&mut self, color: u32) {
            self.area.fill(color);
        }
    }
}
