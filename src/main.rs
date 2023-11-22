use std::{
    fs::*,
    io::{BufWriter, Write, self},
};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

struct Point(usize, usize);
struct Rect(usize, usize);
struct Color(u8, u8, u8, u8);

/*fn bound_cast<T: From<i32>>(v: f32) -> T {
    (v.floor() as i32)
        .try_into()
        .unwrap_or_else(|_| panic!("Out of range: {v}"))
}

#[test]
fn bound_cast_success() {
    assert_eq!(bound_cast(25.0), 25);
    assert_eq!(bound_cast(0.0), 0);
    assert_eq!(bound_cast(255.0), 255);
}*/

fn save_ppm(filename: &str, buf: &[u32]) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(filename)?);
    writeln!(file, "P6").unwrap();
    writeln!(file, "{WIDTH} {HEIGHT} 255")?;
    for byte in buf {
        let colors = [
            ((byte >> 8 * 2) & 0xFF) as u8,
            ((byte >> 8 * 1) & 0xFF) as u8,
            ((byte >> 8 * 0) & 0xFF) as u8,
        ];
        file.write(&colors)?;
    }
    file.flush()
}

fn solid_square(buf: &mut [u32], start: Point, size: Rect, color: u32) {
    let Rect(width, height) = size;
    let Point(sx, sy) = start;

    for x in 0..width {
        for y in 0..height {
            buf[(y + sy) * WIDTH + (x + sx)] = color;
        }
    }
}

fn frag(u: f32, v: f32) -> (f32, f32, f32) {
    (u.sin(), v.sin(), 0.0)
}

fn frag_draw(buf: &mut [u32], start: Point, size: Rect) {
    let Rect(width, height) = size;
    let Point(sx, sy) = start;

    for x in 0..width {
        for y in 0..height {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let (r, g, b) = frag(u, v);
            let ri:u32 = ((r * 255.0).floor() as u32).try_into().unwrap();
            let gi:u32 = ((g * 255.0).floor() as u32).try_into().unwrap();
            let bi:u32 = ((b * 255.0).floor() as u32).try_into().unwrap();
            let color = ((ri << 8 * 2) | (gi << 8 * 1) | (bi << 8 * 0)) as u32;
            buf[(y + sy) * WIDTH + (x + sx)] = color;
        }
    }
}

fn main() {
    let mut buf = [0u32; WIDTH * HEIGHT];
    buf.fill(0xFFFFFF);

    frag_draw(&mut buf, Point(0, 0), Rect(450, 450));

    solid_square(&mut buf, Point(0, 0), Rect(10, 10), 0xFF0000);
    solid_square(&mut buf, Point(100, 100), Rect(200, 300), 0x00FF00);
    solid_square(&mut buf, Point(290, 390), Rect(10, 10), 0xFF0000);
    save_ppm("simple.ppm", &buf).unwrap();
}
