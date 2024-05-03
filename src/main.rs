use std::{
    fs::*,
    io::{self, BufWriter, Write},
};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

struct Point(usize, usize);
struct Rect(usize, usize);

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

fn checkerd(buf: &mut [u32], start: Point, size: Rect, fgcol: u32, bgcol: u32, tilesize: usize) {
    let Rect(width, height) = size;
    let Point(sx, sy) = start;

    for x in 0..width {
        for y in 0..height {
            // This logic do not work, though created an interesting pattern.
            //   (x / tilesize) % 2 == 0 && (y / tilesize) % 2 == 0
            // May be the below works for checkerd pattern because Odd + Odd = Even number.
            buf[(y + sy) * WIDTH + (x + sx)] = if (y / tilesize + x / tilesize) % 2 == 0 {
                fgcol
            } else {
                bgcol
            };
        }
    }
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
            let ri = (r * 255.0).floor() as u32;
            let gi = (g * 255.0).floor() as u32;
            let bi = (b * 255.0).floor() as u32;
            let color = (ri << 8 * 2) | (gi << 8 * 1) | bi;
            buf[(y + sy) * WIDTH + (x + sx)] = color;
        }
    }
}

fn main() {
    let mut buf = [0xFF_FF_FFu32; WIDTH * HEIGHT];

    frag_draw(&mut buf, Point(0, 0), Rect(450, 450));

    solid_square(&mut buf, Point(0, 0), Rect(10, 10), 0xFF0000);
    solid_square(&mut buf, Point(105, 105), Rect(200, 300), 0x3F5F3F);
    solid_square(&mut buf, Point(100, 100), Rect(200, 300), 0x00FF00);
    solid_square(&mut buf, Point(100, 100), Rect(10, 10), 0xFF0000);

    checkerd(
        &mut buf,
        Point(400, 400),
        Rect(100, 100),
        0xFF0000,
        0x000000,
        10,
    );
    save_ppm("simple.ppm", &buf).unwrap();
}
