use crate::bmp::Bitmap;
use crate::color::Color;
use nalgebra_glm::Vec3;
pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    const RED_COLOR: Color = Color { r: 255, g: 0, b: 0 };

    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::new(0, 0, 0);
        let buffer = vec![background_color; width * height];
        Self {
            width,
            height,
            buffer,
            background_color,
            current_color: Color::new(255, 255, 255),
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.current_color;
        }
    }

    pub fn get_point(&self, x: isize, y: isize) -> Option<Color> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, hex_color: u32) {
        self.background_color = Color::from_hex(hex_color);
    }

    pub fn set_current_color(&mut self, hex_color: u32) {
        self.current_color = Color::from_hex(hex_color);
    }

    pub fn render_buffer(&self, filename: &str) -> std::io::Result<()> {
        let mut bitmap = Bitmap::new(self.width as u32, self.height as u32);

        for y in 0..self.height {
            let inverted_y = (self.height - y - 1) as u32;

            for x in 0..self.width {
                let color = self.buffer[y * self.width + x];
                bitmap.set_pixel(x as u32, inverted_y, (color.r, color.g, color.b));
            }
        }

        bitmap.save(filename)
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, color: Color) {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return;
        }
        
        let index = (y as usize) * self.width + (x as usize);
        if index < self.buffer.len() {
            self.buffer[index] = color;
        }
    }

    pub fn fill_polygon(&mut self, vertices: &[Vec3], fill_color: Color) {
        let mut y_min = vertices[0].y as isize;
        let mut y_max = vertices[0].y as isize;

        for vertex in vertices.iter() {
            y_min = y_min.min(vertex.y as isize);
            y_max = y_max.max(vertex.y as isize);
        }

        for y in y_min..=y_max {
            let mut node_x: Vec<isize> = Vec::new();

            let mut j = vertices.len() - 1;
            for i in 0..vertices.len() {
                let vi = vertices[i];
                let vj = vertices[j];

                if (vi.y as isize <= y && vj.y as isize > y) || (vj.y as isize <= y && vi.y as isize > y) {
                    let x = vi.x + (y as f32 - vi.y) * (vj.x - vi.x) / (vj.y - vi.y);
                    node_x.push(x as isize);
                }
                j = i;
            }

            node_x.sort();

            for i in (0..node_x.len()).step_by(2) {
                if i + 1 < node_x.len() {
                    for x in node_x[i]..=node_x[i + 1] {
                        self.set_pixel(x, y, fill_color);
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_framebuffer() {
        let mut framebuffer = Framebuffer::new(10, 10);
        let background_color = Color::from_hex(0x000000);
        let foreground_color = Color::from_hex(0xFFFFFF);

        framebuffer.set_background_color(0x000000);
        framebuffer.clear();

        for pixel in framebuffer.buffer.iter() {
            assert_eq!(*pixel, background_color);
        }

        framebuffer.set_current_color(0xFFFFFF);
        framebuffer.point(5, 5);

        assert_eq!(framebuffer.get_point(5, 5).unwrap(), foreground_color);
        assert_eq!(framebuffer.get_point(-1, -1), None);
        assert_eq!(framebuffer.get_point(10, 10), None);
    }
}