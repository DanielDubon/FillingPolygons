// line_impl.rs

use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;
use nalgebra_glm::Vec3;

pub trait Line {
    fn line(&mut self, start: Vec3, end: Vec3);
    fn draw_polygon(&mut self, points: &[Vec3]);
}


impl Line for Framebuffer {
    fn line(&mut self, start: Vec3, end: Vec3) {
        let mut x = start.x as isize;
        let mut y = start.y as isize;
        let x2 = end.x as isize;
        let y2 = end.y as isize;
        let dx = (x2 - x).abs();
        let dy = -(y2 - y).abs();
        let sx = if x < x2 { 1 } else { -1 };
        let sy = if y < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.point(x, y); // Correct call with isize
            if x == x2 && y == y2 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_polygon(&mut self, points: &[Vec3]) {
        if points.len() < 3 {
            return; // Un polígono requiere al menos 3 puntos
        }

        let mut last_point = &points[0]; // Comienza con el primer punto para cerrar el polígono al final

        for point in points.iter().skip(1) {
            self.line(*last_point, *point);
            last_point = point;
        }

        // Conectar el último punto con el primero para cerrar el polígono
        self.line(*last_point, points[0]);
    }
}






