//partie 2 generation du gif
use gif::{Encoder, Frame, Repeat};
use std::{error::Error, fs::File};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

/// Structure qui aide à gérer le mapping buffer de pixel <-> coordonnées x,y
struct FrameBuffer {
    pixels: [u8; WIDTH * HEIGHT * 3],
}

impl FrameBuffer {
    fn new() -> Self {
        Self {
            pixels: [255; WIDTH * HEIGHT * 3],
        }
    }

    /// fonction qui aide à dessiner sur le buffer comme attendu par
    /// [`gif::Frame`]
    fn draw_at(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        let pos = (y * WIDTH + x) * 3;
        self.pixels[pos] = rgb.0;
        self.pixels[pos + 1] = rgb.1;
        self.pixels[pos + 2] = rgb.2;
    }
}

fn dessiner_cantor(buffer: &mut FrameBuffer, x: f32, y: f32, taille: f32, iterations: u32) {
    if iterations == 0 {
        // dessiner un carré
        for i in 0..taille as usize {
            for j in 0..taille as usize {
                buffer.draw_at(i + x as usize, j + y as usize, (28, 0, 28));
            }
        }
        return;
    }

    let nouvelle_taille = taille / 3.0;

    // Appels récursifs sur les 4 coins
    dessiner_cantor(buffer, x, y, nouvelle_taille, iterations - 1);
    dessiner_cantor(buffer, x + 2.0 * nouvelle_taille, y, nouvelle_taille, iterations - 1);
    dessiner_cantor(buffer, x, y + 2.0 * nouvelle_taille, nouvelle_taille, iterations - 1);
    dessiner_cantor(buffer, x + 2.0 * nouvelle_taille, y + 2.0 * nouvelle_taille, nouvelle_taille, iterations - 1);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut image = File::create("cantor1.gif")?;
    let mut encoder = Encoder::new(&mut image, WIDTH as u16, HEIGHT as u16, &[])?;

    encoder.set_repeat(Repeat::Infinite)?;


    let taille_base = 1200.0;
    let mut scale = 1f32;
    let mut iterations = 1;

    let mut frame_buffer = FrameBuffer::new();

    for frame_num in 0..20 {
        println!("Frame {}: iterations = {}, scale = {:.2}", frame_num, iterations, scale);

    
        let taille = taille_base * scale;
        dessiner_cantor(&mut frame_buffer, 0.0, 0.0, taille, iterations);

        let frame = Frame::from_rgb(WIDTH as u16, HEIGHT as u16, &frame_buffer.pixels);
        encoder.write_frame(&frame)?;

       
        scale *= 1.1;

        if frame_num % 2 == 1 {
            iterations += 1;
        }

        frame_buffer.pixels = [255; WIDTH * HEIGHT * 3];
    }

    Ok(())
}