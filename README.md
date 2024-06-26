# Portable Pixmap based graphics library

Simple drawing to frame buffer and seeing the output!

It is increasingly becoming difficult to get a simple buffer and start drawing. I do somewhat
understand why this is so - we are not living in DOS times! But still the barrier to start working
with graphics should not be this hard.

This is not a graphics library or a GUI toolkit, this provides just a buffer where one can put
pixels then save the buffer to a PPM file and see the render.

Here is my attempt to provide a solution.

## Example:

```rust
use ppm::{Canvas, PPMFormats};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let mut c = Canvas::new(WIDTH, HEIGHT, PPMFormats::RAW);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if (x ^ y) % 7 == 0 {
                c.put_pixel(x, y, 0xFF0000);
            }
        }
    }
    c.save_to_file("example.ppm").expect("Could not save image");
}
```

![Example](/xor.png)

