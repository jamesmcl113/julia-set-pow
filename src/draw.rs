use image::{Rgb, RgbImage};

use crate::maths::Complex;

fn draw() {
    let width = 8000;
    let height = 6000;

    let mut image = RgbImage::new(width, height);

    let c = Complex {
        x: -0.9,
        y: 0.27015,
    };
    let num_iterations = 110;

    for x in 0..width {
        for y in 0..height {
            let inner_width = width as f32;
            let inner_height = height as f32;

            let mut z = Complex {
                x: 3.0 * (x as f32 - 0.5 * inner_width) / (inner_width),
                y: 2.0 * (y as f32 - 0.5 * inner_height) / (inner_height),
            };

            let mut i = num_iterations;

            while z.x * z.x + z.y * z.y < 4. && i > 1 {
                let tmp = z.x * z.x - z.y * z.y + c.x;
                z.y = 2.0 * z.x * z.y + c.y;
                z.x = tmp;
                i -= 1;
            }

            let r = (i << 3) as u8;
            let g = (i << 5) as u8;
            let b = (i << 4) as u8;

            let pixel = Rgb([r, g, b]);
            image.put_pixel(x, y, pixel);
        }
    }

    match image.save("output.png") {
        Err(e) => println!("{}", e),
        _ => {}
    }
}
