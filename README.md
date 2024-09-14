# libpix - A graphics library that's simple

The library provides a frame buffer and simple `put-pixel` and `save-to-file` routines. Thats all
you require to test and play around with graphics.

Simply draw to the frame buffer save to a file then see the output!

It is increasingly becoming difficult to get a simple buffer and start drawing. I do somewhat
understand why this is so - we are not living in DOS times! But still the barrier to start working
with graphics should not be this hard.

This GUI toolkit, this provides just a buffer where one can put pixels then save the buffer to a PPM
file and see the output.

### Why PPM and not any other file format?

Because its a simple image format ([Wikipedia - Netpbm](https://en.wikipedia.org/wiki/Netpbm)) and is supported in Linux.

## Example:

![Example](/xor.png)

```rust
use libpix::{Canvas, PPMFormats};

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

[Other examples](./examples)
