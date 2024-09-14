use libpix::{Canvas, PPMFormats};

fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;
    let mut canvas = Canvas::new(WIDTH, HEIGHT, PPMFormats::RAW);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            canvas.put_pixel(x, y, 0xFF0000);
        }
    }

    canvas.save_to_file("./square.ppm").expect("Cannot save file");
}
