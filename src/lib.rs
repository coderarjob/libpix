
pub mod ppm {
    use std::io;
    use std::{
        fs::*,
        io::{BufWriter, Write},
    };

    pub struct Canvas {
        width: usize,
        height: usize,
        area: Vec<u32>
    }

    impl Canvas {
        pub fn new (width: usize, height: usize) -> Self{
            Canvas {
                width,
                height,
                area: vec![0; width * height]
            }
        }

        pub fn put_pixel(&mut self, x: usize, y: usize, col: u32) {
            let i = (y * self.width) + x;
            self.area[i] = col;
        }

        pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
            let mut file = BufWriter::new(File::create(filename)?);
            writeln!(file, "P6").unwrap();
            writeln!(file, "{} {} 255", self.width, self.height)?;
            for byte in &self.area {
                let colors = [
                    ((byte >> 8 * 2) & 0xFF) as u8,
                    ((byte >> 8 * 1) & 0xFF) as u8,
                    ((byte >> 8 * 0) & 0xFF) as u8,
                ];
                file.write(&colors)?;
            }
            file.flush()
        }
    }

}
