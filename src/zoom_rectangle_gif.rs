//! programme qui zoom sur un carré.
//! à adapter pour faire la partie 2/1 du TP

use gif::{Encoder, Frame, Repeat};
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    let mut image = File::create("zoom_carre.gif")?;
    let mut encoder = Encoder::new(&mut image, WIDTH as u16, WIDTH as u16, &[])?;

    encoder.set_repeat(Repeat::Infinite)?;

    let square = Square {
        x: 1.0,
        y: 1.0,
        side: 4.0,
    };

    let mut scale = 1f32;

    let mut frame_buffer = FrameBuffer::new();

    for _ in 0..20 {
        // le centre de notre zoom est le carré en question
        let x = square.x * scale;
        let y = square.y * scale;
        let side = square.side * scale;

        for i in 0..side as usize {
            for j in 0..side as usize {
                frame_buffer.draw_at(i + x as usize, j + y as usize, (255, 0, 0));
            }
        }

        let frame = Frame::from_rgb(WIDTH as u16, HEIGHT as u16, &frame_buffer.pixels);
        encoder.write_frame(&frame)?;

        // zoomer par 1.1
        scale *= 1.1;
        // reset le buffer
        frame_buffer.pixels = [0; WIDTH * HEIGHT * 3]
    }

    Ok(())
}

struct Square {
    x: f32,
    y: f32,
    side: f32,
}

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

/// Structure qui aide à gerer le mapping buffer de pixel <-> coordonnées x,y
struct FrameBuffer {
    pixels: [u8; WIDTH * HEIGHT * 3],
}

impl FrameBuffer {
    fn new() -> Self {
        Self {
            pixels: [0; WIDTH * HEIGHT * 3],
        }
    }

    /// fonction qui aide à dessiner sur le buffer comme attendu par
    /// [`gif::Frame`]
    fn draw_at(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let pos = (y * WIDTH + x) * 3;
        self.pixels[pos] = rgb.0;
        self.pixels[pos + 1] = rgb.1;
        self.pixels[pos + 2] = rgb.2;
    }
}
