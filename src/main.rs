mod color;
mod framebuffer;
mod bmp;
mod line_impl;
mod vertex;

use framebuffer::Framebuffer;
use line_impl::Line;
use crate::vertex::Vertex;
use nalgebra_glm::{Vec3};
use crate::color::Color;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.set_background_color(0x0000000);
    framebuffer.clear();

    // Usa Vec3 para definir los vértices
    let vertices = [
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0)
    ];

    // Definir colores
    let yellow = Color::new(255, 255, 0);
    let white = Color::new(255, 255, 255);

    framebuffer.fill_polygon(&vertices, yellow);
    framebuffer.draw_polygon(&vertices, white);
    framebuffer.render_buffer("filled_polygonPR.bmp").unwrap();
    
}
