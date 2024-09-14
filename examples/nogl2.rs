use libpix::{Canvas, PPMFormats, Vec2, Vec3};

const WIDTH: usize = 520;
const HEIGHT: usize = 520;

struct Point(usize, usize);
struct Rect(usize, usize);

fn length(l: &Vec2) -> f32 {
    (l.0.powi(2) + l.1.powi(2)).sqrt()
}

fn frag(uv: Vec2, aspect: f32) -> Vec3 {
    let mut uv = uv.mul(3.0).fract().sub(0.5).mul(2.0);
    uv.0 *= aspect;

    let mut d = length(&uv) - 0.5;
    d = (d * 8.0).sin() / 8.0;
    d = d.abs();
    d = 0.02 / d;

    let col = Vec3(0.10, 0.25, 0.56).mul(d);
    col
}

fn frag_draw(can: &mut Canvas, start: Point, size: Rect) {
    let Rect(width, height) = size;
    let Point(sx, sy) = start;
    let aspect = width as f32 / height as f32;

    for x in 0..width {
        for y in 0..height {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let Vec3(r, g, b) = frag(Vec2(u, v), aspect);
            let ri = (r * 255.0).clamp(0.0, 255.0) as u32;
            let gi = (g * 255.0).clamp(0.0, 255.0) as u32;
            let bi = (b * 255.0).clamp(0.0, 255.0) as u32;
            let color = (ri << 8 * 2) | (gi << 8 * 1) | bi;
            can.put_pixel(x + sx, y + sy, color);
        }
    }
}

fn main() {
    let mut c = Canvas::new(WIDTH, HEIGHT, PPMFormats::RAW);
    frag_draw(&mut c, Point(0, 0), Rect(WIDTH, HEIGHT));
    c.save_to_file("simple.ppm").unwrap();
}
